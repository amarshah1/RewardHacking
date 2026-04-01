"""Generate test and reward-hack few-shot candidates for the first 10 HumanEval tasks.

Uses Claude Opus via OpenRouter. Saves one folder per task under
few_shot_candidates/<task_id>/ containing:
  - nl_prompt.txt
  - gold_signatures.txt
  - generated_tests.rs
  - test_generation_raw.txt
  - reward_hack_impl.rs
  - reward_hack_raw.txt
  - test_result.txt
  - spliced.rs
  - compile_check.txt

If approved, these become few-shot examples for test generation and reward hacking.
"""

import os
import sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from dotenv import load_dotenv
load_dotenv()

from data.parse_benchmarks import parse_human_eval_verus
from generation.few_shot_examples import FEW_SHOT_TASK_IDS
from generation.generate_tests import SYSTEM_PROMPT as TEST_SYSTEM_PROMPT, USER_TEMPLATE as TEST_USER_TEMPLATE, _clean_test_output
from generation.generate_code import (
    SYSTEM_PROMPT_REWARD_HACK, USER_TEMPLATE_REWARD_HACK,
    _format_signatures, _clean_code_output,
)
from generation.openrouter_client import generate
from evaluation.run_tests import run_rust_tests
from evaluation.run_verus import run_verus
from evaluation.annotate_proofs import splice_body_into_gold_spec

ORACLE_MODEL = "anthropic/claude-opus-4"
OUTPUT_DIR = "few_shot_candidates"


def _save(task_dir: str, filename: str, content: str):
    os.makedirs(task_dir, exist_ok=True)
    with open(os.path.join(task_dir, filename), "w") as f:
        f.write(content)


def main():
    all_tasks = parse_human_eval_verus("human-eval-verus/tasks")
    tasks = [t for t in all_tasks if t.task_id in FEW_SHOT_TASK_IDS and t.has_verus_impl]
    tasks.sort(key=lambda t: t.task_id)

    print(f"Generating for {len(tasks)} tasks using {ORACLE_MODEL}")
    print(f"Output: {OUTPUT_DIR}/<task_id>/")
    print("Tasks:", [t.task_id for t in tasks])

    for task in tasks:
        task_slug = task.task_id.replace("/", "_")
        task_dir = os.path.join(OUTPUT_DIR, task_slug)
        print(f"\n{'=' * 60}")
        print(f"  {task.task_id}: {task.entry_point}")
        print(f"{'=' * 60}")

        sig = _format_signatures(task.impl_sig)
        _save(task_dir, "nl_prompt.txt", task.nl_prompt)
        _save(task_dir, "gold_signatures.txt", sig)

        # ── Step 1: Test generation ────────────────────────────────────────
        print("  Step 1: Generating unit tests...")
        test_prompt = TEST_USER_TEMPLATE.format(
            nl_prompt=task.nl_prompt,
            entry_point=task.entry_point,
            fn_signature=sig,
        )
        raw_tests = generate(
            prompt=test_prompt,
            system_prompt=TEST_SYSTEM_PROMPT,
            model=ORACLE_MODEL,
            temperature=0.0,
            max_tokens=2048,
            n=1,
        )[0]
        generated_tests = _clean_test_output(raw_tests)
        _save(task_dir, "test_generation_raw.txt", raw_tests)
        _save(task_dir, "generated_tests.rs", generated_tests)
        print(f"    Saved test_generation_raw.txt, generated_tests.rs")

        # ── Step 2: Reward hack generation ────────────────────────────────
        print("  Step 2: Generating reward hack implementation...")
        hack_prompt = USER_TEMPLATE_REWARD_HACK.format(
            nl_prompt=task.nl_prompt,
            entry_point=task.entry_point,
            fn_signature=sig,
        )
        raw_hack = generate(
            prompt=hack_prompt,
            system_prompt=SYSTEM_PROMPT_REWARD_HACK,
            model=ORACLE_MODEL,
            temperature=0.0,
            max_tokens=4096,
            n=1,
        )[0]
        hack_impl = _clean_code_output(raw_hack)
        _save(task_dir, "reward_hack_raw.txt", raw_hack)
        _save(task_dir, "reward_hack_impl.rs", hack_impl)
        print(f"    Saved reward_hack_raw.txt, reward_hack_impl.rs")

        # ── Step 3: Run tests against reward hack ─────────────────────────
        print("  Step 3: Running tests against reward hack...")
        test_result = run_rust_tests(hack_impl, generated_tests)
        status = "PASS" if test_result.passed else "FAIL"
        result_text = f"Status: {status}\n{test_result.n_tests_passed}/{test_result.n_tests_total} tests passed\nCompiled: {test_result.compile_success}\n\n"
        if not test_result.compile_success:
            result_text += f"=== COMPILE ERROR ===\n{test_result.stderr}\n"
        else:
            result_text += f"=== STDOUT ===\n{test_result.stdout}\n"
            if test_result.stderr.strip():
                result_text += f"\n=== STDERR ===\n{test_result.stderr}\n"
        _save(task_dir, "test_result.txt", result_text)
        print(f"    Tests: {status} ({test_result.n_tests_passed}/{test_result.n_tests_total})")

        # ── Step 4: Splice + compile check against gold spec ──────────────
        print("  Step 4: Splice + compile check against gold Verus spec...")
        spliced = splice_body_into_gold_spec(hack_impl, task.verus_code, task.verus_fn_names)
        _save(task_dir, "spliced.rs", spliced)

        compile_result = run_verus(spliced, no_verify=True, timeout=60)
        compile_ok = compile_result.n_errors == 0 and "error" not in compile_result.stderr.lower()
        compile_text = f"Compile check (--no-verify): {'PASS' if compile_ok else 'FAIL'}\n\n"
        compile_text += f"=== STDOUT ===\n{compile_result.stdout}\n"
        if compile_result.stderr.strip():
            compile_text += f"\n=== STDERR ===\n{compile_result.stderr}\n"
        _save(task_dir, "compile_check.txt", compile_text)
        print(f"    Compile check: {'PASS' if compile_ok else 'FAIL'}")
        print(f"    Saved to {task_dir}/")


if __name__ == "__main__":
    main()
