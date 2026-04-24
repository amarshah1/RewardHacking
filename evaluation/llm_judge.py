"""LLM judge for finding counter-examples between two implementations.

The judge shows both implementations to an LLM and asks it to find concrete
input values where they produce different outputs. The counter-example is
validated by embedding a call to the gold implementation in a Verus file—if
Verus verifies it, the requires clause is satisfied for those inputs.
"""

import os
import re
import json
from dataclasses import dataclass, asdict
from typing import Optional

from openai import OpenAI
from dotenv import load_dotenv

load_dotenv()

DEFAULT_JUDGE_MODEL = "anthropic/claude-sonnet-4"

_SYSTEM_PROMPT = """\
You are an expert Rust/Verus programmer. You will be given two implementations \
of the same function and must determine whether they are semantically equivalent \
on all valid inputs (i.e., inputs satisfying the `requires` preconditions).

If a counter-example exists — concrete inputs where the two implementations \
return different values — provide it as executable Verus exec code.
If the implementations are equivalent on all valid inputs, say so clearly."""

_USER_TEMPLATE = """\
## Task Description
{nl_description}

## Function Signature and Preconditions
```rust
{fn_signature}
```

## Gold-Standard Implementation
```rust
{gold_impl}
```

## Generated Implementation (under evaluation)
```rust
{generated_impl}
```

## Instructions
Analyze both implementations and find a **counter-example**: concrete input \
values where they return **different** outputs.

Requirements:
- Inputs must satisfy the `requires` clause (preconditions).
- Provide a Verus exec function body that creates the concrete inputs and calls \
the **gold** function (Verus will verify the requires clause is satisfied for \
those inputs).
- Use concrete Rust literals (e.g. `vec![1i64, 2i64]`, `3i64`, `true`, etc.).
- For slice parameters (`&[T]`), create a `Vec<T>` and pass `.as_slice()`.

## Response Format

If a counter-example EXISTS:
```
COUNTER-EXAMPLE: YES
GOLD RETURNS: <value>
GENERATED RETURNS: <value>
REASONING: <brief explanation of the behavioral difference>
VERUS_TEST_BODY:
```rust
    // Set up concrete inputs
    let param1: Type1 = <literal>;
    // Call the gold function — Verus checks the requires clause
    let _result = {entry_point}(<args>);
```
```

If NO counter-example exists (semantically equivalent on all valid inputs):
```
COUNTER-EXAMPLE: NO
REASONING: <brief explanation>
```
"""


@dataclass
class CounterexampleResult:
    """Result of an LLM judge counter-example search."""
    found: Optional[bool]          # Did the LLM find a counter-example?
    reasoning: str                 # LLM's reasoning
    test_body: str                 # Verus exec function body with the counter-example
    gold_returns: str              # What the LLM says the gold impl returns
    generated_returns: str         # What the LLM says the generated impl returns
    requires_satisfied: Optional[bool]  # Did Verus verify requires is satisfied?
    verus_stdout: str              # Verus output from validation
    verus_stderr: str
    llm_response: str              # Raw LLM response


def _get_client() -> OpenAI:
    api_key = os.environ.get("OPENROUTER_API_KEY")
    if not api_key:
        raise ValueError("OPENROUTER_API_KEY not set.")
    return OpenAI(base_url="https://openrouter.ai/api/v1", api_key=api_key)


def _call_judge(messages: list[dict], model: str, max_tokens: int) -> str:
    client = _get_client()
    response = client.chat.completions.create(
        model=model,
        messages=messages,
        temperature=0.0,
        max_tokens=max_tokens,
    )
    content = response.choices[0].message.content or ""
    # Strip <think>...</think> reasoning blocks (Qwen3, etc.)
    if "<think>" in content:
        content = re.sub(r"<think>.*?</think>", "", content, flags=re.DOTALL).strip()
    return content


def _parse_response(response: str) -> dict:
    """Parse the structured LLM judge response into a dict."""
    result = {
        "found": None,
        "reasoning": "",
        "test_body": "",
        "gold_returns": "",
        "generated_returns": "",
    }

    ce_match = re.search(r"COUNTER-EXAMPLE:\s*(YES|NO)", response, re.IGNORECASE)
    if ce_match:
        result["found"] = ce_match.group(1).upper() == "YES"

    reasoning_match = re.search(
        r"REASONING:\s*(.+?)(?=GOLD RETURNS:|GENERATED RETURNS:|VERUS_TEST_BODY:|$)",
        response, re.DOTALL | re.IGNORECASE,
    )
    if reasoning_match:
        result["reasoning"] = reasoning_match.group(1).strip()

    gold_match = re.search(
        r"GOLD RETURNS:\s*(.+?)(?=GENERATED RETURNS:|REASONING:|VERUS_TEST_BODY:|$)",
        response, re.DOTALL | re.IGNORECASE,
    )
    if gold_match:
        result["gold_returns"] = gold_match.group(1).strip().split("\n")[0].strip()

    gen_match = re.search(
        r"GENERATED RETURNS:\s*(.+?)(?=GOLD RETURNS:|REASONING:|VERUS_TEST_BODY:|$)",
        response, re.DOTALL | re.IGNORECASE,
    )
    if gen_match:
        result["generated_returns"] = gen_match.group(1).strip().split("\n")[0].strip()

    # The VERUS_TEST_BODY code block — look for ``` after the keyword
    body_match = re.search(
        r"VERUS_TEST_BODY:\s*```(?:rust)?\n(.*?)```",
        response, re.DOTALL,
    )
    if body_match:
        result["test_body"] = body_match.group(1).strip()

    return result


