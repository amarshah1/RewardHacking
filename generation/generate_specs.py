"""Generate Verus specifications from natural language task descriptions."""

from generation.openrouter_client import generate

SYSTEM_PROMPT = """You are an expert in Verus, a formal verification tool for Rust. Given a natural language description of a coding task, generate a Verus function signature with formal pre-conditions (requires) and post-conditions (ensures).

Requirements:
- Use Verus syntax: the function must be inside a verus! { } block
- Include `requires` clauses for pre-conditions
- Include `ensures` clauses for post-conditions that fully specify the function behavior
- Use `spec fn` for any helper specification functions needed
- Include necessary imports (use vstd::prelude::*)
- Include `fn main() {}` at the end
- The function body should be left empty or contain a placeholder `todo!()` — only write the spec
- Output ONLY the Verus code, no explanations

Example format:
use vstd::prelude::*;

verus! {

fn my_function(x: i32) -> (result: i32)
    requires
        x >= 0,
    ensures
        result == x * 2,
{
    todo!()  // implementation left for solver
}

} // verus!
fn main() {}"""

USER_TEMPLATE = """Task description:
{nl_prompt}

Function name: {entry_point}

Generate a Verus function signature with formal requires/ensures specifications for this task. Output only the Verus code."""


def generate_verus_spec(
    nl_prompt: str,
    entry_point: str,
    model: str = "qwen/qwen3-coder:free",
    temperature: float = 0.4,
) -> str:
    """Generate Verus specification from a natural language description.

    Args:
        nl_prompt: Natural language description of the task
        entry_point: Function name
        model: OpenRouter model ID
        temperature: Lower temperature for more precise spec generation

    Returns:
        String containing Verus code with specification
    """
    prompt = USER_TEMPLATE.format(nl_prompt=nl_prompt, entry_point=entry_point)
    completions = generate(
        prompt=prompt,
        system_prompt=SYSTEM_PROMPT,
        model=model,
        temperature=temperature,
        max_tokens=2048,
        n=1,
    )
    return _clean_verus_output(completions[0])


def _clean_verus_output(raw: str) -> str:
    """Clean LLM output to extract just the Verus code."""
    lines = raw.strip().split("\n")
    cleaned = []
    in_code_block = False
    for line in lines:
        if line.strip().startswith("```"):
            in_code_block = not in_code_block
            continue
        cleaned.append(line)
    return "\n".join(cleaned).strip()
