"""Parse human-eval-verus benchmark files into structured format."""

import json
import os
import re
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Optional


@dataclass
class HumanEvalVerusTask:
    task_id: str
    nl_prompt: str  # Natural language description (Python docstring)
    entry_point: str
    canonical_solution: str  # Python reference solution
    python_tests: str  # Python test cases
    verus_code: str  # Full Verus code (spec + impl + proof)
    has_verus_impl: bool  # Whether the Verus section has a real implementation


def _extract_section(content: str, section_name: str) -> str:
    """Extract content between ### SECTION_NAME markers inside block comments."""
    # Sections are wrapped in /* ... */ block comments
    # Pattern: find the block comment containing ### SECTION_NAME,
    # then get everything after it until the closing */
    pattern = rf'/\*\s*\n### {re.escape(section_name)}\s*\n(.*?)\*/'
    match = re.search(pattern, content, re.DOTALL)
    if match:
        return match.group(1).strip()
    return ""


def _extract_verus_code(content: str) -> str:
    """Extract the Verus code between VERUS BEGIN and VERUS END markers."""
    begin_pattern = r'/\*\s*\n### VERUS BEGIN\s*\n\*/'
    end_pattern = r'/\*\s*\n### VERUS END\s*\n\*/'

    begin_match = re.search(begin_pattern, content)
    end_match = re.search(end_pattern, content)

    if begin_match and end_match:
        return content[begin_match.end():end_match.start()].strip()
    return ""


def _has_real_verus_impl(verus_code: str) -> bool:
    """Check if the Verus code has a real implementation (not just a TODO)."""
    # Strip boilerplate
    stripped = verus_code.strip()
    if not stripped:
        return False
    # Check if it's just the empty template
    if "TODO" in stripped and stripped.count("\n") < 10:
        return False
    # Check if there's an actual function definition (not just spec)
    # A real implementation should have `fn ` (exec function) with `ensures` or meaningful body
    has_fn = bool(re.search(r'\bfn\s+\w+', stripped))
    has_ensures = 'ensures' in stripped or 'requires' in stripped
    return has_fn and has_ensures


def parse_task_file(filepath: str) -> Optional[HumanEvalVerusTask]:
    """Parse a single human-eval-verus .rs file."""
    with open(filepath, 'r') as f:
        content = f.read()

    # Extract task ID
    id_section = _extract_section(content, "ID")
    task_id = id_section.strip() if id_section else Path(filepath).stem

    # Extract Verus code
    verus_code = _extract_verus_code(content)

    # Extract prompt (NL description)
    nl_prompt = _extract_section(content, "PROMPT")

    # Extract entry point
    entry_point = _extract_section(content, "ENTRY POINT")

    # Extract canonical solution
    canonical_solution = _extract_section(content, "CANONICAL SOLUTION")

    # Extract tests
    python_tests = _extract_section(content, "TEST")

    if not nl_prompt:
        return None

    return HumanEvalVerusTask(
        task_id=task_id,
        nl_prompt=nl_prompt,
        entry_point=entry_point,
        canonical_solution=canonical_solution,
        python_tests=python_tests,
        verus_code=verus_code,
        has_verus_impl=_has_real_verus_impl(verus_code),
    )


def parse_human_eval_verus(tasks_dir: str, only_with_verus: bool = False) -> list[HumanEvalVerusTask]:
    """Parse all human-eval-verus task files.

    Args:
        tasks_dir: Path to the tasks/ directory
        only_with_verus: If True, only return tasks with real Verus implementations
    """
    tasks = []
    tasks_path = Path(tasks_dir)

    for filepath in sorted(tasks_path.glob("human_eval_*.rs")):
        task = parse_task_file(str(filepath))
        if task is None:
            continue
        if only_with_verus and not task.has_verus_impl:
            continue
        tasks.append(task)

    return tasks


def save_tasks_jsonl(tasks: list[HumanEvalVerusTask], output_path: str):
    """Save parsed tasks to JSONL format."""
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, 'w') as f:
        for task in tasks:
            f.write(json.dumps(asdict(task)) + "\n")


def load_tasks_jsonl(path: str) -> list[HumanEvalVerusTask]:
    """Load tasks from JSONL format."""
    tasks = []
    with open(path, 'r') as f:
        for line in f:
            data = json.loads(line.strip())
            tasks.append(HumanEvalVerusTask(**data))
    return tasks


if __name__ == "__main__":
    import sys

    tasks_dir = sys.argv[1] if len(sys.argv) > 1 else "human-eval-verus/tasks"
    tasks = parse_human_eval_verus(tasks_dir)

    total = len(tasks)
    with_verus = sum(1 for t in tasks if t.has_verus_impl)

    print(f"Parsed {total} tasks, {with_verus} with Verus implementations")
    print()

    # Show first few tasks
    for task in tasks[:5]:
        verus_status = "HAS VERUS" if task.has_verus_impl else "no verus"
        print(f"  {task.task_id}: {task.entry_point} [{verus_status}]")

    # Save to JSONL
    output_path = "data/processed/human_eval_verus.jsonl"
    save_tasks_jsonl(tasks, output_path)
    print(f"\nSaved to {output_path}")

    # Also save only tasks with Verus
    verus_tasks = [t for t in tasks if t.has_verus_impl]
    verus_output = "data/processed/human_eval_verus_with_specs.jsonl"
    save_tasks_jsonl(verus_tasks, verus_output)
    print(f"Saved {len(verus_tasks)} tasks with Verus specs to {verus_output}")
