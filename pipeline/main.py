"""Pipeline orchestrator for the reward hacking experiment.

Prototype pipeline:
1. Parse benchmark data
2. For each task, generate unit tests (Branch A) and Verus specs (Branch B)
3. For each task + branch, sample code completions
4. Score completions against reward signals
5. Report metrics
"""

import argparse
import json
import os
import sys
import time
import yaml

from data.parse_benchmarks import parse_human_eval_verus, HumanEvalVerusTask
from generation.generate_tests import generate_unit_tests
from generation.generate_specs import generate_verus_spec
from generation.generate_code import generate_code_for_tests, generate_code_for_verus
from evaluation.run_tests import run_rust_tests
from evaluation.run_verus import run_verus
from evaluation.evaluate import (
    CompletionScore,
    TaskEvaluation,
    compute_metrics,
    print_metrics,
    save_results,
    score_test_completion,
    score_verus_completion,
)


def load_config(config_path: str) -> dict:
    with open(config_path, 'r') as f:
        return yaml.safe_load(f)


def _print_block(label: str, content: str, verbose: bool):
    """Print a labeled block of content when verbose is enabled."""
    if not verbose or not content:
        return
    print(f"\n  ┌── {label} ──")
    for line in content.split("\n"):
        print(f"  │ {line}")
    print(f"  └{'─' * (len(label) + 6)}")


