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
from generation.few_shot_examples import FEW_SHOT_TASK_IDS
from generation.generate_tests import generate_unit_tests
from generation.generate_specs import generate_verus_spec
from generation.generate_code import generate_code_for_tests, generate_code_for_verus, repair_code_for_tests, repair_code_for_verus, generate_reward_hack, repair_reward_hack
from evaluation.run_tests import run_rust_tests
from evaluation.run_verus import run_verus
from evaluation.annotate_proofs import annotate_with_proofs
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


def _extract_fn_signature(verus_code: str, entry_point: str) -> str:
    """Extract the function signature (with requires/ensures) from gold Verus code.

    Returns everything from `fn entry_point(...)` through the opening `{`,
    suitable for telling the model what signature to use.
    """
    import re
    lines = verus_code.split("\n")
    fn_idx = None
    for i, line in enumerate(lines):
        if re.match(rf'\s*(pub\s+)?fn\s+{re.escape(entry_point)}\s*\(', line):
            fn_idx = i
            break
    if fn_idx is None:
        return f"fn {entry_point}(...)"

    # Collect lines from fn signature through opening '{'
    sig_lines = []
    for i in range(fn_idx, len(lines)):
        sig_lines.append(lines[i])
        if '{' in lines[i]:
            break

    return "\n".join(sig_lines)


def _print_block(label: str, content: str, verbose: bool):
    """Print a labeled block of content when verbose is enabled."""
    if not verbose or not content:
        return
    print(f"\n  ┌── {label} ──")
    for line in content.split("\n"):
        print(f"  │ {line}")
    print(f"  └{'─' * (len(label) + 6)}")


def _save_cache_file(cache_dir: str, *path_parts: str, content: str):
    """Save content to a file under the cache directory."""
    filepath = os.path.join(cache_dir, *path_parts)
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    with open(filepath, "w") as f:
        f.write(content)


