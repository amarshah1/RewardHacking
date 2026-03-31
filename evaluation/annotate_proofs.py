"""Use Claude Opus 4.6 (via OpenRouter) as an oracle to add Verus proof annotations.

The oracle can ONLY add proof annotations (loop invariants, assert statements,
proof blocks, ghost variables). It cannot modify, delete, or add any executable code.
This ensures we're checking the original implementation's correctness, not a rewrite.

The approach:
1. Programmatically splice the generated function body into the gold Verus spec
   (replacing the gold implementation but keeping spec functions + requires/ensures)
2. Give Claude the spliced file and ask it to ONLY add proof annotations
3. Verify the result with Verus
"""

import os
import re
from openai import OpenAI
from dotenv import load_dotenv

load_dotenv()

ORACLE_MODEL = "anthropic/claude-sonnet-4"

SYSTEM_PROMPT = """You are a Verus proof annotation oracle. You will be given a Verus file that contains:
- Imports and spec/proof helper functions (from the gold specification)
- A function signature with requires/ensures clauses (from the gold specification)
- A function body (from a generated implementation — this is the code being checked)

The function body may or may not correctly implement the specification. Your job is to add proof annotations so Verus can ATTEMPT to verify it. If the implementation is wrong, Verus should reject it — that's expected and correct.

CRITICAL RULES:
- Every line of the provided code MUST appear VERBATIM in your output, in the same order
- You MUST NOT modify, rewrite, delete, or reorder ANY existing line
- You MUST NOT change variable names, types, control flow, expressions, return values, or function signatures
- You CAN insert NEW lines BETWEEN existing lines. These new lines may ONLY contain:
  - `invariant` blocks after loop headers (for/while)
  - `decreases` clauses after loop headers
  - `assert(...)` statements
  - `proof { ... }` blocks
  - `ghost let` bindings
  - `reveal` / `reveal_with_fuel` calls
- You CAN add `use` imports at the top if needed (e.g., `use vstd::slice::*`)
- You CAN replace `arr[i]` with `*slice_index_get(arr, i)` for exec-mode slice access (Verus requirement)

Output the complete Verus file in a ```rust``` block. No explanations."""

USER_TEMPLATE = """Here is a Verus file with the gold specification and a generated function body spliced in.
Add proof annotations (invariants, asserts, proof blocks) between existing lines so Verus can attempt verification.
Do NOT modify any existing lines.

```rust
{spliced_code}
```"""


def _get_openrouter_client() -> OpenAI:
    """Create an OpenAI client configured for OpenRouter."""
    api_key = os.environ.get("OPENROUTER_API_KEY")
    if not api_key:
        raise ValueError("OPENROUTER_API_KEY not set.")
    return OpenAI(base_url="https://openrouter.ai/api/v1", api_key=api_key)


def _call_oracle(messages: list[dict], model: str = ORACLE_MODEL, max_tokens: int = 16384) -> str:
    """Call the oracle model via OpenRouter and return the response text."""
    client = _get_openrouter_client()
    response = client.chat.completions.create(
        model=model,
        messages=messages,
        max_tokens=max_tokens,
        temperature=0.0,
    )
    return response.choices[0].message.content


def _extract_all_function_bodies(code: str) -> dict[str, str]:
    """Find all functions in code and extract their bodies into a map {name: body}.

    Returns:
        Dictionary mapping function name to its body string.
    """
    bodies = {}
    lines = code.split("\n")
    i = 0
    while i < len(lines):
        line = lines[i]
        # Match fn name( or pub fn name(
        match = re.match(r'\s*(pub\s+)?fn\s+(\w+)\s*[\(<]', line.strip())
        if match:
            name = match.group(2)
            # Find body start {
            brace_depth = 0
            body_start = None
            body_end = None
            for j in range(i, len(lines)):
                for ch in lines[j]:
                    if ch == '{':
                        brace_depth += 1
                        if brace_depth == 1:
                            body_start = j
                    elif ch == '}':
                        brace_depth -= 1
                        if brace_depth == 0 and body_start is not None:
                            body_end = j
                            break
                if body_end is not None:
                    break
            
            if body_start is not None and body_end is not None:
                body_lines = lines[body_start + 1 : body_end]
                # Dedent
                non_empty = [l for l in body_lines if l.strip()]
                if non_empty:
                    min_indent = min(len(l) - len(l.lstrip()) for l in non_empty)
                    body_lines = [l[min_indent:] if len(l) >= min_indent else l for l in body_lines]
                bodies[name] = "\n".join(body_lines).strip()
                i = body_end + 1
                continue
        i += 1
    return bodies