def run_pipeline(config: dict, verbose: bool = False):
    """Run the full prototype pipeline."""
    run_log = {"tasks": [], "config": config, "timestamp": time.strftime("%Y-%m-%d %H:%M:%S")}

    # --- Step 1: Parse benchmark ---
    print("=" * 60)
    print("STEP 1: Parsing benchmark data")
    print("=" * 60)

    tasks_dir = config["benchmark"]["human_eval_verus_dir"]
    max_problems = config["benchmark"]["max_problems"]

    tasks = parse_human_eval_verus(tasks_dir, only_with_verus=True)
    tasks = tasks[:max_problems]

    print(f"Using {len(tasks)} tasks with Verus specs")
    for t in tasks:
        print(f"  {t.task_id}: {t.entry_point}")
    print()

    model = config["openrouter"]["model"]
    generator_model = config["openrouter"].get("generator_model", model)
    n_samples = config["sampling"]["n_samples"]
    temperature = config["sampling"]["temperature"]
    verus_binary = config["evaluation"]["verus_binary"]

    # Validate API key early so we don't silently fail on every task
    import os
    from dotenv import load_dotenv
    load_dotenv()
    if not os.environ.get("OPENROUTER_API_KEY"):
        print("ERROR: OPENROUTER_API_KEY not set.")
        print("  export OPENROUTER_API_KEY='sk-or-...'")
        print("  or create a .env file with OPENROUTER_API_KEY=sk-or-...")
        sys.exit(1)

    print(f"Model: {model}")
    print(f"Generator model: {generator_model}")
    print(f"Samples per problem: {n_samples}")
    print(f"Temperature: {temperature}")
    if verbose:
        print("Verbose mode: ON (showing prompts, generated content, and scores)")
    print()

    evaluations = []

    for task_idx, task in enumerate(tasks):
        print("=" * 60)
        print(f"TASK {task_idx + 1}/{len(tasks)}: {task.task_id} ({task.entry_point})")
        print("=" * 60)

        nl_prompt = task.nl_prompt
        _print_block("NL Prompt", nl_prompt, verbose)

        task_log = {
            "task_id": task.task_id,
            "entry_point": task.entry_point,
            "nl_prompt": nl_prompt,
        }

        # --- Step 2: Generate reward mechanisms ---
        print("\n--- Step 2a: Generating unit tests (Branch A) ---")
        try:
            generated_tests = generate_unit_tests(
                nl_prompt=nl_prompt,
                entry_point=task.entry_point,
                model=generator_model,
            )
            print(f"  Generated tests ({len(generated_tests)} chars)")
            _print_block("Generated Unit Tests", generated_tests, verbose)
        except Exception as e:
            print(f"  ERROR generating tests: {type(e).__name__}: {e}")
            generated_tests = ""
        task_log["generated_tests"] = generated_tests

        print("\n--- Step 2b: Generating Verus spec (Branch B) ---")
        try:
            generated_spec = generate_verus_spec(
                nl_prompt=nl_prompt,
                entry_point=task.entry_point,
                model=generator_model,
            )
            print(f"  Generated spec ({len(generated_spec)} chars)")
            _print_block("Generated Verus Spec", generated_spec, verbose)
        except Exception as e:
            print(f"  ERROR generating spec: {type(e).__name__}: {e}")
            generated_spec = ""
        task_log["generated_spec"] = generated_spec

        # --- Step 3: Sample code completions ---
        print(f"\n--- Step 3a: Sampling {n_samples} code completions for Branch A ---")
        test_completions = []
        if generated_tests:
            try:
                test_completions = generate_code_for_tests(
                    nl_prompt=nl_prompt,
                    entry_point=task.entry_point,
                    tests=generated_tests,
                    model=model,
                    temperature=temperature,
                    n=n_samples,
                )
                print(f"  Generated {len(test_completions)} completions")
                for i, code in enumerate(test_completions):
                    _print_block(f"Branch A Completion {i}", code, verbose)
            except Exception as e:
                print(f"  ERROR generating code (Branch A): {type(e).__name__}: {e}")
        task_log["test_completions"] = test_completions

        print(f"\n--- Step 3b: Sampling {n_samples} code completions for Branch B ---")
        verus_completions = []
        if generated_spec:
            try:
                verus_completions = generate_code_for_verus(
                    nl_prompt=nl_prompt,
                    entry_point=task.entry_point,
                    spec=generated_spec,
                    model=model,
                    temperature=temperature,
                    n=n_samples,
                )
                print(f"  Generated {len(verus_completions)} completions")
                for i, code in enumerate(verus_completions):
                    _print_block(f"Branch B Completion {i}", code, verbose)
            except Exception as e:
                print(f"  ERROR generating code (Branch B): {type(e).__name__}: {e}")
        task_log["verus_completions"] = verus_completions

        # --- Step 4: Score completions ---
        print("\n--- Step 4: Scoring completions ---")

        test_scores = []
        task_log["test_scores"] = []
        for i, code in enumerate(test_completions):
            # Run tests directly so we can capture full output
            test_result = run_rust_tests(code, generated_tests)
            score = score_test_completion(
                task_id=task.task_id,
                code=code,
                tests=generated_tests,
                gold_verus_code=task.verus_code if task.has_verus_impl else None,
                completion_idx=i,
                verus_binary=verus_binary,
            )
            status = "PASS" if score.passes_own_reward else "FAIL"
            print(f"  Branch A completion {i}: {status} ({score.own_reward_detail})")
            if verbose:
                if not test_result.compile_success:
                    _print_block(f"Branch A [{i}] Compile Error", test_result.stderr, verbose)
                else:
                    _print_block(f"Branch A [{i}] Test Output", test_result.stdout, verbose)
                    if test_result.stderr.strip():
                        _print_block(f"Branch A [{i}] Stderr", test_result.stderr, verbose)
            test_scores.append(score)
            task_log["test_scores"].append({
                "idx": i,
                "passes": score.passes_own_reward,
                "detail": score.own_reward_detail,
                "compile_success": test_result.compile_success,
                "stdout": test_result.stdout,
                "stderr": test_result.stderr,
                "n_tests_passed": test_result.n_tests_passed,
                "n_tests_total": test_result.n_tests_total,
            })

        verus_scores = []
        task_log["verus_scores"] = []
        for i, code in enumerate(verus_completions):
            # Run verus directly so we can capture full output
            verus_result = run_verus(code, verus_binary=verus_binary)
            score = score_verus_completion(
                task_id=task.task_id,
                code=code,
                gold_verus_code=task.verus_code if task.has_verus_impl else None,
                completion_idx=i,
                verus_binary=verus_binary,
            )
            status = "PASS" if score.passes_own_reward else "FAIL"
            print(f"  Branch B completion {i}: {status} ({score.own_reward_detail})")
            if verbose:
                verus_output = (verus_result.stdout + "\n" + verus_result.stderr).strip()
                _print_block(f"Branch B [{i}] Verus Output", verus_output, verbose)
            verus_scores.append(score)
            task_log["verus_scores"].append({
                "idx": i,
                "passes": score.passes_own_reward,
                "detail": score.own_reward_detail,
                "verified": verus_result.verified,
                "n_verified": verus_result.n_verified,
                "n_errors": verus_result.n_errors,
                "stdout": verus_result.stdout,
                "stderr": verus_result.stderr,
            })

        evaluations.append(TaskEvaluation(
            task_id=task.task_id,
            entry_point=task.entry_point,
            unit_test_completions=test_scores,
            verus_spec_completions=verus_scores,
        ))
        run_log["tasks"].append(task_log)

    # --- Step 5: Compute and report metrics ---
    print("\n")
    metrics = compute_metrics(evaluations)
    print_metrics(metrics)

    # Save results
    output_dir = "results"
    save_results(evaluations, metrics, output_dir)

    # Save detailed run log
    os.makedirs(output_dir, exist_ok=True)
    log_path = os.path.join(output_dir, "run_log.json")
    with open(log_path, "w") as f:
        json.dump(run_log, f, indent=2)

    print(f"\nResults saved to {output_dir}/")
    print(f"Detailed run log: {log_path}")


def main():
    parser = argparse.ArgumentParser(description="Reward Hacking Experiment Pipeline")
    parser.add_argument(
        "--config",
        default="config/config.yaml",
        help="Path to config file",
    )
    parser.add_argument(
        "--verbose", "-v",
        action="store_true",
        help="Print full prompts, generated content, and scores",
    )
    args = parser.parse_args()

    config = load_config(args.config)
    run_pipeline(config, verbose=args.verbose)


if __name__ == "__main__":
    main()