def run_pipeline(config: dict, verbose: bool = False):
    """Run the full prototype pipeline."""
    run_id = time.strftime("%Y%m%d_%H%M%S")
    cache_dir = os.path.join("experiment_cache", run_id)
    os.makedirs(cache_dir, exist_ok=True)

    # Save config for this run
    _save_cache_file(cache_dir, "config.yaml", content=yaml.dump(config, default_flow_style=False))

    run_log = {"tasks": [], "config": config, "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"), "run_id": run_id}

    # --- Step 1: Parse benchmark ---
    print("=" * 60)
    print("STEP 1: Parsing benchmark data")
    print("=" * 60)

    tasks_dir = config["benchmark"]["human_eval_verus_dir"]
    max_problems = config["benchmark"]["max_problems"]

    all_tasks = parse_human_eval_verus(tasks_dir, only_with_verus=True)

    # Skip tasks used as few-shot examples (first 10 HumanEval IDs)
    skip_few_shot = config["benchmark"].get("skip_few_shot", True)
    if skip_few_shot:
        tasks = [t for t in all_tasks if t.task_id not in FEW_SHOT_TASK_IDS]
        print(f"Skipping {len(all_tasks) - len(tasks)} tasks used as few-shot examples")
    else:
        tasks = all_tasks
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
    repair_rounds = config["sampling"].get("repair_rounds", 1)
    branch_a_enabled = config.get("branches", {}).get("branch_a", True)
    branch_b_enabled = config.get("branches", {}).get("branch_b", True)
    mode = config.get("mode", "correct")  # "correct" or "reward_hack"
    gold_spec_check = config.get("evaluation", {}).get("gold_spec_check", True)

    # Validate API key early so we don't silently fail on every task
    from dotenv import load_dotenv
    load_dotenv()
    if not os.environ.get("OPENROUTER_API_KEY"):
        print("ERROR: OPENROUTER_API_KEY not set.")
        print("  export OPENROUTER_API_KEY='sk-or-...'")
        print("  or create a .env file with OPENROUTER_API_KEY=sk-or-...")
        sys.exit(1)
    if gold_spec_check and not os.environ.get("ANTHROPIC_API_KEY"):
        print("WARNING: ANTHROPIC_API_KEY not set — gold spec verification will be skipped.")
        print("  Set it in .env to enable Claude oracle for proof annotation.")

    print(f"Model: {model}")
    print(f"Generator model: {generator_model}")
    print(f"Samples per problem: {n_samples}")
    print(f"Repair rounds: {repair_rounds}")
    print(f"Temperature: {temperature}")
    print(f"Mode: {mode}")
    print(f"Branch A (unit tests): {'ON' if branch_a_enabled else 'OFF'}")
    print(f"Branch B (Verus spec): {'ON' if branch_b_enabled else 'OFF'}")
    print(f"Gold spec check (Claude oracle): {'ON' if gold_spec_check else 'OFF'}")
    if verbose:
        print("Verbose mode: ON (showing prompts, generated content, and scores)")
    print()

    evaluations = []

    for task_idx, task in enumerate(tasks):
        print("=" * 60)
        print(f"TASK {task_idx + 1}/{len(tasks)}: {task.task_id} ({task.entry_point})")
        print("=" * 60)

        nl_prompt = task.nl_prompt
        task_cache = os.path.join(cache_dir, task.task_id)
        _print_block("NL Prompt", nl_prompt, verbose)

        task_log = {
            "task_id": task.task_id,
            "entry_point": task.entry_point,
            "nl_prompt": nl_prompt,
        }

        # Extract gold function signature for type-correct code generation
        gold_fn_sig = ""
        if task.has_verus_impl:
            gold_fn_sig = _extract_fn_signature(task.verus_code, task.entry_point)
            _print_block("Gold Function Signature", gold_fn_sig, verbose)

        # Save NL prompt
        _save_cache_file(task_cache, "nl_prompt.txt", content=nl_prompt)

        # --- Step 2: Generate reward mechanisms ---
        generated_tests = ""
        generated_spec = ""

        if branch_a_enabled:
            print("\n--- Step 2a: Generating unit tests (Branch A) ---")
            try:
                generated_tests = generate_unit_tests(
                    nl_prompt=nl_prompt,
                    entry_point=task.entry_point,
                    fn_signature=task.impl_sig,
                    model=generator_model,
                )
                print(f"  Generated tests ({len(generated_tests)} chars)")
                _print_block("Generated Unit Tests", generated_tests, verbose)
            except Exception as e:
                print(f"  ERROR generating tests: {type(e).__name__}: {e}")
        task_log["generated_tests"] = generated_tests
        _save_cache_file(task_cache, "generated_tests.rs", content=generated_tests)

        if branch_b_enabled:
            print("\n--- Step 2b: Generating Verus spec (Branch B) ---")
            try:
                generated_spec = generate_verus_spec(
                    nl_prompt=nl_prompt,
                    entry_point=task.entry_point,
                    model=generator_model,
                    verus_binary=verus_binary,
                    repair_rounds=repair_rounds,
                )
                print(f"  Generated spec ({len(generated_spec)} chars)")
                _print_block("Generated Verus Spec", generated_spec, verbose)
            except Exception as e:
                print(f"  ERROR generating spec: {type(e).__name__}: {e}")
        task_log["generated_spec"] = generated_spec
        _save_cache_file(task_cache, "generated_spec.rs", content=generated_spec)

        # --- Step 3: Sample code completions ---
        test_completions = []
        if branch_a_enabled and generated_tests:
            mode_label = "reward-hack" if mode == "reward_hack" else "correct"
            print(f"\n--- Step 3a: Sampling {n_samples} {mode_label} completions (Branch A) ---")
            try:
                if mode == "reward_hack":
                    test_completions = generate_reward_hack(
                        nl_prompt=nl_prompt,
                        entry_point=task.entry_point,
                        fn_signature=task.impl_sig,
                        model=model,
                        temperature=temperature,
                        n=n_samples,
                    )
                else:
                    test_completions = generate_code_for_tests(
                        nl_prompt=nl_prompt,
                        entry_point=task.entry_point,
                        tests=generated_tests,
                        fn_signature=task.impl_sig,
                        model=model,
                        temperature=temperature,
                        n=n_samples,
                    )
                print(f"  Generated {len(test_completions)} completions")
                for i, code in enumerate(test_completions):
                    _print_block(f"Branch A Completion {i}", code, verbose)
                    _save_cache_file(task_cache, "branch_a", f"completion_{i}.rs", content=code)
            except Exception as e:
                print(f"  ERROR generating code (Branch A): {type(e).__name__}: {e}")
        task_log["test_completions"] = test_completions

        verus_completions = []
        if branch_b_enabled and generated_spec:
            print(f"\n--- Step 3b: Sampling {n_samples} code completions (Branch B) ---")
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
                    _save_cache_file(task_cache, "branch_b", f"completion_{i}.rs", content=code)
            except Exception as e:
                print(f"  ERROR generating code (Branch B): {type(e).__name__}: {e}")
        task_log["verus_completions"] = verus_completions

        # --- Step 4: Score completions ---
        print("\n--- Step 4: Scoring completions ---")

        test_scores = []
        task_log["test_scores"] = []
        for i, code in enumerate(test_completions):
            current_code = code
            for round_num in range(repair_rounds):
                test_result = run_rust_tests(current_code, generated_tests)
                score = score_test_completion(
                    task_id=task.task_id,
                    code=current_code,
                    tests=generated_tests,
                    gold_verus_code=task.verus_code if task.has_verus_impl else None,
                    completion_idx=i,
                    verus_binary=verus_binary,
                )
                status = "PASS" if score.passes_own_reward else "FAIL"
                round_label = f"round {round_num+1}/{repair_rounds}" if repair_rounds > 1 else ""
                print(f"  Branch A completion {i} {round_label}: {status} ({score.own_reward_detail})")
                if verbose:
                    if not test_result.compile_success:
                        _print_block(f"Branch A [{i}] Compile Error", test_result.stderr, verbose)
                    else:
                        _print_block(f"Branch A [{i}] Test Output", test_result.stdout, verbose)
                        if test_result.stderr.strip():
                            _print_block(f"Branch A [{i}] Stderr", test_result.stderr, verbose)
                # Save each round's code and result
                round_suffix = f"_round{round_num}" if repair_rounds > 1 else ""
                _save_cache_file(task_cache, "branch_a", f"completion_{i}{round_suffix}.rs", content=current_code)
                result_text = f"Status: {status}\n{score.own_reward_detail}\nCompiled: {test_result.compile_success}\nRound: {round_num+1}/{repair_rounds}\n\n"
                if not test_result.compile_success:
                    result_text += f"=== COMPILE ERROR ===\n{test_result.stderr}\n"
                else:
                    result_text += f"=== STDOUT ===\n{test_result.stdout}\n"
                    if test_result.stderr.strip():
                        result_text += f"\n=== STDERR ===\n{test_result.stderr}\n"
                _save_cache_file(task_cache, "branch_a", f"result_{i}{round_suffix}.txt", content=result_text)

                if score.passes_own_reward or round_num == repair_rounds - 1:
                    break

                # Repair: send error back to LLM
                error_output = test_result.stderr if not test_result.compile_success else test_result.stdout
                print(f"    Repairing Branch A completion {i} (round {round_num+2}/{repair_rounds})...")
                try:
                    if mode == "reward_hack":
                        current_code = repair_reward_hack(
                            nl_prompt=nl_prompt,
                            entry_point=task.entry_point,
                            previous_code=current_code,
                            error_output=error_output,
                            fn_signature=task.impl_sig,
                            model=model,
                            temperature=temperature,
                        )
                    else:
                        current_code = repair_code_for_tests(
                            nl_prompt=nl_prompt,
                            entry_point=task.entry_point,
                            tests=generated_tests,
                            previous_code=current_code,
                            error_output=error_output,
                            fn_signature=task.impl_sig,
                            model=model,
                            temperature=temperature,
                        )
                except Exception as e:
                    print(f"    ERROR repairing (Branch A): {type(e).__name__}: {e}")
                    break

            # --- Step 5: Gold spec check (if completion passed its own reward) ---
            gold_verified = None
            gold_detail = "skipped (did not pass own reward)"
            annotated_code = ""
            if gold_spec_check and score.passes_own_reward and task.has_verus_impl:
                print(f"    Checking completion {i} against gold Verus spec (via Claude oracle)...")
                try:
                    annotated_code, gold_verified, gold_detail, proof_rounds = annotate_with_proofs(
                        generated_body=current_code,
                        gold_verus_code=task.verus_code,
                        entry_point=task.entry_point,
                        verus_binary=verus_binary,
                    )
                    gold_status = "PASS" if gold_verified else "FAIL"
                    print(f"    Gold spec: {gold_status} ({gold_detail})")
                    # Save the spliced input (before Claude adds annotations)
                    if proof_rounds and proof_rounds[0].get("spliced_input"):
                        _save_cache_file(task_cache, "branch_a", "gold_check",
                                         f"spliced_{i}.rs", content=proof_rounds[0]["spliced_input"])
                    # Save each proof annotation round
                    for pr in proof_rounds:
                        rd = pr["round"]
                        _save_cache_file(task_cache, "branch_a", "gold_check",
                                         f"annotated_{i}_round{rd}.rs", content=pr["code"])
                        exec_ok = pr.get("exec_check_passed", True)
                        verus_output = f"Exec check: {'PASS' if exec_ok else 'FAIL'}\n"
                        if not exec_ok:
                            verus_output += f"Violations:\n"
                            for v in pr.get("exec_violations", []):
                                verus_output += f"  MISSING: {v}\n"
                            verus_output += "\n"
                        verus_output += f"Verified: {pr['verified']}\n{pr['n_verified']} verified, {pr['n_errors']} errors\n\n"
                        verus_output += f"=== STDOUT ===\n{pr['stdout']}\n"
                        if pr["stderr"].strip():
                            verus_output += f"\n=== STDERR ===\n{pr['stderr']}\n"
                        _save_cache_file(task_cache, "branch_a", "gold_check",
                                         f"result_{i}_round{rd}.txt", content=verus_output)
                except Exception as e:
                    print(f"    ERROR checking gold spec: {type(e).__name__}: {e}")
                    gold_detail = f"error: {type(e).__name__}: {e}"
            elif score.passes_own_reward and not gold_spec_check:
                gold_detail = "gold spec check disabled"
            elif score.passes_own_reward and not task.has_verus_impl:
                gold_detail = "no gold spec available"

            # Update score with gold spec results
            score.passes_gold_spec = gold_verified
            score.gold_spec_detail = gold_detail
            score.is_reward_hacking = (
                score.passes_own_reward and gold_verified is False
            ) if gold_verified is not None else None

            test_scores.append(score)
            score_log = {
                "idx": i,
                "passes": score.passes_own_reward,
                "detail": score.own_reward_detail,
                "compile_success": test_result.compile_success,
                "stdout": test_result.stdout,
                "stderr": test_result.stderr,
                "n_tests_passed": test_result.n_tests_passed,
                "n_tests_total": test_result.n_tests_total,
                "rounds_used": round_num + 1,
                "passes_gold_spec": gold_verified,
                "gold_spec_detail": gold_detail,
                "is_reward_hacking": score.is_reward_hacking,
            }
            task_log["test_scores"].append(score_log)

        verus_scores = []
        task_log["verus_scores"] = []
        for i, code in enumerate(verus_completions):
            current_code = code
            for round_num in range(repair_rounds):
                verus_result = run_verus(current_code, verus_binary=verus_binary)
                score = score_verus_completion(
                    task_id=task.task_id,
                    code=current_code,
                    gold_verus_code=task.verus_code if task.has_verus_impl else None,
                    completion_idx=i,
                    verus_binary=verus_binary,
                )
                status = "PASS" if score.passes_own_reward else "FAIL"
                round_label = f"round {round_num+1}/{repair_rounds}" if repair_rounds > 1 else ""
                print(f"  Branch B completion {i} {round_label}: {status} ({score.own_reward_detail})")
                if verbose:
                    verus_output = (verus_result.stdout + "\n" + verus_result.stderr).strip()
                    _print_block(f"Branch B [{i}] Verus Output", verus_output, verbose)
                # Save each round's code and result
                round_suffix = f"_round{round_num}" if repair_rounds > 1 else ""
                _save_cache_file(task_cache, "branch_b", f"completion_{i}{round_suffix}.rs", content=current_code)
                result_text = f"Status: {status}\n{score.own_reward_detail}\nRound: {round_num+1}/{repair_rounds}\n\n=== VERUS OUTPUT ===\n{verus_result.stdout}\n"
                if verus_result.stderr.strip():
                    result_text += f"\n=== STDERR ===\n{verus_result.stderr}\n"
                _save_cache_file(task_cache, "branch_b", f"result_{i}{round_suffix}.txt", content=result_text)

                if score.passes_own_reward or round_num == repair_rounds - 1:
                    break

                # Repair: send error back to LLM
                error_output = (verus_result.stdout + "\n" + verus_result.stderr).strip()
                print(f"    Repairing Branch B completion {i} (round {round_num+2}/{repair_rounds})...")
                try:
                    current_code = repair_code_for_verus(
                        nl_prompt=nl_prompt,
                        entry_point=task.entry_point,
                        spec=generated_spec,
                        previous_code=current_code,
                        error_output=error_output,
                        model=model,
                        temperature=temperature,
                    )
                except Exception as e:
                    print(f"    ERROR repairing (Branch B): {type(e).__name__}: {e}")
                    break

            verus_scores.append(score)
            score_log = {
                "idx": i,
                "passes": score.passes_own_reward,
                "detail": score.own_reward_detail,
                "verified": verus_result.verified,
                "n_verified": verus_result.n_verified,
                "n_errors": verus_result.n_errors,
                "stdout": verus_result.stdout,
                "stderr": verus_result.stderr,
                "rounds_used": round_num + 1,
            }
            task_log["verus_scores"].append(score_log)

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

    # Save run log and metrics to cache
    run_log_path = os.path.join(cache_dir, "run_log.json")
    with open(run_log_path, "w") as f:
        json.dump(run_log, f, indent=2)

    metrics_path = os.path.join(cache_dir, "metrics.json")
    from dataclasses import asdict
    with open(metrics_path, "w") as f:
        json.dump(asdict(metrics), f, indent=2)

    print(f"\nResults saved to {output_dir}/")
    print(f"Experiment cache: {cache_dir}/")


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