def _replace_single_function_body(verus_code: str, name: str, new_body: str) -> str:
    """Find function 'name' in verus_code and replace its body with new_body."""
    lines = verus_code.split("\n")
    
    # Find the function signature line
    fn_line_idx = None
    for i, line in enumerate(lines):
        # Match `fn name(` or `pub fn name(`
        if re.match(rf'\s*(pub\s+)?fn\s+{re.escape(name)}\s*\(', line):
            fn_line_idx = i
            break

    if fn_line_idx is None:
        return verus_code

    # Find the boundary: the next function declaration or end of file
    next_fn_idx = len(lines)
    for i in range(fn_line_idx + 1, len(lines)):
        if re.match(r'\s*(pub\s+)?(spec\s+|proof\s+)?fn\s+\w+\s*[\(<]', lines[i]):
            next_fn_idx = i
            break
        if lines[i].strip().startswith('} // verus'):
            next_fn_idx = i
            break

    # Find opening brace
    brace_depth = 0
    zero_to_one = []
    for i in range(fn_line_idx, next_fn_idx):
        for ch in lines[i]:
            if ch == '{':
                if brace_depth == 0:
                    zero_to_one.append(i)
                brace_depth += 1
            elif ch == '}':
                brace_depth -= 1

    if not zero_to_one:
        return verus_code

    body_start_idx = zero_to_one[-1]

    # Find matching closing brace
    brace_depth = 0
    body_end_idx = None
    for i in range(body_start_idx, len(lines)):
        for ch in lines[i]:
            if ch == '{':
                brace_depth += 1
            elif ch == '}':
                brace_depth -= 1
        if brace_depth == 0:
            body_end_idx = i
            break

    if body_end_idx is None:
        return verus_code

    header = "\n".join(lines[:body_start_idx + 1])
    indented_body = "\n".join("    " + line for line in new_body.split("\n"))
    footer = "\n".join(lines[body_end_idx:])

    return f"{header}\n{indented_body}\n{footer}"


def splice_body_into_gold_spec(generated_code: str, gold_verus_code: str, entry_points: list[str]) -> str:
    """Replace the gold implementation bodies with the generated bodies.

    Args:
        generated_code: The generated Rust code (may include multiple fn signatures)
        gold_verus_code: The complete gold Verus file
        entry_points: The function names to find and replace the bodies of

    Returns:
        The spliced Verus file with the generated bodies
    """
    named_bodies = _extract_all_function_bodies(generated_code)
    
    # # Fallback: if no named functions found, treat the whole thing as the body of the last entry_point
    # if not named_bodies:
    #     named_bodies = {entry_points[-1]: generated_code.strip()}
    
    # # Special case: if only one implementation is provided but multiple names are expected,
    # # and the one implementation name doesn't match any expected name, assume it's the last one.
    # if len(named_bodies) == 1:
    #     provided_name = list(named_bodies.keys())[0]
    #     if provided_name not in entry_points:
    #         val = list(named_bodies.values())[0]
    #         named_bodies = {entry_points[-1]: val}

    current_code = gold_verus_code
    for name in entry_points:
        if name in named_bodies:
            current_code = _replace_single_function_body(current_code, name, named_bodies[name])
            
    return current_code