def _build_validation_file(gold_verus_code: str, test_body: str) -> str:
    """Insert a counter-example validation function into the gold Verus code.

    The inserted function creates the proposed inputs and calls the gold
    implementation. Verus verifying the file proves the requires clause is
    satisfied for those inputs.
    """
    lines = gold_verus_code.split("\n")

    # Find the last "} // verus!" closing line to insert before it
    verus_close_idx = None
    for i in range(len(lines) - 1, -1, -1):
        if re.match(r"\s*\}\s*//\s*verus!", lines[i]):
            verus_close_idx = i
            break

    indented = "\n".join(
        "    " + line if line.strip() else "" for line in test_body.split("\n")
    )
    test_fn = f"\nfn __judge_counterexample_check() {{\n{indented}\n}}\n"

    if verus_close_idx is not None:
        new_lines = lines[:verus_close_idx] + [test_fn] + lines[verus_close_idx:]
    else:
        new_lines = lines + [test_fn]

    result = "\n".join(new_lines)
    if "fn main()" not in result:
        result += "\nfn main() {}\n"
    return result


def find_counterexample(
    nl_description: str,
    fn_signature: str,
    gold_verus_code: str,
    generated_code: str,
    entry_point: str,
    model: str = DEFAULT_JUDGE_MODEL,
    verus_binary: str = "verus",
    max_tokens: int = 4096,
    verus_timeout: int = 60,
) -> CounterexampleResult:
    """Use an LLM to find a counter-example where two implementations differ.

    The counter-example is validated with Verus: a test function calling the
    gold implementation on the proposed inputs is inserted into the gold Verus
    file. If Verus verifies the file, the requires clause is satisfied.

    Args:
        nl_description: Natural language description of the function.
        fn_signature: Function signature extracted from the gold Verus code
            (includes requires/ensures clauses).
        gold_verus_code: Full gold-standard Verus implementation.
        generated_code: The generated implementation to compare against.
        entry_point: Name of the target function.
        model: LLM model to use as judge.
        verus_binary: Path to Verus binary.
        max_tokens: Max tokens for LLM response.
        verus_timeout: Seconds allowed for Verus verification.

    Returns:
        CounterexampleResult with LLM judgment and Verus validation outcome.
    """
    from evaluation.run_verus import run_verus

    prompt = _USER_TEMPLATE.format(
        nl_description=nl_description.strip(),
        fn_signature=fn_signature.strip(),
        gold_impl=gold_verus_code.strip(),
        generated_impl=generated_code.strip(),
        entry_point=entry_point,
    )

    messages = [
        {"role": "system", "content": _SYSTEM_PROMPT},
        {"role": "user", "content": prompt},
    ]

    llm_response = _call_judge(messages, model=model, max_tokens=max_tokens)
    parsed = _parse_response(llm_response)

    if not parsed["found"]:
        return CounterexampleResult(
            found=parsed["found"],
            reasoning=parsed["reasoning"],
            test_body="",
            gold_returns="",
            generated_returns="",
            requires_satisfied=None,
            verus_stdout="",
            verus_stderr="",
            llm_response=llm_response,
        )

    # Validate that the proposed counter-example satisfies the requires clause
    requires_satisfied = None
    verus_stdout = ""
    verus_stderr = ""

    test_body = parsed["test_body"]
    if test_body:
        validation_code = _build_validation_file(gold_verus_code, test_body)
        verus_result = run_verus(
            validation_code, verus_binary=verus_binary, timeout=verus_timeout
        )
        requires_satisfied = verus_result.verified
        verus_stdout = verus_result.stdout
        verus_stderr = verus_result.stderr

    return CounterexampleResult(
        found=parsed["found"],
        reasoning=parsed["reasoning"],
        test_body=test_body,
        gold_returns=parsed["gold_returns"],
        generated_returns=parsed["generated_returns"],
        requires_satisfied=requires_satisfied,
        verus_stdout=verus_stdout,
        verus_stderr=verus_stderr,
        llm_response=llm_response,
    )


def format_judge_result(result: CounterexampleResult) -> str:
    """Format a CounterexampleResult as a human-readable text report."""
    lines = []
    if result.found is None:
        lines.append("LLM judge: no response parsed")
    elif not result.found:
        lines.append("LLM judge: NO counter-example (implementations appear equivalent)")
        lines.append(f"Reasoning: {result.reasoning}")
    else:
        lines.append("LLM judge: COUNTER-EXAMPLE FOUND")
        lines.append(f"  Gold returns:      {result.gold_returns}")
        lines.append(f"  Generated returns: {result.generated_returns}")
        lines.append(f"  Reasoning: {result.reasoning}")
        lines.append("")
        if result.requires_satisfied is True:
            lines.append("  Verus validation: PASS (requires clause satisfied for this input)")
        elif result.requires_satisfied is False:
            lines.append("  Verus validation: FAIL (requires clause NOT satisfied — counter-example is invalid)")
        else:
            lines.append("  Verus validation: not run (no test body generated)")
        if result.test_body:
            lines.append("")
            lines.append("  Counter-example test body:")
            for l in result.test_body.split("\n"):
                lines.append(f"    {l}")
    return "\n".join(lines)


def save_judge_result(result: CounterexampleResult, path: str):
    """Save a CounterexampleResult to a JSON file."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(asdict(result), f, indent=2)
