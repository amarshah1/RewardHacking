"""Run one or more cached completions against one cached oracle test suite."""

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


def _completion_path_for_task(task_id: str, run_dir: str, branch: str, completion_index: int) -> Path:
    """Build the cached Branch A completion path for one task and completion index."""
    if task_id.startswith("HumanEval/"):
        suffix = task_id.split("/", 1)[1]
    else:
        suffix = str(int(task_id))
    return Path(run_dir) / "HumanEval" / suffix / f"branch_{branch}" / f"completion_{completion_index}.rs"


def main() -> None:
    """Load one or more completions and one oracle suite, then run the tests locally."""
    parser = argparse.ArgumentParser(description="Check cached completions against one cached oracle suite")
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
        nargs="+",
        default=[0],
        help="One or more completion indices to test (e.g. --completion-index 0 1 2 3)",
    )
    parser.add_argument(
        "--branch",
        default="a",
        help="Branch used with --task-id",
    )
    parser.add_argument(
        "--trial",
        default="b",
        help="Corresponds to how many times we've generated and tested oracle tests for the same experiment",
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
        oracle_path = _oracle_path_for_task(args.task_id, args.oracle_dir)

        if not oracle_path.exists():
            raise SystemExit(f"Oracle test file not found: {oracle_path}")

        oracle_tests = oracle_path.read_text()
        all_passed = True

        for idx in args.completion_index:
            completion_path = _completion_path_for_task(args.task_id, str(run_dir), args.branch, idx)
            if not completion_path.exists():
                print(f"[{idx}] Completion file not found: {completion_path}", file=sys.stderr)
                all_passed = False
                continue

            code = completion_path.read_text()
            result = run_rust_tests(code, oracle_tests, timeout=args.timeout)

            result_lines = [
                f"Completion: {completion_path}",
                f"Oracle tests: {oracle_path}",
                f"Passed: {result.passed}",
                f"Compile success: {result.compile_success}",
                f"Tests passed: {result.n_tests_passed}/{result.n_tests_total}",
            ]
            if result.stdout:
                result_lines += ["", "STDOUT:", result.stdout]
            if result.stderr:
                result_lines += ["", "STDERR:", result.stderr]

            result_path = completion_path.parent / f"oracle_result_{idx}{args.trial}.txt"
            result_path.write_text("\n".join(result_lines))

            print(f"\n--- Completion {idx} ---")
            for line in result_lines[:5]:
                print(line)
            print(f"Result written to: {result_path}")
            if result.stdout:
                print("\nSTDOUT:")
                print(result.stdout)
            if result.stderr:
                print("\nSTDERR:")
                print(result.stderr)

            if not result.passed:
                all_passed = False

        raise SystemExit(0 if all_passed else 1)

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