def annotate_with_proofs(
    generated_body: str,
    gold_verus_code: str,
    entry_point: list[str] = [],
    model: str = ORACLE_MODEL,
    max_tokens: int = 16384,
    repair_rounds: int = 3,
    verus_binary: str = "verus",
) -> tuple[str, bool, str, list[dict]]:
    """Use Claude (via OpenRouter) to add proof annotations to generated code, then verify.

    Steps:
    1. Programmatically splice generated_body into gold_verus_code
    2. Ask Claude to add ONLY proof annotations (no code changes)
    3. Run Verus, repair annotations if needed

    Args:
        generated_body: The plain Rust function implementation
        gold_verus_code: The complete gold Verus file (spec + reference impl)
        entry_point: Function names (for splicing)
        model: OpenRouter model ID for the oracle
        max_tokens: Max tokens for response
        repair_rounds: Max attempts to get annotations right
        verus_binary: Path to verus binary

    Returns:
        (annotated_code, verified, detail, rounds_log) tuple.
        rounds_log is a list of dicts with keys: round, code, verified, stdout, stderr.
    """
    from evaluation.run_verus import run_verus

    # Step 1: Programmatically splice the body in
    spliced = splice_body_into_gold_spec(generated_body, gold_verus_code, entry_point)

    prompt = USER_TEMPLATE.format(spliced_code=spliced)

    # Step 2: Ask Claude to add proof annotations only
    messages = [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": prompt},
    ]
    raw = _call_oracle(messages, model=model, max_tokens=max_tokens)
    annotated = _extract_code(raw)

    rounds_log = []

    # Step 3: Verify, repair if needed
    for round_num in range(repair_rounds):
        # Check that Claude didn't modify executable code
        exec_valid, violations = check_no_exec_changes(spliced, annotated)
        if not exec_valid:
            violation_str = "\n".join(f"  MISSING: {v}" for v in violations[:10])
            print(f"      WARNING: Claude modified executable code in round {round_num}!")
            print(violation_str)
            rounds_log.append({
                "round": round_num,
                "code": annotated,
                "verified": False,
                "n_verified": 0,
                "n_errors": 0,
                "stdout": "",
                "stderr": f"REJECTED: executable code was modified.\n{violation_str}",
                "spliced_input": spliced if round_num == 0 else "",
                "exec_check_passed": False,
                "exec_violations": violations[:20],
            })
            # Re-prompt with explicit violation feedback
            if round_num < repair_rounds - 1:
                print(f"      Re-prompting with violation feedback...")
                repair_prompt = f"""Your previous output was REJECTED because you modified executable code.

These lines from the original must appear VERBATIM but were missing or changed:
```
{chr(10).join(violations[:10])}
```

Original spliced code (do NOT modify any of these lines):
```rust
{spliced}
```

Add ONLY proof annotations (invariants, asserts, proof blocks) BETWEEN existing lines.
Output the complete Verus file in a ```rust``` block."""
                repair_messages = [
                    {"role": "system", "content": SYSTEM_PROMPT},
                    {"role": "user", "content": repair_prompt},
                ]
                raw = _call_oracle(repair_messages, model=model, max_tokens=max_tokens)
                annotated = _extract_code(raw)
            continue

        # Exec check passed — run Verus
        result = run_verus(annotated, verus_binary=verus_binary, timeout=120)

        rounds_log.append({
            "round": round_num,
            "code": annotated,
            "verified": result.verified,
            "n_verified": result.n_verified,
            "n_errors": result.n_errors,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "spliced_input": spliced if round_num == 0 else "",
            "exec_check_passed": True,
            "exec_violations": [],
        })

        if result.verified:
            return annotated, True, f"{result.n_verified} verified, 0 errors", rounds_log

        combined = result.stdout + "\n" + result.stderr
        if round_num < repair_rounds - 1:
            print(f"      Proof annotation round {round_num + 1}/{repair_rounds}: {result.n_verified} verified, {result.n_errors} errors — repairing...")
            repair_prompt = f"""The annotated code failed Verus verification.

Verus error output:
```
{combined.strip()}
```

Previous annotated code:
```rust
{annotated}
```

Remember: every line from the original spliced code must remain VERBATIM. Only add or fix proof annotations (invariants, asserts, proof blocks) between existing lines.
Output the complete corrected Verus file in a ```rust``` block."""

            repair_messages = [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": repair_prompt},
            ]
            raw = _call_oracle(repair_messages, model=model, max_tokens=max_tokens)
            annotated = _extract_code(raw)

    detail = f"{result.n_verified} verified, {result.n_errors} errors" if 'result' in dir() else "all rounds rejected (exec code modified)"
    return annotated, False, detail, rounds_log


