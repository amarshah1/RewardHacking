"""Compile and run Rust unit tests to score code completions."""

import os
import subprocess
import tempfile
from dataclasses import dataclass


@dataclass
class TestResult:
    passed: bool
    compile_success: bool
    stdout: str
    stderr: str
    n_tests_passed: int
    n_tests_total: int


def build_test_file(code: str, tests: str) -> str:
    """Combine generated code and unit tests into a single Rust file."""
    return f"""{code}

#[cfg(test)]
mod tests {{
    use super::*;

{tests}
}}
"""


def run_rust_tests(
    code: str,
    tests: str,
    timeout: int = 30,
) -> TestResult:
    """Compile and run Rust unit tests on a code completion.

    Args:
        code: Rust code implementing the function
        tests: Rust #[test] functions
        timeout: Max seconds for compilation + execution

    Returns:
        TestResult with pass/fail and details
    """
    full_source = build_test_file(code, tests)

    with tempfile.TemporaryDirectory() as tmpdir:
        src_path = os.path.join(tmpdir, "solution.rs")
        bin_path = os.path.join(tmpdir, "solution_test")

        with open(src_path, 'w') as f:
            f.write(full_source)

        # Compile with test harness
        try:
            compile_result = subprocess.run(
                ["rustc", "--test", src_path, "-o", bin_path],
                capture_output=True,
                text=True,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            return TestResult(
                passed=False,
                compile_success=False,
                stdout="",
                stderr="Compilation timed out",
                n_tests_passed=0,
                n_tests_total=0,
            )

        if compile_result.returncode != 0:
            return TestResult(
                passed=False,
                compile_success=False,
                stdout=compile_result.stdout,
                stderr=compile_result.stderr,
                n_tests_passed=0,
                n_tests_total=0,
            )

        # Run tests
        try:
            run_result = subprocess.run(
                [bin_path],
                capture_output=True,
                text=True,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            return TestResult(
                passed=False,
                compile_success=True,
                stdout="",
                stderr="Test execution timed out",
                n_tests_passed=0,
                n_tests_total=0,
            )

        # Parse test results from stdout
        # Rust test output format: "test result: ok. X passed; Y failed; ..."
        n_passed = 0
        n_total = 0
        for line in run_result.stdout.split("\n"):
            if line.startswith("test result:"):
                parts = line.split()
                for i, part in enumerate(parts):
                    if part == "passed;":
                        n_passed = int(parts[i - 1])
                    if part == "failed;":
                        n_failed = int(parts[i - 1])
                        n_total = n_passed + n_failed

        all_passed = run_result.returncode == 0

        return TestResult(
            passed=all_passed,
            compile_success=True,
            stdout=run_result.stdout,
            stderr=run_result.stderr,
            n_tests_passed=n_passed,
            n_tests_total=n_total if n_total > 0 else n_passed,
        )


if __name__ == "__main__":
    # Smoke test with a simple example
    code = """
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"""
    tests = """
    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }
"""
    result = run_rust_tests(code, tests)
    print(f"Passed: {result.passed}")
    print(f"Compiled: {result.compile_success}")
    print(f"Tests: {result.n_tests_passed}/{result.n_tests_total}")
    if not result.passed:
        print(f"Stderr: {result.stderr}")
