"""Generate Verus specifications from natural language task descriptions."""

from generation.openrouter_client import generate
from generation.verus_reference import VERUS_CHEAT_SHEET
from generation.generate_code import _VERUS_RULES
from generation.few_shot_examples import build_few_shot_messages
from evaluation.run_verus import run_verus

SYSTEM_PROMPT = f"""You are an expert in Verus, a formal verification tool for Rust. Given a natural language description of a coding task, generate a Verus specification with formal pre-conditions (requires) and post-conditions (ensures).

Requirements:
- Use Verus syntax: the function must be inside a verus! {{ }} block
- Include `requires` clauses for pre-conditions
- Include `ensures` clauses for post-conditions that fully specify the function behavior
- Use `spec fn` for any helper specification functions needed
- You will be given one or more function signatures. The LAST function signature is the main task function. Any earlier signatures are helper functions
- If you use helper functions, you MUST write their complete implementation with proof annotations
- Only the LAST function (the main task function) body should be `todo!()`
- Include the imports listed in the user prompt at the top of your code
- Include `fn main() {{}}` at the end, OUTSIDE the verus! block
{_VERUS_RULES}
- First briefly explain your reasoning, then output the Verus code in a ```rust``` block

{VERUS_CHEAT_SHEET}"""

USER_TEMPLATE = """Task description:
{nl_prompt}

Function signatures (the LAST one is the main function to specify; earlier ones are helpers you may implement if needed):
{fn_signature}

Available imports (include these at the top of your code):
{imports}

Generate a Verus specification for the main (last) function. Use the exact signatures above. Add requires/ensures clauses and any helper spec functions needed. If you use any of the helper function signatures, write their complete implementation with proof annotations. Only the last function body should be `todo!()`. Output only the Verus code."""


REPAIR_SPEC_USER = """Running Verus on your specification gave this error:

```
{error_output}
```

The errors are about the spec itself (types, syntax, missing decreases clauses, triggers, etc.), not the function body. Fix the specification. The function body should remain `todo!()`. Output only the corrected Verus code."""


def check_spec_syntax(spec: str, verus_binary: str = "verus") -> tuple[bool, str]:
    """Check if a Verus spec compiles by marking exec fns as external_body.

    Adds `#[verifier::external_body]` to each exec fn and replaces `todo!()`
    with `unimplemented!()`. Verus will verify the spec fns and type-check
    the signatures/ensures without needing a real body.

    Returns:
        (is_valid, error_output) tuple
    """
    import re
    # Add #[verifier::external_body] before each exec fn (not spec fn, not proof fn)
    lines = spec.split("\n")
    patched = []
    for i, line in enumerate(lines):
        stripped = line.strip()
        if re.match(r'(pub\s+)?fn\s+\w+', stripped) and \
           not re.match(r'(pub\s+)?(open\s+)?spec\s+fn', stripped) and \
           not re.match(r'(pub\s+)?proof\s+fn', stripped) and \
           'fn main' not in stripped:
            patched.append(line.rstrip().replace(stripped, f"#[verifier::external_body]\n{stripped}"))
        else:
            patched.append(line)
    test_spec = "\n".join(patched).replace("todo!()", "unimplemented!()")

    result = run_verus(test_spec, verus_binary=verus_binary)
    combined = result.stdout + "\n" + result.stderr
    has_errors = "error[E" in combined or "error:" in combined
    if has_errors:
        return False, combined.strip()
    return True, ""


