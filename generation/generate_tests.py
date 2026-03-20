"""Generate Rust unit tests from natural language task descriptions."""

from generation.openrouter_client import generate

SYSTEM_PROMPT = """You are an expert Rust programmer. Given a natural language description of a coding task, generate Rust unit tests that thoroughly verify a correct implementation.

Requirements:
- Generate 5-8 unit tests using #[test] attribute
- Cover normal cases, edge cases, and boundary conditions
- Tests should call a function with the given entry point name
- Use assert_eq! or assert! macros
- Output ONLY the test functions (no mod block, no imports beyond what's needed)
- Each test function should be self-contained"""

USER_TEMPLATE = """Task description:
{nl_prompt}

Function name to test: {entry_point}

Generate Rust unit tests for this function. Output only the #[test] functions, nothing else."""


def generate_unit_tests(
    nl_prompt: str,
    entry_point: str,
    model: str = "qwen/qwen3-coder:free",
    temperature: float = 0.4,
) -> str:
    """Generate Rust unit tests from a natural language description.

    Args:
        nl_prompt: Natural language description of the task
        entry_point: Function name to test
        model: OpenRouter model ID
        temperature: Lower temperature for more focused test generation

    Returns:
        String containing Rust #[test] functions
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
    return _clean_test_output(completions[0])


def _clean_test_output(raw: str) -> str:
    """Clean LLM output to extract just the test functions."""
    # Strip markdown code fences if present
    lines = raw.strip().split("\n")
    cleaned = []
    in_code_block = False
    for line in lines:
        if line.strip().startswith("```"):
            in_code_block = not in_code_block
            continue
        cleaned.append(line)
    return "\n".join(cleaned).strip()
