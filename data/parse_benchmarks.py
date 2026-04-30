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
    impl_sig: list[str] # Signatures (named return stripped, e.g. "fn foo(x: i32) -> i32")
    verus_sig: list[str] # Verus signatures with named returns (e.g. "fn foo(x: i32) -> (result: i32)")
    verus_fn_names: list[str]  # Names of the exec fns in Verus code (may differ from entry_point)
    gold_imports: list[str]  # Import lines from the gold Verus code (e.g. "use vstd::prelude::*;")

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


def _extract_verus_impl(verus_code: str) -> tuple[list[str], list[str], list[str]]:
    """Extract (names, plain_signatures, verus_signatures) of the real Verus implementations.

    plain_signatures have named returns stripped (for Rust unit test compat).
    verus_signatures keep named returns (for Verus spec generation).
    """
    # Strip comments to avoid false positives (e.g. 'fn' in comments)
    # Strip /* ... */ comments
    verus_clean = re.sub(r"/\*.*?\*/", "", verus_code, flags=re.DOTALL)
    # Strip // ... comments
    verus_clean = re.sub(r"//.*", "", verus_clean)

    # Find all 'fn' occurrences
    pattern = r"\bfn\s+(\w+)"
    names = []
    all_signatures = []
    all_verus_signatures = []

    for match in re.finditer(pattern, verus_clean):
        start = match.start()
        name = match.group(1)

        if name == "main":
            continue

        # Check prefix for 'spec' or 'proof'
        prefix = verus_clean[max(0, start - 20) : start]
        if re.search(r"\b(spec|proof)\s+$", prefix):
            continue

        # This is an exec fn (the real implementation).
        # Find the signature: from 'fn' (or 'pub fn') to the first 'requires', 'ensures', or '{'
        sig_start = start
        pub_match = re.search(r"\bpub\s+$", prefix)
        if pub_match:
            # Check if pub is part of the same line or reasonably close
            sig_start = start - (len(prefix) - pub_match.start())

        # Find the end of the signature
        # It ends at 'requires', 'ensures', 'by', or '{'
        end_markers = ["requires", "ensures", "by", "{"]
        earliest_end = len(verus_clean)
        
        for marker in end_markers:
            # Search for marker as a whole word after the function name match
            marker_match = re.search(rf"\b{marker}\b", verus_clean[match.end():])
            if marker_match:
                marker_pos = match.end() + marker_match.start()
                if marker_pos < earliest_end:
                    earliest_end = marker_pos
            elif marker == "{":
                # Special case for '{' which might not be surrounded by word boundaries in all cases
                brace_pos = verus_clean.find("{", match.end())
                if brace_pos != -1 and brace_pos < earliest_end:
                    earliest_end = brace_pos

        signature = verus_clean[sig_start:earliest_end].strip()
        # Replace multiple spaces/newlines with single space
        signature = re.sub(r"\s+", " ", signature)

        # Save the Verus signature (with named return) before stripping
        all_verus_signatures.append(signature)

        # Replace '-> (name: type)' with '-> type'
        # Handles nested parentheses by finding the first ':' and then the matching closing ')'
        if "->" in signature:
            parts = signature.split("->", 1)
            ret_part = parts[1].strip()
            if ret_part.startswith("("):
                # Check if it follows the (name: type) pattern
                # We look for a ':' before any nested '('
                colon_pos = ret_part.find(":")
                first_open_paren = ret_part.find("(", 1)
                
                if colon_pos != -1 and (first_open_paren == -1 or colon_pos < first_open_paren):
                    # Potential (name: type) pattern
                    # Find the matching closing parenthesis for the one at index 0
                    depth = 0
                    matching_end = -1
                    for i, char in enumerate(ret_part):
                        if char == "(":
                            depth += 1
                        elif char == ")":
                            depth -= 1
                            if depth == 0:
                                matching_end = i
                                break
                    
                    if matching_end != -1:
                        # Extract the type part: from colon_pos + 1 to matching_end
                        type_content = ret_part[colon_pos + 1 : matching_end].strip()
                        signature = parts[0] + "-> " + type_content
        
        all_signatures.append(signature)
        names.append(name)

    return names, all_signatures, all_verus_signatures


def _extract_gold_imports(verus_code: str) -> list[str]:
    """Extract `use ...;` import lines from gold Verus code."""
    imports = []
    for line in verus_code.split("\n"):
        stripped = line.strip()
        if stripped.startswith("use ") and stripped.endswith(";"):
            imports.append(stripped)
    return imports


def _has_real_verus_impl(verus_code: str) -> bool:
    """Check if the Verus code has a real implementation (not just a TODO)."""
    # Strip boilerplate
    stripped = verus_code.strip()
    if not stripped:
        return False
    # Check if it's just the empty template
    if "TODO" in stripped and stripped.count("\n") < 10:
        return False
    
    # Use the implementation extractor
    name, _, _ = _extract_verus_impl(verus_code)
    return bool(name)


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

    # Extract implementation details
    verus_fn_names, impl_sig, verus_sig = _extract_verus_impl(verus_code)

    return HumanEvalVerusTask(
        task_id=task_id,
        nl_prompt=nl_prompt,
        entry_point=entry_point,
        canonical_solution=canonical_solution,
        python_tests=python_tests,
        verus_code=verus_code,
        has_verus_impl=_has_real_verus_impl(verus_code),
        impl_sig=impl_sig,
        verus_sig=verus_sig,
        verus_fn_names=verus_fn_names,
        gold_imports=_extract_gold_imports(verus_code),
    )


def parse_human_eval_verus(tasks_dir: str, only_with_verus: bool = False) -> list[HumanEvalVerusTask]:
    """Parse all human-eval-verus task files.

    Args:
        tasks_dir: Path to the tasks/ directory
        only_with_verus: If True, only return tasks with real Verus implementations
    """
    tasks = []
    seen_ids = set()
    tasks_path = Path(tasks_dir)

    for filepath in sorted(tasks_path.glob("human_eval_*.rs")):
        task = parse_task_file(str(filepath))
        if task is None:
            continue
        if only_with_verus and not task.has_verus_impl:
            continue
        # Skip duplicate task IDs (e.g. 003a, 003b, 003c all map to HumanEval/3)
        if task.task_id in seen_ids:
            continue
        seen_ids.add(task.task_id)
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
