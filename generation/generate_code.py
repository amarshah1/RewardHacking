"""Generate Rust code completions from natural language task descriptions."""

from generation.openrouter_client import generate
from generation.verus_reference import VERUS_CHEAT_SHEET

SYSTEM_PROMPT_TESTS = """You are an expert Rust programmer. Given a natural language description and unit tests, write a Rust function that passes all the tests.

Requirements:
- Write clean, correct Rust code
- The function must compile and pass all provided tests
- Output ONLY the function implementation (no tests, no main function)
- Include any necessary use/import statements at the top"""

SYSTEM_PROMPT_VERUS = f"""You are an expert in Verus, a formal verification tool for Rust. Given a natural language description and a Verus specification, write a Rust implementation that satisfies the specification.

Requirements:
- Write the implementation inside a verus! {{}} block
- Include proof annotations (loop invariants, assertions) as needed for Verus to verify
- The implementation must satisfy all requires/ensures clauses
- Include necessary imports (use vstd::prelude::*, use vstd::slice::*)
- Include `fn main() {{}}` at the end, OUTSIDE the verus! block
- Output ONLY the complete Verus code

{VERUS_CHEAT_SHEET}"""

USER_TEMPLATE_TESTS = """Task description:
{nl_prompt}

Function name: {entry_point}

Unit tests to pass:
{tests}

Write a Rust function `{entry_point}` that passes all these tests. Output only the code."""

USER_TEMPLATE_VERUS = """Task description:
{nl_prompt}

Function name: {entry_point}

Verus specification to satisfy:
{spec}

Write a complete Verus implementation that satisfies this specification. Include proof annotations as needed. Output only the code."""


def generate_code_for_tests(
    nl_prompt: str,
    entry_point: str,
    tests: str,
    model: str = "qwen/qwen3-coder:free",
    temperature: float = 0.8,
    n: int = 1,
) -> list[str]:
    """Generate code completions that should pass the given unit tests.

    Args:
        nl_prompt: Natural language description
        entry_point: Function name
        tests: Rust unit test code
        model: OpenRouter model ID
        temperature: Sampling temperature
        n: Number of completions

    Returns:
        List of code completion strings
    """
    prompt = USER_TEMPLATE_TESTS.format(
        nl_prompt=nl_prompt,
        entry_point=entry_point,
        tests=tests,
    )
    completions = generate(
        prompt=prompt,
        system_prompt=SYSTEM_PROMPT_TESTS,
        model=model,
        temperature=temperature,
        max_tokens=2048,
        n=n,
    )
    return [_clean_code_output(c) for c in completions]


def generate_code_for_verus(
    nl_prompt: str,
    entry_point: str,
    spec: str,
    model: str = "qwen/qwen3-coder:free",
    temperature: float = 0.8,
    n: int = 1,
) -> list[str]:
    """Generate code completions that should satisfy the given Verus spec.

    Args:
        nl_prompt: Natural language description
        entry_point: Function name
        spec: Verus specification code
        model: OpenRouter model ID
        temperature: Sampling temperature
        n: Number of completions

    Returns:
        List of code completion strings
    """
    prompt = USER_TEMPLATE_VERUS.format(
        nl_prompt=nl_prompt,
        entry_point=entry_point,
        spec=spec,
    )
    completions = generate(
        prompt=prompt,
        system_prompt=SYSTEM_PROMPT_VERUS,
        model=model,
        temperature=temperature,
        max_tokens=2048,
        n=n,
    )
    return [_clean_code_output(c) for c in completions]


REPAIR_SYSTEM_TESTS = """You are an expert Rust programmer. Your previous code attempt failed. Fix the code based on the error output below.

Requirements:
- Fix the compilation or test failure
- Output ONLY the corrected function implementation (no tests, no main function)
- Include any necessary use/import statements at the top"""

REPAIR_SYSTEM_VERUS = f"""You are an expert in Verus, a formal verification tool for Rust. Your previous code attempt failed Verus verification. Fix the code based on the error output below.

Requirements:
- Fix the verification errors (add/fix proof annotations, loop invariants, assertions)
- Write the implementation inside a verus! {{}} block
- Include necessary imports (use vstd::prelude::*, use vstd::slice::*)
- Include `fn main() {{}}` at the end, OUTSIDE the verus! block
- Output ONLY the complete corrected Verus code

{VERUS_CHEAT_SHEET}"""

REPAIR_TEMPLATE_TESTS = """Task description:
{nl_prompt}

Function name: {entry_point}

Unit tests to pass:
{tests}

Your previous attempt:
```rust
{previous_code}
```

Error output:
```
{error_output}
```

Fix the code so it compiles and passes all the tests. Output only the corrected code."""

REPAIR_TEMPLATE_VERUS = """Task description:
{nl_prompt}

Function name: {entry_point}

Verus specification to satisfy:
{spec}

Your previous attempt:
```rust
{previous_code}
```

Error output:
```
{error_output}
```

Fix the code so it passes Verus verification. Output only the corrected code."""


def repair_code_for_tests(
    nl_prompt: str,
    entry_point: str,
    tests: str,
    previous_code: str,
    error_output: str,
    model: str = "qwen/qwen3-coder:free",
    temperature: float = 0.8,
) -> str:
    """Generate a repaired code completion given test failure output."""
    prompt = REPAIR_TEMPLATE_TESTS.format(
        nl_prompt=nl_prompt,
        entry_point=entry_point,
        tests=tests,
        previous_code=previous_code,
        error_output=error_output,
    )
    completions = generate(
        prompt=prompt,
        system_prompt=REPAIR_SYSTEM_TESTS,
        model=model,
        temperature=temperature,
        max_tokens=2048,
        n=1,
    )
    return _clean_code_output(completions[0])


def repair_code_for_verus(
    nl_prompt: str,
    entry_point: str,
    spec: str,
    previous_code: str,
    error_output: str,
    model: str = "qwen/qwen3-coder:free",
    temperature: float = 0.8,
) -> str:
    """Generate a repaired code completion given Verus verification failure output."""
    prompt = REPAIR_TEMPLATE_VERUS.format(
        nl_prompt=nl_prompt,
        entry_point=entry_point,
        spec=spec,
        previous_code=previous_code,
        error_output=error_output,
    )
    completions = generate(
        prompt=prompt,
        system_prompt=REPAIR_SYSTEM_VERUS,
        model=model,
        temperature=temperature,
        max_tokens=2048,
        n=1,
    )
    return _clean_code_output(completions[0])


def _clean_code_output(raw: str) -> str:
    """Clean LLM output to extract just the code."""
    lines = raw.strip().split("\n")
    cleaned = []
    in_code_block = False
    for line in lines:
        if line.strip().startswith("```"):
            in_code_block = not in_code_block
            continue
        cleaned.append(line)
    return "\n".join(cleaned).strip()
