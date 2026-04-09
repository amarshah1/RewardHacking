"""Precompute and render oracle test cases from gold Verus implementations."""

from __future__ import annotations

import json
import re
import subprocess
import tempfile
from dataclasses import asdict, dataclass
from pathlib import Path

class OracleTestGenerationError(ValueError):
    """Raised when oracle IO-pair generation cannot be applied safely."""


@dataclass
class ParsedArg:
    """A single argument from an executable Rust signature."""
    name: str
    rust_type: str


@dataclass
class ParsedSignature:
    """The parsed shape of the target exec function we will fuzz."""
    fn_name: str
    args: list[ParsedArg]
    return_type: str


@dataclass
class OracleCase:
    """One cached oracle example: concrete Rust argument expressions plus an expected output."""
    arg_exprs: list[str]
    expected_expr: str


@dataclass
class OracleTestCache:
    """A reusable cache entry for one task's oracle-generated input/output pairs."""
    task_id: str
    target_fn: str
    target_signature: str
    arg_types: list[str]
    return_type: str
    cases: list[OracleCase]


def oracle_cache_filename(task_id: str) -> str:
    """Map a task id like `HumanEval/0` to a stable JSON cache filename."""
    return task_id.replace("/", "_") + ".json"


def generate_oracle_test_cache(
    task_id: str,
    verus_code: str,
    verus_fn_names: list[str],
    impl_signatures: list[str],
    verus_binary: str = "verus",
    num_cases: int = 24,
    max_trials: int = 256,
    seed: int = 0,
    timeout: int = 60,
) -> OracleTestCache:
    """Sample inputs, run the gold Verus implementation once, and cache input/output pairs."""
    if not verus_fn_names:
        raise OracleTestGenerationError("missing verus_fn_names")

    target_name = verus_fn_names[-1]
    signature_map = _build_signature_map(verus_fn_names, impl_signatures)
    signature = _parse_rust_signature(signature_map[target_name], target_name)
    sampled_arg_exprs = _sample_cases(signature, num_cases=num_cases, max_trials=max_trials, seed=seed)
    if not sampled_arg_exprs:
        raise OracleTestGenerationError("oracle fuzzing did not yield any usable test cases")

    expected_exprs = _compute_expected_outputs(
        verus_code=verus_code,
        signature=signature,
        sampled_arg_exprs=sampled_arg_exprs,
        verus_binary=verus_binary,
        timeout=timeout,
    )
    if len(expected_exprs) != len(sampled_arg_exprs):
        raise OracleTestGenerationError(
            f"oracle produced {len(expected_exprs)} outputs for {len(sampled_arg_exprs)} sampled cases"
        )

    cases = []
    for arg_exprs, expected_expr in zip(sampled_arg_exprs, expected_exprs):
        cases.append(OracleCase(arg_exprs=arg_exprs, expected_expr=expected_expr))

    return OracleTestCache(
        task_id=task_id,
        target_fn=signature.fn_name,
        target_signature=signature_map[target_name],
        arg_types=[arg.rust_type for arg in signature.args],
        return_type=signature.return_type,
        cases=cases,
    )