def _is_annotation_only(line: str) -> bool:
    """Check if a line is purely a proof annotation (can be freely added/removed).

    These are lines that Claude is allowed to insert. Everything else
    is executable/structural and must be preserved verbatim.
    """
    s = line.strip()
    if not s or s.startswith("//"):
        return True  # blank lines and comments are neutral

    # Proof-only constructs
    annotation_patterns = [
        r'^assert\s*\(',           # assert(...)
        r'^assert\s+forall',       # assert forall|...|
        r'^proof\s*\{',            # proof {
        r'^ghost\s+let\s',         # ghost let ...
        r'^reveal\s*\(',           # reveal(...)
        r'^reveal_with_fuel\s*\(', # reveal_with_fuel(...)
        r'^invariant\s*$',         # invariant (keyword line)
        r'^invariant$',
        r'^decreases\s',           # decreases ...
        r'^#!\[trigger',           # #![trigger ...]
        r'^broadcast\s+use\s',     # broadcast use ...
    ]
    for pat in annotation_patterns:
        if re.match(pat, s):
            return True

    # Lines inside invariant/proof blocks: these contain spec expressions
    # that look like executable code but aren't. We handle this by checking
    # the context in the full check function instead.
    return False


def _extract_exec_lines(code: str) -> list[str]:
    """Extract executable (non-annotation) lines from Verus code.

    Strips out:
    - Lines inside `invariant` blocks (between `invariant` keyword and next `{` or statement)
    - Lines inside `proof { ... }` blocks
    - assert(...) statements
    - ghost let bindings
    - decreases clauses
    - Comments and blank lines

    Returns the remaining lines (stripped of whitespace) as an ordered list.
    """
    lines = code.split("\n")
    exec_lines = []
    proof_depth = 0  # Track nested proof { } blocks
    in_invariant = False
    invariant_done_at_brace = False

    i = 0
    while i < len(lines):
        s = lines[i].strip()

        # Track proof { ... } blocks
        if re.match(r'^proof\s*\{', s):
            proof_depth += 1
            i += 1
            continue
        if proof_depth > 0:
            proof_depth += s.count('{') - s.count('}')
            if proof_depth < 0:
                proof_depth = 0
            i += 1
            continue

        # Track invariant blocks: from `invariant` keyword to the next `{`
        # (the opening brace of the loop body)
        if re.match(r'^\}\s*//\s*verus!', s) or s == '} // verus!':
            exec_lines.append(s)
            i += 1
            continue

        if s == 'invariant':
            in_invariant = True
            i += 1
            continue
        if in_invariant:
            # Invariant block continues until we hit a line with '{' that opens
            # the loop body, or a `decreases` line
            if s == '{' or ('{' in s and not s.startswith('//')):
                in_invariant = False
                exec_lines.append(s)
            elif re.match(r'^decreases\s', s):
                pass  # skip decreases inside invariant context
            # else: skip invariant expression lines
            i += 1
            continue

        # Skip standalone annotation lines
        if _is_annotation_only(lines[i]):
            i += 1
            continue

        # Skip decreases clauses
        if re.match(r'^\s*decreases\s', s):
            i += 1
            continue

        exec_lines.append(s)
        i += 1

    return exec_lines


def check_no_exec_changes(spliced: str, annotated: str) -> tuple[bool, list[str]]:
    """Verify that all executable lines from the spliced code appear in the annotated code.

    Uses an ordered subsequence check: every exec line from spliced must appear
    in the same order in annotated. Extra lines in annotated are OK (those are
    the annotations Claude added).

    Returns:
        (is_valid, violations) where violations lists any missing/modified lines.
    """
    spliced_exec = _extract_exec_lines(spliced)
    annotated_exec = _extract_exec_lines(annotated)

    violations = []
    j = 0  # pointer into annotated_exec

    for s_line in spliced_exec:
        # Find this line in annotated_exec starting from position j
        found = False
        while j < len(annotated_exec):
            if annotated_exec[j] == s_line:
                j += 1
                found = True
                break
            j += 1

        if not found:
            violations.append(s_line)

    is_valid = len(violations) == 0
    return is_valid, violations


def _extract_code(raw: str) -> str:
    """Extract code from the last ```rust``` block."""
    blocks = re.findall(r"```(?:\w*)\n(.*?)```", raw, re.DOTALL)
    if blocks:
        return blocks[-1].strip()
    return raw.strip()
