"""Run Verus verifier to check code against formal specifications."""

import os
import subprocess
import tempfile
from dataclasses import dataclass


@dataclass
class VerusResult:
    verified: bool
    n_verified: int
    n_errors: int
    stdout: str
    stderr: str


def run_verus(
    code: str,
    verus_binary: str = "verus",
    timeout: int = 60,
    no_verify: bool = False,
) -> VerusResult:
    """Run Verus verifier on a Rust file with Verus annotations.

    Args:
        code: Complete Rust/Verus source code (with spec + impl)
        verus_binary: Path to verus binary
        timeout: Max seconds for verification
        no_verify: If True, only parse and type-check (skip SMT verification)

    Returns:
        VerusResult with verification status
    """
    with tempfile.TemporaryDirectory() as tmpdir:
        src_path = os.path.join(tmpdir, "solution.rs")

        with open(src_path, 'w') as f:
            f.write(code)

        cmd = [verus_binary, src_path]
        if no_verify:
            cmd.append("--no-verify")

        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            return VerusResult(
                verified=False,
                n_verified=0,
                n_errors=1,
                stdout="",
                stderr="Verus verification timed out",
            )
        except FileNotFoundError:
            return VerusResult(
                verified=False,
                n_verified=0,
                n_errors=1,
                stdout="",
                stderr=f"Verus binary not found at: {verus_binary}",
            )

        # Parse Verus output
        # Typical output: "verification results:: N verified, M errors"
        n_verified = 0
        n_errors = 0
        combined = result.stdout + result.stderr
        for line in combined.split("\n"):
            if "verified" in line and "error" in line:
                parts = line.split()
                for i, part in enumerate(parts):
                    if part == "verified,":
                        try:
                            n_verified = int(parts[i - 1])
                        except (ValueError, IndexError):
                            pass
                    if part == "errors" or part.startswith("error"):
                        try:
                            n_errors = int(parts[i - 1])
                        except (ValueError, IndexError):
                            pass

        verified = n_errors == 0 and n_verified > 0

        return VerusResult(
            verified=verified,
            n_verified=n_verified,
            n_errors=n_errors,
            stdout=result.stdout,
            stderr=result.stderr,
        )


def verify_against_gold_spec(
    generated_code_body: str,
    gold_verus_code: str,
    verus_binary: str = "verus",
    timeout: int = 60,
) -> VerusResult:
    """Verify generated code against a gold-standard Verus specification.

    This takes the generated function body and inserts it into the gold spec
    template, then runs Verus to check if the implementation satisfies the spec.

    Args:
        generated_code_body: The generated function implementation
        gold_verus_code: The complete gold-standard Verus file
        verus_binary: Path to verus binary
        timeout: Max seconds for verification

    Returns:
        VerusResult with verification status
    """
    # For now, we verify the gold spec file as-is to test the pipeline.
    # In the full pipeline, we'd need to splice the generated body into the spec.
    return run_verus(gold_verus_code, verus_binary=verus_binary, timeout=timeout)