def save_oracle_test_cache(cache: OracleTestCache, output_path: str) -> None:
    """Write cached oracle IO pairs to disk as JSON."""
    path = Path(output_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as f:
        json.dump(asdict(cache), f, indent=2)


def load_oracle_test_cache(path: str) -> OracleTestCache:
    """Load cached oracle IO pairs from disk."""
    with open(path, "r") as f:
        data = json.load(f)
    data["cases"] = [OracleCase(**case) for case in data["cases"]]
    return OracleTestCache(**data)


def render_oracle_unit_tests(cache: OracleTestCache) -> str:
    """Render plain `assert_eq!(candidate(...), expected)` tests from cached oracle IO pairs."""
    test_blocks = []
    for idx, case in enumerate(cache.cases):
        lines = ["#[test]", f"fn oracle_case_{idx}() {{"]
        call_args = []
        for arg_idx, (arg_type, arg_expr) in enumerate(zip(cache.arg_types, case.arg_exprs)):
            binding_lines, call_expr = _render_cached_binding(f"arg_{arg_idx}", arg_expr, arg_type)
            lines.extend(f"    {line}" for line in binding_lines)
            call_args.append(call_expr)
        lines.append(f"    let expected: {cache.return_type} = {case.expected_expr};")
        lines.append(f"    assert_eq!({cache.target_fn}({', '.join(call_args)}), expected);")
        lines.append("}")
        test_blocks.append("\n".join(lines))
    return "\n\n".join(test_blocks)


def _split_top_level(text: str, delimiter: str = ",") -> list[str]:
    """Split a type/signature fragment without breaking nested generics or tuples."""
    parts: list[str] = []
    current: list[str] = []
    angle = paren = bracket = 0

    for ch in text:
        if ch == "<":
            angle += 1
        elif ch == ">":
            angle = max(0, angle - 1)
        elif ch == "(":
            paren += 1
        elif ch == ")":
            paren = max(0, paren - 1)
        elif ch == "[":
            bracket += 1
        elif ch == "]":
            bracket = max(0, bracket - 1)

        if ch == delimiter and angle == 0 and paren == 0 and bracket == 0:
            parts.append("".join(current).strip())
            current = []
            continue

        current.append(ch)

    tail = "".join(current).strip()
    if tail:
        parts.append(tail)
    return parts


def _normalize_type(rust_type: str) -> str:
    """Canonicalize Rust type spelling so parsing and rendering stay consistent."""
    rust_type = rust_type.strip()
    rust_type = re.sub(r"^&'(?:static|[a-zA-Z_]\w*)\s*", "&", rust_type)
    rust_type = re.sub(r"\s+", " ", rust_type)
    return rust_type


def _strip_ref(rust_type: str) -> str:
    """Remove all leading Rust references from an already-normalized Rust type."""
    while rust_type.startswith("&"):
        rust_type = rust_type[1:].strip()
    return rust_type


def _parse_rust_signature(signature: str, fallback_name: str) -> ParsedSignature:
    """Parse one already-valid Rust exec signature from `impl_sig` into a structured form."""
    sig = re.sub(r"\s+", " ", signature.strip())
    match = re.search(r"(?:pub\s+)?fn\s+(\w+)\s*\(", sig)
    if not match:
        raise OracleTestGenerationError(f"could not find a Rust function signature for {fallback_name}")

    fn_name = match.group(1)
    open_idx = sig.find("(", match.start())
    depth = 0
    close_idx = -1
    for idx in range(open_idx, len(sig)):
        if sig[idx] == "(":
            depth += 1
        elif sig[idx] == ")":
            depth -= 1
            if depth == 0:
                close_idx = idx
                break
    if close_idx == -1:
        raise OracleTestGenerationError(f"could not parse Rust args for {fallback_name}")

    args_text = sig[open_idx + 1 : close_idx].strip()
    return_type = sig[close_idx + 1 :].strip()
    if return_type.startswith("->"):
        return_type = return_type[2:].strip()
    else:
        return_type = "()"

    args = []
    for part in _split_top_level(args_text):
        if not part:
            continue
        name, rust_type = part.split(":", 1)
        args.append(ParsedArg(name=name.strip(), rust_type=rust_type.strip()))

    return ParsedSignature(fn_name=fn_name, args=args, return_type=return_type)


def _build_signature_map(verus_fn_names: list[str], impl_signatures: list[str]) -> dict[str, str]:
    """Align each executable Verus function name with its corresponding `impl_sig` entry."""
    if len(impl_signatures) != len(verus_fn_names):
        raise OracleTestGenerationError(
            f"impl_sig/verus_fn_names length mismatch: {len(impl_signatures)} vs {len(verus_fn_names)}"
        )
    return {fn_name: signature for fn_name, signature in zip(verus_fn_names, impl_signatures)}


def _sample_cases(
    signature: ParsedSignature,
    num_cases: int,
    max_trials: int,
    seed: int,
) -> list[list[str]]:
    """Use a temporary Rust `proptest` generator to sample distinct argument expressions."""
    sampler_source = _build_proptest_sampler_source(signature, num_cases=num_cases, max_trials=max_trials, seed=seed)
    return _run_proptest_sampler(sampler_source)


def _sampler_decl_type(rust_type: str) -> str:
    """Choose the owned Rust type sampled by the proptest harness for one argument."""
    rust_type = _normalize_type(rust_type)
    bare_type = _strip_ref(rust_type)

    if rust_type == "&str":
        return "String"
    if rust_type.startswith("&") and bare_type.startswith("Vec<"):
        return bare_type
    if rust_type.startswith("&") and bare_type.startswith("[") and ";" not in bare_type:
        inner = bare_type[1:-1].strip()
        return f"Vec<{inner}>"
    return rust_type


def _proptest_leaf_expr(rust_type: str, runner_var: str) -> str:
    """Sample one scalar Rust value using `proptest`'s built-in generators."""
    rust_type = _normalize_type(rust_type)
    if rust_type == "&str":
        rust_type = "String"
    return (
        "{ "
        f"use proptest::strategy::Strategy; "
        f"use proptest::strategy::ValueTree; "
        f"proptest::arbitrary::any::<{rust_type}>().new_tree({runner_var}).unwrap().current() "
        "}"
    )


def _sampling_expr(rust_type: str, runner_var: str, rng_var: str) -> str:
    """Generate Rust code that samples one value of the requested argument type."""
    rust_type = _normalize_type(rust_type)
    bare_type = _strip_ref(rust_type)

    if bare_type.startswith("Option<") and bare_type.endswith(">"):
        inner = bare_type[len("Option<") : -1].strip()
        inner_expr = _sampling_expr(inner, runner_var, rng_var)
        bool_expr = _proptest_leaf_expr("bool", runner_var)
        return f"{{ if {bool_expr} {{ Some({inner_expr}) }} else {{ None }} }}"

    if bare_type.startswith("Vec<") and bare_type.endswith(">"):
        inner = bare_type[len("Vec<") : -1].strip()
        inner_expr = _sampling_expr(inner, runner_var, rng_var)
        return (
            "{ "
            f"let len = sample_len({rng_var}); "
            "let mut values = Vec::with_capacity(len); "
            "for _ in 0..len { "
            f"values.push({inner_expr}); "
            "} "
            "values "
            "}"
        )

    if bare_type.startswith("[") and bare_type.endswith("]") and ";" not in bare_type:
        inner = bare_type[1:-1].strip()
        inner_expr = _sampling_expr(inner, runner_var, rng_var)
        return (
            "{ "
            f"let len = sample_len({rng_var}); "
            "let mut values = Vec::with_capacity(len); "
            "for _ in 0..len { "
            f"values.push({inner_expr}); "
            "} "
            "values "
            "}"
        )

    if bare_type.startswith("[") and ";" in bare_type and bare_type.endswith("]"):
        inner, length_str = bare_type[1:-1].split(";", 1)
        inner_expr = _sampling_expr(inner.strip(), runner_var, rng_var)
        length = int(length_str.strip())
        return (
            "{ "
            f"std::array::from_fn::<_, {length}, _>(|_| {inner_expr}) "
            "}"
        )

    if bare_type.startswith("(") and bare_type.endswith(")"):
        inner_parts = _split_top_level(bare_type[1:-1])
        pieces = [_sampling_expr(part, runner_var, rng_var) for part in inner_parts]
        if len(pieces) == 1:
            return f"({pieces[0]},)"
        return f"({', '.join(pieces)})"

    return _proptest_leaf_expr(bare_type, runner_var)


def _build_proptest_sampler_source(
    signature: ParsedSignature,
    num_cases: int,
    max_trials: int,
    seed: int,
) -> str:
    """Create a temporary Cargo `main.rs` that samples argument expressions with proptest."""
    seed_words = [(seed + i) % 256 for i in range(32)]
    seed_array = ", ".join(str(word) for word in seed_words)

    lines = [
        "use rand::Rng;",
        "use rand::SeedableRng;",
        "use rand_chacha::ChaCha20Rng;",
        "use std::collections::HashSet;",
        "",
        "fn sample_len(rng: &mut ChaCha20Rng) -> usize {",
        "    let mut len = 0usize;",
        "    while rng.gen::<bool>() {",
        "        len += 1;",
        "    }",
        "    len",
        "}",
        "",
        "fn main() {",
        f"    let seed = [{seed_array}];",
        "    let mut runner = proptest::test_runner::TestRunner::new_with_rng(",
        "        proptest::test_runner::Config::default(),",
        "        proptest::test_runner::TestRng::from_seed(",
        "            proptest::test_runner::RngAlgorithm::ChaCha,",
        "            &seed,",
        "        ),",
        "    );",
        "    let mut len_rng = ChaCha20Rng::from_seed(seed);",
        "    let mut seen = HashSet::new();",
        f"    let target_cases: usize = {num_cases};",
        f"    let max_trials: usize = {max_trials};",
        "    let mut emitted = 0usize;",
        "    let mut trials = 0usize;",
        "    while emitted < target_cases && trials < max_trials {",
    ]

    for idx, arg in enumerate(signature.args):
        sample_type = _sampler_decl_type(arg.rust_type)
        sample_expr = _sampling_expr(sample_type, "&mut runner", "&mut len_rng")
        serializer = _serializer_expr(f"value_{idx}", sample_type)
        lines.append(f"        let value_{idx}: {sample_type} = {sample_expr};")
        lines.append(f"        let expr_{idx} = {serializer};")

    expr_list = ", ".join(f"expr_{idx}.clone()" for idx in range(len(signature.args)))
    lines.extend(
        [
            f"        let exprs = vec![{expr_list}];",
            "        if seen.insert(exprs.clone()) {",
            "            println!(\"{}\", serde_json::to_string(&exprs).unwrap());",
            "            emitted += 1;",
            "        }",
            "        trials += 1;",
            "    }",
            "}",
        ]
    )
    return "\n".join(lines)


def _run_proptest_sampler(source: str) -> list[list[str]]:
    """Compile and run a temporary Cargo sampler that emits JSON argument-expression rows."""
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
        (tmp_path / "Cargo.toml").write_text(
            "\n".join(
                [
                    "[package]",
                    'name = "oracle_case_sampler"',
                    'version = "0.1.0"',
                    'edition = "2021"',
                    "",
                    "[dependencies]",
                    'proptest = "1"',
                    'rand = "0.8"',
                    'rand_chacha = "0.3"',
                    'serde_json = "1"',
                    "",
                ]
            )
        )
        src_dir = tmp_path / "src"
        src_dir.mkdir()
        (src_dir / "main.rs").write_text(source)

        result = subprocess.run(
            ["cargo", "run", "--quiet"],
            cwd=tmpdir,
            capture_output=True,
            text=True,
            timeout=120,
        )
        if result.returncode != 0:
            raise OracleTestGenerationError(
                "Rust proptest sampler failed:\n" + (result.stdout + "\n" + result.stderr).strip()
            )

        rows = []
        for line in result.stdout.splitlines():
            line = line.strip()
            if not line:
                continue
            rows.append(json.loads(line))
        return rows


def _binding_decl_type(rust_type: str) -> str:
    """Choose the local binding type used in generated Rust for one function argument."""
    rust_type = _normalize_type(rust_type)
    bare_type = _strip_ref(rust_type)

    if rust_type == "&str":
        return "&str"
    if rust_type.startswith("&") and bare_type.startswith("Vec<"):
        return bare_type
    if rust_type.startswith("&") and bare_type.startswith("[") and ";" not in bare_type:
        inner = bare_type[1:-1].strip()
        return f"Vec<{inner}>"
    return rust_type


def _render_cached_binding(name: str, expr: str, rust_type: str) -> tuple[list[str], str]:
    """Create Rust local bindings from a cached argument expression."""
    binding_type = _binding_decl_type(rust_type)
    lines = [f"let {name}: {binding_type} = {expr};"]
    if rust_type.startswith("&") and rust_type != "&str":
        return lines, f"&{name}"
    return lines, name


def _serializer_expr(expr: str, rust_type: str) -> str:
    """Generate a Rust expression that serializes a value into a Rust-literal string."""
    rust_type = _normalize_type(rust_type)
    bare_type = _strip_ref(rust_type)

    if rust_type.startswith("&") and rust_type != "&str":
        return _serializer_expr(f"({expr})", bare_type)

    if bare_type == "bool" or bare_type in {"i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "int"}:
        return f"format!(\"{{}}\", {expr})"

    if bare_type in {"char", "String", "str"} or rust_type == "&str":
        return f"format!(\"{{:?}}\", {expr})"

    if bare_type.startswith("Option<") and bare_type.endswith(">"):
        inner = bare_type[len("Option<") : -1].strip()
        inner_expr = _serializer_expr("value", "&" + inner)
        return (
            "{ "
            f"match &({expr}) {{ "
            f"Some(value) => format!(\"Some({{}})\", {inner_expr}), "
            "None => \"None\".to_string(), "
            "} "
            "}"
        )

    if bare_type.startswith("Vec<") and bare_type.endswith(">"):
        inner = bare_type[len("Vec<") : -1].strip()
        item_expr = _serializer_expr("item", "&" + inner)
        return (
            "{ "
            f"let parts: Vec<String> = ({expr}).iter().map(|item| {item_expr}).collect(); "
            "format!(\"vec![{}]\", parts.join(\", \")) "
            "}"
        )

    if bare_type.startswith("[") and bare_type.endswith("]") and ";" not in bare_type:
        inner = bare_type[1:-1].strip()
        item_expr = _serializer_expr("item", "&" + inner)
        return (
            "{ "
            f"let parts: Vec<String> = ({expr}).iter().map(|item| {item_expr}).collect(); "
            "format!(\"vec![{}]\", parts.join(\", \")) "
            "}"
        )

    if bare_type.startswith("[") and ";" in bare_type and bare_type.endswith("]"):
        inner, _ = bare_type[1:-1].split(";", 1)
        item_expr = _serializer_expr("item", "&" + inner.strip())
        return (
            "{ "
            f"let parts: Vec<String> = ({expr}).iter().map(|item| {item_expr}).collect(); "
            "format!(\"[{}]\", parts.join(\", \")) "
            "}"
        )

    if bare_type.startswith("(") and bare_type.endswith(")"):
        inner_parts = _split_top_level(bare_type[1:-1])
        rendered = [
            _serializer_expr(f"({expr}).{idx}", part)
            for idx, part in enumerate(inner_parts)
        ]
        if len(rendered) == 1:
            return f"format!(\"({{}},)\", {rendered[0]})"
        placeholders = ", ".join(["{}"] * len(rendered))
        escaped = placeholders.replace("{", "{{").replace("}", "}}")
        return f"format!(\"({escaped})\", {', '.join(rendered)})"

    raise OracleTestGenerationError(f"unsupported oracle result type for serialization: {rust_type}")


def _replace_main(verus_code: str, new_main: str) -> str:
    """Replace the crate-level `main` function with a generated driver `main`."""
    match = re.search(r"\bfn\s+main\s*\(\s*\)\s*\{", verus_code)
    if not match:
        return verus_code + "\n\n" + new_main

    brace_idx = verus_code.find("{", match.start())
    depth = 0
    end_idx = -1
    for idx in range(brace_idx, len(verus_code)):
        ch = verus_code[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                end_idx = idx
                break
    if end_idx == -1:
        raise OracleTestGenerationError("could not replace main() in verus_code")
    return verus_code[: match.start()] + new_main + verus_code[end_idx + 1 :]


def _build_driver_source(verus_code: str, signature: ParsedSignature, sampled_arg_exprs: list[list[str]]) -> str:
    """Create a temporary Verus driver that prints one serialized expected output per sampled case."""
    lines = ["fn main() {"]
    for idx, arg_exprs in enumerate(sampled_arg_exprs):
        lines.append(f"    // oracle case {idx}")
        call_args = []
        for arg_idx, (arg_expr, arg) in enumerate(zip(arg_exprs, signature.args)):
            binding_lines, call_expr = _render_cached_binding(f"arg_{idx}_{arg_idx}", arg_expr, arg.rust_type)
            lines.extend(f"    {line}" for line in binding_lines)
            call_args.append(call_expr)
        lines.append(f"    let result_{idx} = {signature.fn_name}({', '.join(call_args)});")
        serializer = _serializer_expr(f"result_{idx}", signature.return_type)
        lines.append(f"    println!(\"{{}}\", {serializer});")
    lines.append("}")
    return _replace_main(verus_code, "\n".join(lines))


def _run_compiled_driver(source: str, verus_binary: str, timeout: int) -> list[str]:
    """Compile a temporary Verus driver to a binary and return the stdout lines it prints."""
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
        src_path = tmp_path / "oracle_driver.rs"
        bin_path = tmp_path / "oracle_driver"
        src_path.write_text(source)

        compile_result = subprocess.run(
            [verus_binary, src_path.name, "--compile"],
            cwd=tmpdir,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        if compile_result.returncode != 0:
            raise OracleTestGenerationError(
                "verus --compile failed:\n"
                + (compile_result.stdout + "\n" + compile_result.stderr).strip()
            )
        if not bin_path.exists():
            raise OracleTestGenerationError(f"compiled oracle binary not found at {bin_path}")

        run_result = subprocess.run(
            [str(bin_path)],
            cwd=tmpdir,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        if run_result.returncode != 0:
            raise OracleTestGenerationError(
                "compiled oracle driver exited non-zero:\n"
                + (run_result.stdout + "\n" + run_result.stderr).strip()
            )
        return [line for line in run_result.stdout.splitlines() if line.strip()]


def _compute_expected_outputs(
    verus_code: str,
    signature: ParsedSignature,
    sampled_arg_exprs: list[list[str]],
    verus_binary: str,
    timeout: int,
) -> list[str]:
    """Run sampled cases through the compiled gold implementation and capture expected outputs."""
    driver_source = _build_driver_source(verus_code, signature, sampled_arg_exprs)
    return _run_compiled_driver(driver_source, verus_binary=verus_binary, timeout=timeout)