def generate_verus_spec(
    nl_prompt: str,
    entry_point: str,
    model: str = "qwen/qwen3-235b-a22b",
    temperature: float = 0.4,
    verus_binary: str = "verus",
    repair_rounds: int = 1,
    gold_imports: list[str] | None = None,
    fn_signature: str = "",
    n_few_shot: int | None = None,
) -> tuple[str, list[dict], str]:
    """Generate Verus specification from a natural language description.

    Args:
        nl_prompt: Natural language description of the task
        entry_point: Function name
        model: OpenRouter model ID
        temperature: Lower temperature for more precise spec generation
        verus_binary: Path to verus binary
        repair_rounds: Max repair attempts for syntax errors
        gold_imports: Import lines from the gold Verus file (e.g. ["use vstd::prelude::*;"])
        fn_signature: Gold function signature(s) to use (ensures correct types)
        n_few_shot: Number of few-shot examples to use (None = all available)

    Returns:
        (spec, repair_history, raw_trace) — spec is the final Verus code, repair_history
        is a list of {"round": N, "spec": str, "error": str} dicts for each failed attempt,
        raw_trace is the initial LLM output (reasoning + code).
    """
    imports_str = "\n".join(gold_imports) if gold_imports else "use vstd::prelude::*;"
    if isinstance(fn_signature, list):
        fn_signature = "\n".join(fn_signature)
    if not fn_signature:
        fn_signature = f"fn {entry_point}(...)"
    few_shot = build_few_shot_messages("spec", USER_TEMPLATE, n_few_shot=n_few_shot)
    prompt = USER_TEMPLATE.format(nl_prompt=nl_prompt, entry_point=entry_point, imports=imports_str, fn_signature=fn_signature)
    completions = generate(
        prompt=prompt,
        system_prompt=SYSTEM_PROMPT,
        model=model,
        temperature=temperature,
        max_tokens=8192,
        n=1,
        few_shot_messages=few_shot,
    )
    import re
    from generation.openrouter_client import get_client, _generate_single

    raw_output = completions[0]
    spec = _strip_impl_bodies(_clean_verus_output(raw_output))
    repair_history = []

    # Build conversation from the initial generation
    conversation = [{"role": "system", "content": SYSTEM_PROMPT}]
    conversation.extend(few_shot)
    conversation.append({"role": "user", "content": prompt})
    conversation.append({"role": "assistant", "content": raw_output})

    # Check syntax and repair if needed
    for round_num in range(repair_rounds):
        # Count exec fns vs todo!() occurrences.
        # _strip_impl_bodies puts exactly one todo!() per exec fn body.
        # If there are MORE todo!()s than exec fns, one leaked into ensures/requires.
        n_exec_fns = len([
            l for l in spec.split("\n")
            if re.match(r'\s*(pub\s+)?fn\s+\w+', l.strip())
            and not re.match(r'\s*(pub\s+)?(open\s+)?spec\s+fn', l.strip())
            and not re.match(r'\s*(pub\s+)?proof\s+fn', l.strip())
            and 'fn main' not in l
        ])
        n_todos = spec.count("todo!()")
        if n_todos > n_exec_fns:
            is_valid = False
            error_output = (
                "The specification contains todo!() outside of a function body "
                "(e.g., inside an ensures or requires clause). The ensures/requires "
                "clauses must be complete — do not use todo!() in them."
            )
        else:
            is_valid, error_output = check_spec_syntax(spec, verus_binary=verus_binary)
        if is_valid:
            break
        repair_history.append({"round": round_num, "spec": spec, "error": error_output})
        print(f"    Spec has syntax errors (round {round_num + 1}/{repair_rounds}), repairing...")
        try:
            repair_msg = REPAIR_SPEC_USER.format(error_output=error_output)
            conversation.append({"role": "user", "content": repair_msg})
            client = get_client()
            raw_repair = _generate_single(client, conversation, model, temperature, 8192, False)
            conversation.append({"role": "assistant", "content": raw_repair})
            spec = _strip_impl_bodies(_clean_verus_output(raw_repair))
        except Exception as e:
            print(f"    ERROR repairing spec: {type(e).__name__}: {e}")
            break

    return spec, repair_history, raw_output


