"""Generate Rust unit tests from natural language task descriptions."""

from generation.openrouter_client import generate
from generation.few_shot_examples import build_few_shot_messages

SYSTEM_PROMPT = """You are an expert Rust programmer. Given a natural language description of a coding task, generate Rust unit tests that thoroughly verify a correct implementation.

Requirements:
- Generate 5-8 unit tests using #[test] attribute
- Cover normal cases, edge cases, and boundary conditions
- Tests should call a function with the given entry point name
- Use assert_eq! or assert! macros
- CRITICAL: Always annotate empty vectors with their type (e.g. `Vec::<char>::new()`, `vec![] as Vec<String>`). Bare `vec![]` will NOT compile in assert_eq! — you MUST provide a type annotation
- Use double quotes for string literals (`"hello"`), NOT single quotes (single quotes are only for char literals like `'a'`)
- Do NOT use fixed-size arrays (`[T; N]`) where `Vec<T>` is expected. Use `vec![...]` or `.to_vec()` to create vectors
- Make sure test literals fit the parameter type (e.g. `u8` maxes at 255, `i8` at 127 — don't use values like `13441` for a `u8`)
- Each test function should be self-contained
- First briefly explain your reasoning, then output the test code in a ```rust``` block"""

USER_TEMPLATE = """Task description:
{nl_prompt}

Function signature:
{fn_signature}

Generate Rust unit tests for this function. The tests must use the types from the function signature above. Output only the #[test] functions, nothing else."""


def generate_unit_tests(
    nl_prompt: str,
    entry_point: str,
    fn_signature: str = "",
    model: str = "qwen/qwen3-coder:free",
    temperature: float = 0.4,
) -> str:
    """Generate Rust unit tests from a natural language description.

    Args:
        nl_prompt: Natural language description of the task
        entry_point: Function name to test
        fn_signature: Function signature (from gold spec) to ensure correct types
        model: OpenRouter model ID
        temperature: Lower temperature for more focused test generation

    Returns:
        String containing Rust #[test] functions
    """
    if not fn_signature:
        fn_signature = f"fn {entry_point}(...)"
    few_shot = build_few_shot_messages("tests", USER_TEMPLATE)
    prompt = USER_TEMPLATE.format(nl_prompt=nl_prompt, entry_point=entry_point, fn_signature=fn_signature)
    completions = generate(
        prompt=prompt,
        system_prompt=SYSTEM_PROMPT,
        model=model,
        temperature=temperature,
        max_tokens=2048,
        n=1,
        few_shot_messages=few_shot,
    )
    return _clean_test_output(completions[0])


def _clean_test_output(raw: str) -> str:
    """Extract test code from the last ```rust``` (or ```) block in LLM output."""
    import re
    blocks = re.findall(r"```(?:\w*)\n(.*?)```", raw, re.DOTALL)
    if blocks:
        return blocks[-1].strip()
    return raw.strip()
