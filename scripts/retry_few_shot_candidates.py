"""Retry failed few-shot candidates with a fresh prompt (no error feedback).

Re-runs the original reward hack prompt at temperature=0.3 to get a different
attempt. Saves to new folders with _2, _3, etc. suffixes.
"""

import os
import sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from dotenv import load_dotenv
load_dotenv()

from data.parse_benchmarks import parse_human_eval_verus
from generation.few_shot_examples import FEW_SHOT_TASK_IDS
from generation.generate_code import (
    SYSTEM_PROMPT_REWARD_HACK, USER_TEMPLATE_REWARD_HACK,
    _format_signatures, _clean_code_output,
)
from generation.openrouter_client import generate
from evaluation.run_tests import run_rust_tests
from evaluation.run_verus import run_verus
from evaluation.annotate_proofs import splice_body_into_gold_spec

ORACLE_MODEL = "anthropic/claude-opus-4"
BASE_DIR = "few_shot_candidates"

# Tasks that need retry and what attempt number
RETRY_TASKS = {
    "HumanEval/1": 4,
    "HumanEval/4": 4,
    "HumanEval/6": 4,
    "HumanEval/8": 4,
}


def _save(task_dir, filename, content):
    os.makedirs(task_dir, exist_ok=True)
    with open(os.path.join(task_dir, filename), "w") as f:
        f.write(content)


def _read(path):
    with open(path) as f:
        return f.read()


def main():
    all_tasks = parse_human_eval_verus("human-eval-verus/tasks")
    tasks = {t.task_id: t for t in all_tasks if t.task_id in RETRY_TASKS}

    for task_id, attempt in RETRY_TASKS.items():
        task = tasks[task_id]
        task_slug = task_id.replace("/", "_")

        # Previous attempt folder (to get tests from)
        prev_dir = os.path.join(BASE_DIR, task_slug)
        new_dir = os.path.join(BASE_DIR, f"{task_slug}_{attempt}")

        print(f"\n{'=' * 60}")
        print(f"  {task_id}: {task.entry_point} (attempt {attempt})")
        print(f"{'=' * 60}")

        sig = _format_signatures(task.impl_sig)
        generated_tests = _read(os.path.join(prev_dir, "generated_tests.rs"))

        # Fresh prompt (same as original, no error feedback)
        hack_prompt = USER_TEMPLATE_REWARD_HACK.format(
            nl_prompt=task.nl_prompt,
            entry_point=task.entry_point,
            fn_signature=sig,
        )

        print(f"  Generating reward hack (fresh prompt, temp=0.3)...")
        raw_hack = generate(
            prompt=hack_prompt,
            system_prompt=SYSTEM_PROMPT_REWARD_HACK,
            model=ORACLE_MODEL,
            temperature=0.3,
            max_tokens=4096,
            n=1,
        )[0]
        hack_impl = _clean_code_output(raw_hack)
        _save(new_dir, "reward_hack_raw.txt", raw_hack)
        _save(new_dir, "reward_hack_impl.rs", hack_impl)
        _save(new_dir, "generated_tests.rs", generated_tests)
        _save(new_dir, "nl_prompt.txt", task.nl_prompt)
        _save(new_dir, "gold_signatures.txt", sig)

        # Run tests
        print(f"  Running tests...")
        test_result = run_rust_tests(hack_impl, generated_tests)
        status = "PASS" if test_result.passed else "FAIL"
        result_text = f"Status: {status}\n{test_result.n_tests_passed}/{test_result.n_tests_total} tests passed\nCompiled: {test_result.compile_success}\n\n"
        if not test_result.compile_success:
            result_text += f"=== COMPILE ERROR ===\n{test_result.stderr}\n"
        else:
            result_text += f"=== STDOUT ===\n{test_result.stdout}\n"
            if test_result.stderr.strip():
                result_text += f"\n=== STDERR ===\n{test_result.stderr}\n"
        _save(new_dir, "test_result.txt", result_text)
        print(f"  Tests: {status} ({test_result.n_tests_passed}/{test_result.n_tests_total})")

        # Splice + compile check
        print(f"  Splice + compile check...")
        spliced = splice_body_into_gold_spec(hack_impl, task.verus_code, task.verus_fn_names)
        _save(new_dir, "spliced.rs", spliced)

        compile_result = run_verus(spliced, no_verify=True, timeout=60)
        compile_ok = compile_result.n_errors == 0 and "error" not in compile_result.stderr.lower()
        compile_text = f"Compile check (--no-verify): {'PASS' if compile_ok else 'FAIL'}\n\n"
        compile_text += f"=== STDOUT ===\n{compile_result.stdout}\n"
        if compile_result.stderr.strip():
            compile_text += f"\n=== STDERR ===\n{compile_result.stderr}\n"
        _save(new_dir, "compile_check.txt", compile_text)
        print(f"  Compile check: {'PASS' if compile_ok else 'FAIL'}")


if __name__ == "__main__":
    main()
