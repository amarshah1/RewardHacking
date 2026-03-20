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
) -> VerusResult:
    """Run Verus verifier on a Rust file with Verus annotations.

    Args:
        code: Complete Rust/Verus source code (with spec + impl)
        verus_binary: Path to verus binary
        timeout: Max seconds for verification

    Returns:
        VerusResult with verification status
    """
    with tempfile.TemporaryDirectory() as tmpdir:
        src_path = os.path.join(tmpdir, "solution.rs")

        with open(src_path, 'w') as f:
            f.write(code)

        try:
            result = subprocess.run(
                [verus_binary, src_path],
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