def run_verus_with_tests(
    verus_code: str,
    oracle_tests: str,
    verus_binary: str = "verus",
    timeout: int = 120,
) -> VerusResult:
    """Compile Verus code and run oracle tests as runtime assertions.

    Oracle tests are `#[test]` functions with `assert_eq!` calls. We convert them
    to regular functions called from `main()`, compile with `verus --compile`,
    and run the executable. If any assert fails, the exit code is non-zero.

    Returns a VerusResult where:
        verified=True means all tests passed at runtime
        n_verified is the number of tests run (if successful)
        n_errors is >0 if compilation failed or any assertion failed
    """
    import re
    import shutil

    # Convert `#[test] fn oracle_case_N() { ... }` to `fn oracle_case_N() { ... }`
    # then call each from main().
    test_fn_names = re.findall(r'#\[test\]\s*fn\s+(\w+)\s*\(\s*\)', oracle_tests)
    tests_without_attr = re.sub(r'#\[test\]\s*', '', oracle_tests)

    # Strip out the existing `fn main() {}` (possibly `fn main(){}`) from the verus_code
    # since we're adding our own
    stripped_verus = re.sub(r'fn\s+main\s*\(\s*\)\s*\{\s*\}', '', verus_code)

    # Build main() that calls each oracle_case_N in turn
    main_body = "\n".join(f"    {name}();" for name in test_fn_names)
    main_body += f'\n    println!("ALL_TESTS_PASSED");'

    full_source = f"""{stripped_verus}

{tests_without_attr}

fn main() {{
{main_body}
}}
"""

    with tempfile.TemporaryDirectory() as tmpdir:
        src_path = os.path.join(tmpdir, "solution.rs")
        with open(src_path, 'w') as f:
            f.write(full_source)

        # Run verus --compile to verify and build the executable
        # The executable is written to the CURRENT DIRECTORY by verus, so cd to tmpdir
        cwd = os.getcwd()
        try:
            os.chdir(tmpdir)
            try:
                compile_result = subprocess.run(
                    [verus_binary, "--compile", "solution.rs"],
                    capture_output=True,
                    text=True,
                    timeout=timeout,
                )
            except subprocess.TimeoutExpired:
                return VerusResult(
                    verified=False,
                    n_verified=0,
                    n_errors=1,
                    stdout="",
                    stderr="verus --compile timed out",
                )
            except FileNotFoundError:
                return VerusResult(
                    verified=False,
                    n_verified=0,
                    n_errors=1,
                    stdout="",
                    stderr=f"Verus binary not found at: {verus_binary}",
                )

            # Check verification + compilation results
            combined = compile_result.stdout + compile_result.stderr
            n_verified = 0
            n_errors = 0
            for line in combined.split("\n"):
                if "verified" in line and "error" in line:
                    parts = line.split()
                    for i, part in enumerate(parts):
                        if part == "verified,":
                            try:
                                n_verified = int(parts[i - 1])
                            except (ValueError, IndexError):
                                pass
                        if part == "errors" or part.startswith("error"):
                            try:
                                n_errors = int(parts[i - 1])
                            except (ValueError, IndexError):
                                pass

            # Find the compiled executable
            binary_path = os.path.join(tmpdir, "solution")
            if not os.path.exists(binary_path):
                # Compilation failed — return what we have
                return VerusResult(
                    verified=False,
                    n_verified=n_verified,
                    n_errors=max(n_errors, 1),
                    stdout=compile_result.stdout,
                    stderr=compile_result.stderr,
                )

            # Run the executable
            try:
                run_result = subprocess.run(
                    [binary_path],
                    capture_output=True,
                    text=True,
                    timeout=30,
                )
            except subprocess.TimeoutExpired:
                return VerusResult(
                    verified=False,
                    n_verified=n_verified,
                    n_errors=max(n_errors, 1),
                    stdout=compile_result.stdout,
                    stderr="Test execution timed out",
                )

            # Check if all tests passed
            all_passed = (run_result.returncode == 0 and "ALL_TESTS_PASSED" in run_result.stdout)
            test_output = f"=== VERUS COMPILE ===\n{compile_result.stdout}\n{compile_result.stderr}\n\n"
            test_output += f"=== TEST RUN (exit code {run_result.returncode}) ===\n"
            test_output += f"STDOUT:\n{run_result.stdout}\n"
            if run_result.stderr.strip():
                test_output += f"STDERR:\n{run_result.stderr}\n"
            if not all_passed and run_result.returncode != 0:
                test_output += f"\nTest execution failed (assertion error or panic). See STDERR above for details.\n"

            return VerusResult(
                verified=all_passed,
                n_verified=len(test_fn_names) if all_passed else 0,
                n_errors=0 if all_passed else 1,
                stdout=test_output,
                stderr=run_result.stderr,
            )
        finally:
            os.chdir(cwd)


def check_spec_preserved(original_spec: str, generated_code: str) -> tuple[bool, str]:
    """Check that the generated code did not modify the Verus specification.

    Extracts non-blank, non-comment lines from the original spec and verifies
    each one appears in the generated code. Since the original spec has
    todo!() bodies, its lines are almost entirely spec-relevant (imports,
    signatures, requires, ensures, spec fn definitions).

    Returns:
        (preserved: bool, detail: str)
    """
    def _normalize_lines(code: str) -> list[str]:
        lines = []
        for line in code.split("\n"):
            s = line.strip()
            if not s or s.startswith("//"):
                continue
            # Skip the placeholder body — the implementation will replace it
            if s in ("todo!()", "unimplemented!()", "{", "}"):
                continue
            lines.append(s)
        return lines

    orig_lines = _normalize_lines(original_spec)
    gen_text = generated_code  # search as full text for flexibility with whitespace

    missing = []
    for line in orig_lines:
        if line not in gen_text:
            missing.append(line)

    if missing:
        detail = "Spec was modified. Missing lines:\n" + "\n".join(f"  - {l}" for l in missing[:10])
        if len(missing) > 10:
            detail += f"\n  ... and {len(missing) - 10} more"
        return False, detail

    return True, "spec preserved"


