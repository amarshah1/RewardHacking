"""Validate LLM-generated unit tests by running them against the gold implementation.

Each #[test] function is called from main() using std::panic::catch_unwind.
Tests that panic are incorrect; tests that pass are correct.

Usage (by task id + experiment run):
    python scripts/validate_generated_tests.py \
        --task-id HumanEval/135 \
        --experiment-id 20260429_125414

Usage (with explicit file paths):
    python scripts/validate_generated_tests.py \
        --generated-tests path/to/generated_tests.rs \
        --gold-impl path/to/human_eval_135.rs
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from data.parse_benchmarks import parse_human_eval_verus
from evaluation.run_verus import validate_tests_against_gold


def _find_task(task_id: str, tasks_dir: str):
    """Return the HumanEvalVerusTask for task_id, or None if not found."""
    tasks = parse_human_eval_verus(tasks_dir, only_with_verus=True)
    for t in tasks:
        if t.task_id == task_id:
            return t
    return None


def _normalise_task_id(raw: str) -> str:
    """Accept 'HumanEval/135' or bare '135'."""
    if raw.startswith("HumanEval/"):
        return raw
    return f"HumanEval/{int(raw)}"


def _generated_tests_path(task_id: str, experiment_id: str, cache_root: str) -> Path:
    suffix = task_id.split("/", 1)[1]
    return Path(cache_root) / experiment_id / "HumanEval" / suffix / "generated_tests.rs"


def _print_results(validation: dict, generated_tests_path: str, gold_source: str) -> bool:
    print(f"Generated tests: {generated_tests_path}")
    print(f"Gold implementation: {gold_source}")
    print(f"Compiled: {validation['compiled']}")
    print(f"Tests correct: {validation['n_passed']}/{validation['n_total']}")
    if validation["compiled"]:
        for fn_name, passed in validation["test_results"].items():
            status = "PASS" if passed else "FAIL"
            print(f"  {status}: {fn_name}")
    if validation["stdout"]:
        print("\nSTDOUT:")
        print(validation["stdout"])
    if validation["stderr"]:
        print("\nSTDERR:")
        print(validation["stderr"])
    return validation["compiled"] and validation["n_passed"] == validation["n_total"]


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Validate LLM-generated tests against the gold Verus implementation"
    )
    parser.add_argument("--task-id", help="Task id in the form `n` or `HumanEval/n`")
    parser.add_argument(
        "--experiment-id",
        help="Experiment cache folder name (e.g. `20260429_125414`); required with --task-id",
    )
    parser.add_argument(
        "--experiment-cache-root",
        default="experiment_cache",
        help="Parent directory of experiment cache runs",
    )
    parser.add_argument(
        "--tasks-dir",
        default="human-eval-verus/tasks",
        help="Directory containing gold human-eval-verus task .rs files",
    )
    parser.add_argument("--generated-tests", help="Path to a generated_tests.rs file")
    parser.add_argument(
        "--gold-impl",
        help="Path to a human_eval_*.rs gold task file (the full file, not just the Verus section)",
    )
    parser.add_argument("--verus-binary", default="verus", help="Path to verus binary")
    parser.add_argument("--timeout", type=int, default=60, help="Compilation timeout in seconds")
    parser.add_argument(
        "--save",
        action="store_true",
        help="Write results to generated_tests_validation.txt next to the generated_tests.rs file",
    )
    args = parser.parse_args()

    if args.task_id:
        task_id = _normalise_task_id(args.task_id)
        if not args.experiment_id:
            raise SystemExit("--experiment-id is required when using --task-id")

        tests_path = _generated_tests_path(task_id, args.experiment_id, args.experiment_cache_root)
        if not tests_path.exists():
            raise SystemExit(f"Generated tests not found: {tests_path}")

        task = _find_task(task_id, args.tasks_dir)
        if task is None:
            raise SystemExit(f"Task {task_id} not found in {args.tasks_dir}")
        if not task.has_verus_impl:
            raise SystemExit(f"Task {task_id} has no gold Verus implementation")

        generated_tests = tests_path.read_text()
        gold_verus_code = task.verus_code
        gold_source = f"{args.tasks_dir} ({task_id})"

    elif args.generated_tests and args.gold_impl:
        tests_path = Path(args.generated_tests)
        gold_path = Path(args.gold_impl)
        if not tests_path.exists():
            raise SystemExit(f"Generated tests not found: {tests_path}")
        if not gold_path.exists():
            raise SystemExit(f"Gold implementation not found: {gold_path}")

        generated_tests = tests_path.read_text()

        # Parse the full task file to extract just the verus_code section.
        from data.parse_benchmarks import parse_task_file
        task = parse_task_file(str(gold_path))
        if task is None or not task.has_verus_impl:
            raise SystemExit(f"Could not extract a Verus implementation from {gold_path}")
        gold_verus_code = task.verus_code
        gold_source = str(gold_path)

    else:
        raise SystemExit(
            "Provide either --task-id + --experiment-id, "
            "or both --generated-tests and --gold-impl."
        )

    validation = validate_tests_against_gold(
        gold_verus_code=gold_verus_code,
        generated_tests=generated_tests,
        verus_binary=args.verus_binary,
        timeout=args.timeout,
    )

    all_correct = _print_results(validation, str(tests_path), gold_source)

    if args.save:
        lines = [
            f"Compiled: {validation['compiled']}",
            f"Tests correct: {validation['n_passed']}/{validation['n_total']}",
            "",
        ]
        if validation["compiled"]:
            for fn_name, passed in validation["test_results"].items():
                lines.append(f"{'PASS' if passed else 'FAIL'}: {fn_name}")
        lines += ["", "=== STDOUT ===", validation["stdout"]]
        if validation["stderr"]:
            lines += ["", "=== STDERR ===", validation["stderr"]]
        out_path = tests_path.parent / "generated_tests_validation.txt"
        out_path.write_text("\n".join(lines))
        print(f"\nResults written to: {out_path}")

    raise SystemExit(0 if all_correct else 1)


if __name__ == "__main__":
    main()
