"""Generate Verus specifications from natural language task descriptions."""

from generation.openrouter_client import generate
from generation.verus_reference import VERUS_CHEAT_SHEET
from generation.few_shot_examples import build_few_shot_messages
from evaluation.run_verus import run_verus

SYSTEM_PROMPT = f"""You are an expert in Verus, a formal verification tool for Rust. Given a natural language description of a coding task, generate a Verus function signature with formal pre-conditions (requires) and post-conditions (ensures).

Requirements:
- Use Verus syntax: the function must be inside a verus! {{ }} block
- Include `requires` clauses for pre-conditions
- Include `ensures` clauses for post-conditions that fully specify the function behavior
- Use `spec fn` for any helper specification functions needed
- Include necessary imports (use vstd::prelude::*)
- Include `fn main() {{}}` at the end, OUTSIDE the verus! block
- The function body should be left empty or contain a placeholder `todo!()` — only write the spec
- First briefly explain your reasoning, then output the Verus code in a ```rust``` block

{VERUS_CHEAT_SHEET}"""

USER_TEMPLATE = """Task description:
{nl_prompt}

Function name: {entry_point}

Generate a Verus function signature with formal requires/ensures specifications for this task. Output only the Verus code."""


REPAIR_SPEC_TEMPLATE = """Task description:
{nl_prompt}

Function name: {entry_point}

Your previous Verus specification had syntax errors:
```rust
{previous_spec}
```

Verus error output:
```
{error_output}
```

Fix the specification so it compiles with Verus. The function body should remain `todo!()`. Output only the corrected Verus code."""


def check_spec_syntax(spec: str, verus_binary: str = "verus") -> tuple[bool, str]:
    """Check if a Verus spec compiles without syntax errors.

    Returns:
        (is_valid, error_output) tuple
    """
    result = run_verus(spec, verus_binary=verus_binary)
    # A spec with todo!() will never fully verify, but it should compile.
    # If stderr contains "error[E" it has syntax/type errors.
    combined = result.stdout + "\n" + result.stderr
    has_compile_errors = "error[E" in combined or "error:" in combined
    # Allow "not yet verified" / "aborting due to" from todo!() panics — those are expected
    # But actual syntax errors like E0599, E0425, E0308 mean the spec is broken
    if has_compile_errors:
        # Filter: if the ONLY error is about todo!() / panic, that's fine
        error_lines = [l for l in combined.split("\n") if "error" in l.lower()]
        real_errors = [l for l in error_lines if "todo!" not in l and "panic" not in l and "not yet implemented" not in l]
        if not real_errors:
            return True, ""
        return False, combined.strip()
    return True, ""


def generate_verus_spec(
    nl_prompt: str,
    entry_point: str,
    model: str = "qwen/qwen3-235b-a22b",
    temperature: float = 0.4,
    verus_binary: str = "verus",
    repair_rounds: int = 1,
) -> str:
    """Generate Verus specification from a natural language description.

    Args:
        nl_prompt: Natural language description of the task
        entry_point: Function name
        model: OpenRouter model ID
        temperature: Lower temperature for more precise spec generation
        verus_binary: Path to verus binary
        repair_rounds: Max repair attempts for syntax errors

    Returns:
        String containing Verus code with specification
    """
    few_shot = build_few_shot_messages("spec", USER_TEMPLATE)
    prompt = USER_TEMPLATE.format(nl_prompt=nl_prompt, entry_point=entry_point)
    completions = generate(
        prompt=prompt,
        system_prompt=SYSTEM_PROMPT,
        model=model,
        temperature=temperature,
        max_tokens=8192,
        n=1,
        few_shot_messages=few_shot,
    )
    spec = _clean_verus_output(completions[0])

    # Check syntax and repair if needed
    for round_num in range(repair_rounds):
        is_valid, error_output = check_spec_syntax(spec, verus_binary=verus_binary)
        if is_valid:
            break
        print(f"    Spec has syntax errors (round {round_num + 1}/{repair_rounds}), repairing...")
        repair_prompt = REPAIR_SPEC_TEMPLATE.format(
            nl_prompt=nl_prompt,
            entry_point=entry_point,
            previous_spec=spec,
            error_output=error_output,
        )
        try:
            repaired = generate(
                prompt=repair_prompt,
                system_prompt=SYSTEM_PROMPT,
                model=model,
                temperature=temperature,
                max_tokens=8192,
                n=1,
            )
            spec = _clean_verus_output(repaired[0])
        except Exception as e:
            print(f"    ERROR repairing spec: {type(e).__name__}: {e}")
            break

    return spec


def _clean_verus_output(raw: str) -> str:
    """Extract Verus code from the last ```rust``` (or ```) block in LLM output."""
    import re
    blocks = re.findall(r"```(?:\w*)\n(.*?)```", raw, re.DOTALL)
    if blocks:
        return blocks[-1].strip()
    return raw.strip()
