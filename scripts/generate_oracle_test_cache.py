"""Precompute oracle input/output pairs for HumanEval-Verus tasks."""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from data.parse_benchmarks import parse_human_eval_verus
from generation.few_shot_examples import FEW_SHOT_TASK_IDS
from generation.oracle_tests import (
    OracleTestGenerationError,
    generate_oracle_test_cache,
    oracle_cache_filename,
    render_oracle_unit_tests,
    save_oracle_test_cache,
)
from pipeline.main import load_config


def main() -> None:
    """Generate and cache oracle IO pairs once so the pipeline can reuse them later."""
    parser = argparse.ArgumentParser(description="Generate cached oracle IO pairs")
    parser.add_argument("--config", default="config/config.yaml", help="Path to config file")
    parser.add_argument("--tasks-dir", help="Override benchmark task directory")
    parser.add_argument("--output-dir", help="Override oracle cache directory")
    parser.add_argument("--max-problems", type=int, help="Optional task limit")
    parser.add_argument("--task-id", help="Generate cache for only one task id")
    args = parser.parse_args()

    config = load_config(args.config)
    tasks_dir = args.tasks_dir or config["benchmark"]["human_eval_verus_dir"]
    output_dir = Path(args.output_dir or config["test_generation"]["oracle_cache_dir"])
    max_problems = args.max_problems
    skip_few_shot = config["benchmark"].get("skip_few_shot", True)
    oracle_cases = config["test_generation"].get("oracle_cases", 24)
    oracle_trials = config["test_generation"].get("oracle_max_trials", 256)
    oracle_seed = config["test_generation"].get("oracle_seed", 0)
    verus_binary = config["evaluation"].get("verus_binary", "verus")
    timeout = config["evaluation"].get("timeout_seconds", 30)

    tasks = parse_human_eval_verus(tasks_dir, only_with_verus=True)
    # if skip_few_shot and not args.task_id:
    #     tasks = [task for task in tasks if task.task_id not in FEW_SHOT_TASK_IDS]
    if args.task_id:
        tasks = [task for task in tasks if task.task_id == args.task_id]
    elif max_problems:
        tasks = tasks[:max_problems]

    output_dir.mkdir(parents=True, exist_ok=True)
    print(f"Generating oracle caches for {len(tasks)} tasks into {output_dir}")

    successes = 0
    failures = 0
    for task in tasks:
        # if task.has_verus_impl:
        cache_path = output_dir / oracle_cache_filename(task.task_id)
        tests_path = cache_path.with_suffix(".rs")
        print(f"  {task.task_id}: ", end="", flush=True)
        try:
            cache = generate_oracle_test_cache(
                task_id=task.task_id,
                verus_code=task.verus_code,
                verus_fn_names=task.verus_fn_names,
                impl_signatures=task.impl_sig,
                verus_binary=verus_binary,
                num_cases=oracle_cases,
                max_trials=oracle_trials,
                seed=oracle_seed,
                timeout=timeout,
            )
            save_oracle_test_cache(cache, str(cache_path))
            tests_path.write_text(render_oracle_unit_tests(cache))
            successes += 1
            print("ok")
        except OracleTestGenerationError as exc:
            failures += 1
            print(f"failed ({exc})")

    print(f"Done. {successes} succeeded, {failures} failed.")


if __name__ == "__main__":
    main()
