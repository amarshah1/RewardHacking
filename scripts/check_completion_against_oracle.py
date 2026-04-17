"""Run one cached completion against one cached oracle test suite."""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from evaluation.run_tests import run_rust_tests


def _oracle_path_for_task(task_id: str, oracle_dir: str) -> Path:
    """Map `n` or `HumanEval/n` to the corresponding cached oracle `.rs` path."""
    if task_id.startswith("HumanEval/"):
        suffix = task_id.split("/", 1)[1]
    else:
        suffix = str(int(task_id))
    return Path(oracle_dir) / f"HumanEval_{suffix}.rs"


def _completion_path_for_task(task_id: str, run_dir: str, completion_index: int) -> Path:
    """Build the cached Branch A completion path for one task and completion index."""
    if task_id.startswith("HumanEval/"):
        suffix = task_id.split("/", 1)[1]
    else:
        suffix = str(int(task_id))
    return Path(run_dir) / "HumanEval" / suffix / "branch_a" / f"completion_{completion_index}.rs"


def main() -> None:
    """Load one completion and one oracle suite, then run the tests locally."""
    parser = argparse.ArgumentParser(description="Check one cached completion against one cached oracle suite")
    parser.add_argument("--completion", help="Path to a completion .rs file")
    parser.add_argument("--oracle-tests", help="Path to a cached oracle .rs test file")
    parser.add_argument("--task-id", help="Task id in the form `n` or `HumanEval/n`")
    parser.add_argument(
        "--experiment-id",
        help="Experiment cache folder name used with --task-id, for example `20260417_094744`",
    )
    parser.add_argument(
        "--experiment-cache-root",
        default="experiment_cache",
        help="Parent directory containing experiment cache runs",
    )
    parser.add_argument(
        "--oracle-dir",
        default="data/oracle_test_cache",
        help="Oracle cache directory used with --task-id",
    )
    parser.add_argument(
        "--completion-index",
        type=int,
        default=0,
        help="Completion index used with --task-id",
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=30,
        help="Compilation/execution timeout in seconds",
    )
    args = parser.parse_args()

    if args.task_id:
        if not args.experiment_id:
            raise SystemExit("Provide --experiment-id when using --task-id.")
        run_dir = Path(args.experiment_cache_root) / args.experiment_id
        completion_path = _completion_path_for_task(args.task_id, str(run_dir), args.completion_index)
        oracle_path = _oracle_path_for_task(args.task_id, args.oracle_dir)
    else:
        if not args.completion or not args.oracle_tests:
            raise SystemExit("Provide either --task-id, or both --completion and --oracle-tests.")
        completion_path = Path(args.completion)
        oracle_path = Path(args.oracle_tests)

    if not completion_path.exists():
        raise SystemExit(f"Completion file not found: {completion_path}")
    if not oracle_path.exists():
        raise SystemExit(f"Oracle test file not found: {oracle_path}")

    code = completion_path.read_text()
    tests = oracle_path.read_text()
    result = run_rust_tests(code, tests, timeout=args.timeout)

    print(f"Completion: {completion_path}")
    print(f"Oracle tests: {oracle_path}")
    print(f"Passed: {result.passed}")
    print(f"Compile success: {result.compile_success}")
    print(f"Tests passed: {result.n_tests_passed}/{result.n_tests_total}")
    if result.stdout:
        print("\nSTDOUT:")
        print(result.stdout)
    if result.stderr:
        print("\nSTDERR:")
        print(result.stderr)

    raise SystemExit(0 if result.passed else 1)


if __name__ == "__main__":
    main()