def _strip_impl_bodies(code: str, only_last: bool = True) -> str:
    """Replace exec function bodies with todo!(), keeping spec fn and helper fn bodies intact.

    If only_last is True (default), only strips the body of the LAST exec function
    (the main task function). Helper exec fn bodies are preserved — they should be
    fully implemented by the spec generator.

    The key challenge: ensures/requires clauses can contain `{` (e.g.,
    `if x > 0 { ... }`). The fn body `{` is identified as a line where
    braces are balanced at the fn indentation level — typically a standalone
    `{` line or a `{` that brings cumulative depth to 1 at fn-level indent.
    """
    import re
    lines = code.split("\n")

    # First pass: count total exec fns so we can identify the last one
    total_exec_fns = 0
    for line in lines:
        s = line.strip()
        if (re.match(r'\s*(pub\s+)?fn\s+\w+\s*[\(<]', s) and
            not re.match(r'\s*(pub\s+)?(open\s+)?spec\s+fn', s) and
            not re.match(r'\s*(pub\s+)?proof\s+fn', s) and
            'fn main' not in s):
            total_exec_fns += 1

    result = []
    exec_fn_count = 0
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Detect exec fn definitions (not spec fn)
        is_exec_fn = (
            re.match(r'\s*(pub\s+)?fn\s+\w+\s*[\(<]', stripped) and
            not re.match(r'\s*(pub\s+)?(open\s+)?spec\s+fn', stripped) and
            not re.match(r'\s*(pub\s+)?proof\s+fn', stripped) and
            'fn main' not in stripped
        )
        if is_exec_fn:
            exec_fn_count += 1
        # When only_last, only strip the last exec fn body
        if is_exec_fn and (not only_last or exec_fn_count == total_exec_fns):
            fn_indent = len(line) - len(line.lstrip())

            # Walk forward through the signature + ensures/requires to find
            # the body opening `{`. The body `{` is a standalone `{` at the
            # fn indent level (after all spec clauses have balanced their braces).
            sig_lines = [line]
            body_start = None
            cumulative_depth = stripped.count('{') - stripped.count('}')
            j = i + 1
            while j < len(lines):
                s_line = lines[j]
                s_stripped = s_line.strip()
                s_indent = len(s_line) - len(s_line.lstrip()) if s_stripped else 999

                # A standalone `{` at fn indent level is the body opener
                if s_stripped == '{' and s_indent <= fn_indent:
                    body_start = j
                    break

                # Track brace depth; when it goes to 0 then back to 1 at fn indent,
                # we might have hit the body. But prefer standalone `{`.
                cumulative_depth += s_stripped.count('{') - s_stripped.count('}')

                # If cumulative depth > 0 and this line ends with `{` at fn indent,
                # and the depth just went from 0 to 1, this could be the body opener.
                # But only if it's not inside a spec clause (requires/ensures/if/forall).
                sig_lines.append(s_line)
                j += 1

            if body_start is not None:
                # Emit everything up to (but not including) the body `{`
                result.extend(sig_lines)
                result.append(lines[body_start])  # the `{` line
                result.append(" " * (fn_indent + 4) + "todo!()")
                # Skip through body to the closing `}` at fn_indent level
                depth = 1
                j = body_start + 1
                while j < len(lines) and depth > 0:
                    depth += lines[j].count('{') - lines[j].count('}')
                    j += 1
                result.append(" " * fn_indent + "}")
                i = j
                continue
            else:
                # No standalone body `{` found — check if the code already has todo!()
                # in which case it's already a spec, emit as-is
                result.append(line)
                i += 1
                continue
        else:
            result.append(line)
            i += 1

    return "\n".join(result)


def _clean_verus_output(raw: str) -> str:
    """Extract Verus code from the last ```rust``` (or ```) block in LLM output."""
    import re
    blocks = re.findall(r"```(?:\w*)\n(.*?)```", raw, re.DOTALL)
    if blocks:
        return blocks[-1].strip()
    return raw.strip()