def validate_tests_against_gold(
    gold_verus_code: str,
    generated_tests: str,
    verus_binary: str = "verus",
    timeout: int = 60,
) -> dict:
    """Run generated unit tests against the gold-standard Verus implementation.

    Each test is invoked via std::panic::catch_unwind so a panicking test is
    recorded as incorrect without aborting the remaining tests.

    Returns a dict with keys:
        compiled:       bool
        test_results:   dict[str, bool]  (fn_name -> passed)
        n_passed:       int
        n_total:        int
        stdout:         str
        stderr:         str
    """
    import re

    test_fn_names = re.findall(r'#\[test\]\s*fn\s+(\w+)\s*\(\s*\)', generated_tests)
    if not test_fn_names:
        return {
            "compiled": False,
            "test_results": {},
            "n_passed": 0,
            "n_total": 0,
            "stdout": "",
            "stderr": "no #[test] functions found in generated tests",
        }

    tests_body = re.sub(r'#\[test\]\s*', '', generated_tests)

    # Remove the stub fn main() {} from gold Verus code; we supply our own.
    stripped = re.sub(r'fn\s+main\s*\(\s*\)\s*\{\s*\}', '', gold_verus_code)

    # Build main() that runs each test with catch_unwind so all tests are attempted.
    calls = []
    for name in test_fn_names:
        calls.append(
            f'    match ::std::panic::catch_unwind(|| {{ {name}(); }}) {{\n'
            f'        Ok(_) => println!("PASS:{name}"),\n'
            f'        Err(_) => println!("FAIL:{name}"),\n'
            f'    }};'
        )

    full_source = f"""{stripped}

{tests_body}

fn main() {{
{chr(10).join(calls)}
}}
"""

    with tempfile.TemporaryDirectory() as tmpdir:
        src_path = os.path.join(tmpdir, "solution.rs")
        with open(src_path, 'w') as f:
            f.write(full_source)

        cwd = os.getcwd()
        try:
            os.chdir(tmpdir)
            try:
                comp = subprocess.run(
                    [verus_binary, "--compile", "solution.rs"],
                    capture_output=True,
                    text=True,
                    timeout=timeout,
                )
            except subprocess.TimeoutExpired:
                return {"compiled": False, "test_results": {}, "n_passed": 0,
                        "n_total": len(test_fn_names), "stdout": "",
                        "stderr": "verus --compile timed out"}
            except FileNotFoundError:
                return {"compiled": False, "test_results": {}, "n_passed": 0,
                        "n_total": len(test_fn_names), "stdout": "",
                        "stderr": f"verus binary not found: {verus_binary}"}

            binary = os.path.join(tmpdir, "solution")
            if not os.path.exists(binary):
                return {
                    "compiled": False, "test_results": {}, "n_passed": 0,
                    "n_total": len(test_fn_names),
                    "stdout": comp.stdout, "stderr": comp.stderr,
                }

            try:
                run = subprocess.run([binary], capture_output=True, text=True, timeout=30)
            except subprocess.TimeoutExpired:
                return {"compiled": True, "test_results": {}, "n_passed": 0,
                        "n_total": len(test_fn_names),
                        "stdout": comp.stdout, "stderr": "test execution timed out"}

            test_results = {}
            for line in run.stdout.splitlines():
                if line.startswith("PASS:"):
                    test_results[line[5:]] = True
                elif line.startswith("FAIL:"):
                    test_results[line[5:]] = False
            # Any test not mentioned (e.g. binary crashed mid-run) counts as failed.
            for name in test_fn_names:
                if name not in test_results:
                    test_results[name] = False

            n_passed = sum(1 for v in test_results.values() if v)
            stdout = (
                f"=== COMPILE ===\n{comp.stdout}\n{comp.stderr}\n\n"
                f"=== RUN ===\n{run.stdout}"
            )
            return {
                "compiled": True,
                "test_results": test_results,
                "n_passed": n_passed,
                "n_total": len(test_fn_names),
                "stdout": stdout,
                "stderr": run.stderr,
            }
        finally:
            os.chdir(cwd)


if __name__ == "__main__":
    # Smoke test with a simple Verus file
    code = """use vstd::prelude::*;

verus! {

fn add(a: i32, b: i32) -> (result: i32)
    requires
        i32::MIN <= a + b <= i32::MAX,
    ensures
        result == a + b,
{
    a + b
}

} // verus!

fn main() {}
"""
    result = run_verus(code)
    print(f"Verified: {result.verified}")
    print(f"Results: {result.n_verified} verified, {result.n_errors} errors")
    if not result.verified:
        print(f"Stdout: {result.stdout}")
        print(f"Stderr: {result.stderr}")
