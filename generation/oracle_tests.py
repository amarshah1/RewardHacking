"""Precompute and render oracle test cases from gold Verus implementations."""

from __future__ import annotations

import ast
import json
import random
import re
import shutil
import subprocess
import tempfile
from dataclasses import asdict, dataclass
from pathlib import Path

class OracleTestGenerationError(ValueError):
    """Raised when oracle IO-pair generation cannot be applied safely."""

    def __init__(
        self,
        message: str,
        preserved_driver_path: str | None = None,
        preserved_sampler_path: str | None = None,
        parsed_constraints: ParsedConstraints | None = None,
    ):
        super().__init__(message)
        self.preserved_driver_path = preserved_driver_path
        self.preserved_sampler_path = preserved_sampler_path
        self.parsed_constraints = parsed_constraints


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
class ArgConstraints:
    """Supported input restrictions for one target-function argument."""
    min_len: int = 0
    max_len: int | None = None
    shared_len_group: str | None = None
    allowed_chars: list[str] | None = None
    min_value: int | None = None
    max_value: int | None = None
    allowed_values: list[int] | None = None
    element_allowed_chars: list[str] | None = None
    element_min_value: int | None = None
    element_max_value: int | None = None
    element_allowed_values: list[int] | None = None
    sort_tuple_non_decreasing: bool = False
    distinct_elements: bool = False
    prefix_sum_max: int | None = None
    balanced_paren_groups: bool = False
    square_unique_u8_grid: bool = False
    min_const_size: int | None = None
    max_const_size: int | None = None


@dataclass
class ParsedConstraints:
    """The subset of target `requires` clauses we can enforce during sampling."""
    arg_constraints: dict[str, ArgConstraints]
    shared_length_groups: dict[str, list[str]]


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
    try:
        constraints = _parse_requires_constraints(verus_code, signature)
    except OracleTestGenerationError:
        constraints = _special_case_constraints(task_id, signature)
        if constraints is None:
            raise
    sample_case_target, sample_trial_target = _sampling_budget(signature.return_type, num_cases, max_trials)
    try:
        sampled_arg_exprs = _sample_cases(
            signature,
            constraints,
            num_cases=sample_case_target,
            max_trials=sample_trial_target,
            seed=seed,
        )
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
        cases = [
            OracleCase(arg_exprs=arg_exprs, expected_expr=expected_expr)
            for arg_exprs, expected_expr in zip(sampled_arg_exprs, expected_exprs)
        ]
        cases = _recover_output_coverage(
            task_id=task_id,
            verus_code=verus_code,
            signature=signature,
            constraints=constraints,
            cases=cases,
            verus_binary=verus_binary,
            timeout=timeout,
        )
    except OracleTestGenerationError as exc:
        exc.parsed_constraints = constraints
        raise

    cases = _select_cases(cases, signature.return_type, num_cases)

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


def _sampling_budget(return_type: str, num_cases: int, max_trials: int) -> tuple[int, int]:
    """Increase raw sampling budget for return types where output diversity matters."""
    normalized = _normalize_type(return_type)
    if normalized == "bool" or normalized.startswith("Option<"):
        return max(num_cases, num_cases * 4), max(max_trials, max_trials * 4)
    return num_cases, max_trials


def _output_bucket(expected_expr: str, return_type: str) -> str:
    """Map a rendered oracle output into a coarse diversity bucket."""
    normalized = _normalize_type(return_type)
    expr = expected_expr.strip()
    if normalized == "bool":
        return expr
    if normalized.startswith("Option<"):
        return "Some" if expr.startswith("Some(") else "None"
    return expr


def _select_cases(cases: list[OracleCase], return_type: str, num_cases: int) -> list[OracleCase]:
    """Choose a final case subset, preferring mixed bool/Option outputs when available."""
    if len(cases) <= num_cases:
        return cases

    normalized = _normalize_type(return_type)
    if normalized != "bool" and not normalized.startswith("Option<"):
        return cases[:num_cases]

    bucket_order: list[str]
    if normalized == "bool":
        bucket_order = ["true", "false"]
    else:
        bucket_order = ["Some", "None"]

    buckets: dict[str, list[OracleCase]] = {bucket: [] for bucket in bucket_order}
    extras: list[OracleCase] = []
    for case in cases:
        bucket = _output_bucket(case.expected_expr, return_type)
        if bucket in buckets:
            buckets[bucket].append(case)
        else:
            extras.append(case)

    selected: list[OracleCase] = []
    for bucket in bucket_order:
        if buckets[bucket]:
            selected.append(buckets[bucket].pop(0))

    refill_sources = [buckets[bucket] for bucket in bucket_order] + [extras]
    refill_index = 0
    while len(selected) < num_cases:
        source = refill_sources[refill_index % len(refill_sources)]
        if source:
            selected.append(source.pop(0))
        refill_index += 1
        if refill_index >= len(refill_sources) and not any(refill_sources):
            break

    return selected[:num_cases]


def _recover_output_coverage(
    task_id: str,
    verus_code: str,
    signature: ParsedSignature,
    constraints: ParsedConstraints,
    cases: list[OracleCase],
    verus_binary: str,
    timeout: int,
) -> list[OracleCase]:
    """Try a small targeted recovery pass when a bool/Option output bucket is still missing."""
    missing_buckets = _missing_output_buckets(cases, signature.return_type)

    recovery_arg_exprs = _targeted_recovery_arg_exprs(task_id, signature, constraints, missing_buckets, cases)
    if not recovery_arg_exprs:
        return cases

    recovery_outputs = _compute_expected_outputs(
        verus_code=verus_code,
        signature=signature,
        sampled_arg_exprs=recovery_arg_exprs,
        verus_binary=verus_binary,
        timeout=timeout,
    )
    recovered_cases = [
        OracleCase(arg_exprs=arg_exprs, expected_expr=expected_expr)
        for arg_exprs, expected_expr in zip(recovery_arg_exprs, recovery_outputs)
    ]
    return recovered_cases + cases


def _missing_output_buckets(cases: list[OracleCase], return_type: str) -> list[str]:
    """Return the bool/Option output buckets we still have not observed."""
    normalized = _normalize_type(return_type)
    if normalized == "bool":
        wanted_buckets = ["true", "false"]
    elif normalized.startswith("Option<"):
        wanted_buckets = ["Some", "None"]
    else:
        return []

    present = {_output_bucket(case.expected_expr, return_type) for case in cases}
    return [bucket for bucket in wanted_buckets if bucket not in present]


def _targeted_recovery_arg_exprs(
    task_id: str,
    signature: ParsedSignature,
    constraints: ParsedConstraints,
    missing_buckets: list[str],
    existing_cases: list[OracleCase],
) -> list[list[str]]:
    """Generate extra direct argument expressions for tasks with rare output buckets."""
    seen = {tuple(case.arg_exprs) for case in existing_cases}
    candidates: list[list[str]] = []

    if task_id == "HumanEval/43":
        candidates.extend(
            _pairs_sum_to_zero_recovery_cases(signature, constraints, missing_buckets, seen)
        )
    if task_id == "HumanEval/48":
        candidates.extend(
            _palindrome_recovery_cases(signature, missing_buckets, existing_cases, seen)
        )
    if task_id == "HumanEval/18":
        candidates.extend(
            _how_many_times_recovery_cases(signature, missing_buckets, existing_cases, seen)
        )
    if task_id == "HumanEval/157":
        candidates.extend(
            _right_angle_triangle_recovery_cases(signature, existing_cases, seen)
        )
    if task_id == "HumanEval/53":
        candidates.extend(
            _checked_add_recovery_cases(signature, missing_buckets, seen)
        )
    if task_id == "HumanEval/54":
        candidates.extend(
            _same_chars_recovery_cases(signature, missing_buckets, existing_cases, seen)
        )
    if task_id == "HumanEval/134":
        candidates.extend(
            _last_char_letter_recovery_cases(signature, missing_buckets, existing_cases, seen)
        )
    if task_id == "HumanEval/161":
        candidates.extend(
            _solve_alpha_bytes_recovery_cases(signature, existing_cases, seen)
        )

    unique_candidates: list[list[str]] = []
    for arg_exprs in candidates:
        key = tuple(arg_exprs)
        if key in seen:
            continue
        seen.add(key)
        unique_candidates.append(arg_exprs)
    return unique_candidates


def _pairs_sum_to_zero_recovery_cases(
    signature: ParsedSignature,
    constraints: ParsedConstraints,
    missing_buckets: list[str],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct direct cases for `HumanEval/43` that intentionally hit true/false outputs."""
    if len(signature.args) != 2:
        raise OracleTestGenerationError("HumanEval/43 recovery expected exactly two arguments")

    nums_arg = signature.args[0]
    target_arg = signature.args[1]
    if _normalize_type(nums_arg.rust_type) != "&[i32]" or _normalize_type(target_arg.rust_type) != "i32":
        raise OracleTestGenerationError("HumanEval/43 recovery expected signature `fn(&[i32], i32) -> bool`")

    nums_constraints = constraints.arg_constraints.get(nums_arg.name, ArgConstraints())
    target_constraints = constraints.arg_constraints.get(target_arg.name, ArgConstraints())

    low = nums_constraints.element_min_value if nums_constraints.element_min_value is not None else -16
    high = nums_constraints.element_max_value if nums_constraints.element_max_value is not None else 16
    zero_literal = _int_expr(0, target_constraints.min_value, target_constraints.max_value, "i32")
    one = _bounded_nonzero_int(low, high)
    two = _bounded_nonzero_int(low, high, avoid={one, -one})
    three = _bounded_nonzero_int(low, high, avoid={one, -one, two, -two})

    nums_exprs_by_bucket: dict[str, list[str]] = {
        "true": [
            _vec_expr([one, -one], "i32"),
            _vec_expr([one, three, -one], "i32"),
            _vec_expr([two, three, -two, one], "i32"),
        ],
        "false": [
            _vec_expr([one, two], "i32"),
            _vec_expr([one, two, three], "i32"),
            _vec_expr([one, one, two], "i32"),
        ],
    }

    candidates: list[list[str]] = []
    for bucket in missing_buckets:
        for nums_expr in nums_exprs_by_bucket.get(bucket, []):
            arg_exprs = [nums_expr, zero_literal]
            if tuple(arg_exprs) not in seen:
                candidates.append(arg_exprs)
    return candidates


def _palindrome_recovery_cases(
    signature: ParsedSignature,
    missing_buckets: list[str],
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct direct cases for `HumanEval/48`, preferring nontrivial true palindromes."""
    if len(signature.args) != 1:
        raise OracleTestGenerationError("HumanEval/48 recovery expected exactly one argument")
    if _normalize_type(signature.args[0].rust_type) != "&str":
        raise OracleTestGenerationError("HumanEval/48 recovery expected signature `fn(&str) -> bool`")

    needs_true = "true" in missing_buckets or not _has_nontrivial_true_string_case(existing_cases)
    needs_false = "false" in missing_buckets

    candidates: list[list[str]] = []
    if needs_true:
        for text in ["\"aba\"", "\"xywyx\"", "\"aaaaa\"", "\"abba\""]:
            if (text,) not in seen:
                candidates.append([text])
    if needs_false:
        for text in ["\"ab\"", "\"xywyz\"", "\"palindrome? no\""]:
            if (text,) not in seen:
                candidates.append([text])
    return candidates


def _checked_add_recovery_cases(
    signature: ParsedSignature,
    missing_buckets: list[str],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct direct overflow and non-overflow cases for `HumanEval/53`."""
    if len(signature.args) != 2:
        raise OracleTestGenerationError("HumanEval/53 recovery expected exactly two arguments")
    if _normalize_type(signature.args[0].rust_type) != "i32" or _normalize_type(signature.args[1].rust_type) != "i32":
        raise OracleTestGenerationError("HumanEval/53 recovery expected signature `fn(i32, i32) -> Option<i32>`")

    candidates: list[list[str]] = []
    if "None" in missing_buckets:
        for pair in [
            ["2147483647i32", "1i32"],
            ["1i32", "2147483647i32"],
            ["-2147483648i32", "-1i32"],
            ["-1i32", "-2147483648i32"],
        ]:
            if tuple(pair) not in seen:
                candidates.append(pair)

    if "Some" in missing_buckets:
        for pair in [
            ["0i32", "0i32"],
            ["1i32", "-1i32"],
            ["7i32", "5i32"],
        ]:
            if tuple(pair) not in seen:
                candidates.append(pair)

    return candidates


def _how_many_times_recovery_cases(
    signature: ParsedSignature,
    missing_buckets: list[str],
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct practical positive-count cases for `HumanEval/18`.

    Note: `None` would require more than `u32::MAX` overlapping matches, which is not feasible
    for runnable cached tests.
    """
    if len(signature.args) != 2:
        raise OracleTestGenerationError("HumanEval/18 recovery expected exactly two arguments")
    if _normalize_type(signature.args[0].rust_type) != "Vec<char>" or _normalize_type(signature.args[1].rust_type) != "Vec<char>":
        raise OracleTestGenerationError("HumanEval/18 recovery expected signature `fn(Vec<char>, Vec<char>) -> Option<u32>`")

    needs_positive_some = not _has_positive_option_case(existing_cases)
    candidates: list[list[str]] = []

    if needs_positive_some:
        for pair in _generate_how_many_times_positive_cases():
            if tuple(pair) not in seen:
                candidates.append(pair)

    return candidates


def _right_angle_triangle_recovery_cases(
    signature: ParsedSignature,
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct larger true/false cases for `HumanEval/157`."""
    if len(signature.args) != 3:
        raise OracleTestGenerationError("HumanEval/157 recovery expected exactly three arguments")
    arg_types = [_normalize_type(arg.rust_type) for arg in signature.args]
    if arg_types != ["u32", "u32", "u32"]:
        raise OracleTestGenerationError("HumanEval/157 recovery expected signature `fn(u32, u32, u32) -> bool`")

    candidates: list[list[str]] = []
    if not _has_large_true_triangle_case(existing_cases):
        rng = random.Random(157)
        large_ms = [20, 24, 30, 36]
        for idx in range(8):
            if idx < len(large_ms):
                m = large_ms[idx]
            else:
                m = rng.randint(12, 40)
            n = rng.randint(1, m - 1)
            a = m * m - n * n
            b = 2 * m * n
            c = m * m + n * n
            triple = [a, b, c]
            rng.shuffle(triple)
            arg_exprs = [_rust_numeric_literal(value, "u32") for value in triple]
            if tuple(arg_exprs) not in seen:
                candidates.append(arg_exprs)

    if not _has_large_false_triangle_case(existing_cases):
        rng = random.Random(1157)
        for _ in range(8):
            values = [rng.randint(100, 2000) for _ in range(3)]
            if _is_right_triangle(values):
                values[2] = values[2] + 1
            rng.shuffle(values)
            arg_exprs = [_rust_numeric_literal(value, "u32") for value in values]
            if tuple(arg_exprs) not in seen:
                candidates.append(arg_exprs)

    return candidates


def _same_chars_recovery_cases(
    signature: ParsedSignature,
    missing_buckets: list[str],
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct direct nontrivial equal-set cases for `HumanEval/54`."""
    if len(signature.args) != 2:
        raise OracleTestGenerationError("HumanEval/54 recovery expected exactly two arguments")
    if _normalize_type(signature.args[0].rust_type) != "&Vec<u8>" or _normalize_type(signature.args[1].rust_type) != "&Vec<u8>":
        raise OracleTestGenerationError("HumanEval/54 recovery expected signature `fn(&Vec<u8>, &Vec<u8>) -> bool`")

    needs_true = "true" in missing_buckets or not _has_substantial_vec_pair_case(existing_cases, "true")
    needs_false = "false" in missing_buckets or not _has_substantial_vec_pair_case(existing_cases, "false")

    candidates: list[list[str]] = []
    if needs_true:
        for pair in _generate_same_chars_true_pairs():
            if tuple(pair) not in seen:
                candidates.append(pair)
    if needs_false:
        for pair in _generate_same_chars_false_pairs():
            if tuple(pair) not in seen:
                candidates.append(pair)

    return candidates


def _has_positive_option_case(cases: list[OracleCase]) -> bool:
    """Check whether we already have an `Option` case of the form `Some(n)` with `n > 0`."""
    for case in cases:
        expr = case.expected_expr.strip()
        match = re.fullmatch(r"Some\((\d+)\)", expr)
        if match and int(match.group(1)) > 0:
            return True
    return False


def _has_large_true_triangle_case(cases: list[OracleCase]) -> bool:
    """Check whether we already have a true triangle case with all sides > 0 and a large side."""
    for case in cases:
        if case.expected_expr.strip() != "true" or len(case.arg_exprs) != 3:
            continue
        try:
            values = [int(re.sub(r"u32$", "", expr.strip())) for expr in case.arg_exprs]
        except ValueError:
            continue
        if all(value > 0 for value in values) and max(values) >= 100:
            return True
    return False


def _has_large_false_triangle_case(cases: list[OracleCase]) -> bool:
    """Check whether we already have a false triangle case with a large side."""
    for case in cases:
        if case.expected_expr.strip() != "false" or len(case.arg_exprs) != 3:
            continue
        try:
            values = [int(re.sub(r"u32$", "", expr.strip())) for expr in case.arg_exprs]
        except ValueError:
            continue
        if max(values) >= 100:
            return True
    return False


def _is_right_triangle(values: list[int]) -> bool:
    """Check the Pythagorean condition for one concrete triple."""
    if len(values) != 3:
        return False
    a, b, c = values
    return (
        a * a + b * b == c * c
        or a * a + c * c == b * b
        or b * b + c * c == a * a
    )


def _last_char_letter_recovery_cases(
    signature: ParsedSignature,
    missing_buckets: list[str],
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct direct true/false cases for `HumanEval/134`."""
    if len(signature.args) != 1:
        raise OracleTestGenerationError("HumanEval/134 recovery expected exactly one argument")
    if _normalize_type(signature.args[0].rust_type) != "&str":
        raise OracleTestGenerationError("HumanEval/134 recovery expected signature `fn(&str) -> bool`")

    needs_true = "true" in missing_buckets or not _has_nontrivial_true_string_case(existing_cases)
    needs_false = "false" in missing_buckets or not _has_last_char_letter_false_mix(existing_cases)

    candidates: list[list[str]] = []
    if needs_true:
        for text in _generate_last_char_letter_true_cases():
            if (text,) not in seen:
                candidates.append([text])
    if needs_false:
        for text in _generate_last_char_letter_false_cases():
            if (text,) not in seen:
                candidates.append([text])

    return candidates


def _solve_alpha_bytes_recovery_cases(
    signature: ParsedSignature,
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct mixed byte-vector cases for `HumanEval/161` that include alphabetic ASCII bytes."""
    if len(signature.args) != 1:
        raise OracleTestGenerationError("HumanEval/161 recovery expected exactly one argument")
    if _normalize_type(signature.args[0].rust_type) != "&Vec<u8>":
        raise OracleTestGenerationError("HumanEval/161 recovery expected signature `fn(&Vec<u8>) -> Vec<u8>`")
    if _has_ascii_alpha_u8_vec_case(existing_cases):
        return []

    rng = random.Random(161)
    candidates: list[list[str]] = []
    alpha_values = list(range(65, 91)) + list(range(97, 123))

    for _ in range(8):
        length = max(1, 1 + _python_geometric_length(rng))
        values = [_u8_recovery_value(rng) for _ in range(length)]
        n_alpha = rng.randint(1, length)
        insert_indices = rng.sample(range(length), k=n_alpha)
        for insert_idx in insert_indices:
            values[insert_idx] = rng.choice(alpha_values)
        arg_exprs = [_vec_expr(values, "u8")]
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    return candidates


def _generate_last_char_letter_true_cases() -> list[str]:
    """Generate strings that satisfy `HumanEval/134` by ending in a standalone ASCII letter."""
    rng = random.Random(134)
    cases: list[str] = []
    for idx in range(8):
        last_char = rng.choice("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
        if idx == 0:
            cases.append(_rust_string_literal(last_char))
            continue

        prefix_len = max(1, _python_geometric_length(rng))
        prefix = "".join(_random_ascii_text_char(rng) for _ in range(prefix_len))
        cases.append(_rust_string_literal(prefix + " " + last_char))
    return cases


def _generate_last_char_letter_false_cases() -> list[str]:
    """Generate strings that fail `HumanEval/134` in a few different ways."""
    rng = random.Random(1134)
    cases = [_rust_string_literal("")]

    for idx in range(3):
        last_char = _random_non_alphabetic_ascii_char(rng)
        if idx == 0:
            cases.append(_rust_string_literal(last_char))
            continue
        prefix_len = max(1, _python_geometric_length(rng))
        prefix = "".join(_random_ascii_text_char(rng) for _ in range(prefix_len))
        cases.append(_rust_string_literal(prefix + " " + last_char))

    for _ in range(4):
        last_char = rng.choice("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
        prefix_len = max(1, _python_geometric_length(rng))
        prefix = "".join(_random_non_whitespace_ascii_char(rng) for _ in range(prefix_len))
        cases.append(_rust_string_literal(prefix + last_char))

    return cases


def _has_ascii_alpha_u8_vec_case(cases: list[OracleCase]) -> bool:
    """Check whether we already have a `Vec<u8>` input containing an ASCII letter byte."""
    for case in cases:
        if len(case.arg_exprs) != 1:
            continue
        values = _parse_u8_vec_literal(case.arg_exprs[0])
        if values is None:
            continue
        if any(65 <= value <= 90 or 97 <= value <= 122 for value in values):
            return True
    return False


def _parse_u8_vec_literal(expr: str) -> list[int] | None:
    """Parse a simple `vec![...]` of integers into Python ints for recovery checks."""
    expr = expr.strip()
    if not expr.startswith("vec![") or not expr.endswith("]"):
        return None
    inner = expr[len("vec![") : -1].strip()
    if not inner:
        return []

    values: list[int] = []
    for part in _split_top_level(inner):
        token = part.strip()
        token = re.sub(r"u8$", "", token)
        if not re.fullmatch(r"\d+", token):
            return None
        values.append(int(token))
    return values


def _has_last_char_letter_false_mix(cases: list[OracleCase]) -> bool:
    """Check whether task 134 already has the desired mix of false-string shapes."""
    seen_kinds: set[str] = set()
    for case in cases:
        if case.expected_expr.strip() != "false" or len(case.arg_exprs) != 1:
            continue
        kind = _last_char_letter_false_kind(case.arg_exprs[0])
        if kind is not None:
            seen_kinds.add(kind)
    return {"empty", "standalone_non_alpha", "non_standalone_letter"}.issubset(seen_kinds)


def _last_char_letter_false_kind(expr: str) -> str | None:
    """Classify one Rust string literal into the false-case shapes relevant for task 134."""
    text = _parse_simple_rust_string_literal(expr)
    if text is None:
        return None
    if text == "":
        return "empty"
    if len(text) == 1 and not text[-1].isalpha():
        return "standalone_non_alpha"
    if len(text) >= 2 and text[-1].isalpha() and not text[-2].isspace():
        return "non_standalone_letter"
    return None


def _random_ascii_text_char(rng: random.Random) -> str:
    """Generate one ordinary printable ASCII character for recovery-string prefixes."""
    alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,!?;:-_'/"
    return rng.choice(alphabet)


def _random_non_alphabetic_ascii_char(rng: random.Random) -> str:
    """Generate one printable ASCII character that is not alphabetic."""
    while True:
        candidate = chr(rng.randrange(32, 127))
        if not candidate.isalpha():
            return candidate


def _random_non_whitespace_ascii_char(rng: random.Random) -> str:
    """Generate one printable ASCII character that is not whitespace."""
    while True:
        candidate = chr(rng.randrange(33, 127))
        if not candidate.isspace():
            return candidate


def _generate_how_many_times_positive_cases() -> list[list[str]]:
    """Generate positive-count substring cases for `HumanEval/18`."""
    rng = random.Random(18)
    cases: list[list[str]] = []

    for _ in range(8):
        substring_len = min(3, max(1, 1 + _python_geometric_length(rng)))
        repeat_count = max(1, 1 + _python_geometric_length(rng))
        substring = "".join(_random_printable_ascii_char(rng) for _ in range(substring_len))
        filler_prefix = "".join(_random_printable_ascii_char(rng) for _ in range(_python_geometric_length(rng)))
        filler_suffix = "".join(_random_printable_ascii_char(rng) for _ in range(_python_geometric_length(rng)))

        parts = [filler_prefix]
        for idx in range(repeat_count):
            parts.append(substring)
            if idx + 1 < repeat_count and rng.choice([True, False]):
                filler_len = 1 + _python_geometric_length(rng)
                parts.append("".join(_random_printable_ascii_char(rng) for _ in range(filler_len)))
        parts.append(filler_suffix)

        string = "".join(parts)
        cases.append([_char_vec_expr(string), _char_vec_expr(substring)])

    return cases


def _random_printable_ascii_char(rng: random.Random) -> str:
    """Generate one printable ASCII character."""
    return chr(rng.randrange(32, 127))


def _rust_string_literal(text: str) -> str:
    """Render a plain Rust string literal for simple ASCII recovery strings."""
    escaped = text.replace("\\", "\\\\").replace("\"", "\\\"")
    return f"\"{escaped}\""


def _parse_simple_rust_string_literal(expr: str) -> str | None:
    """Parse a basic Rust string literal into Python text for lightweight recovery checks."""
    expr = expr.strip()
    if len(expr) < 2 or not expr.startswith("\"") or not expr.endswith("\""):
        return None

    inner = re.sub(
        r"\\u\{([0-9A-Fa-f]+)\}",
        lambda match: chr(int(match.group(1), 16)),
        expr[1:-1],
    )
    try:
        return ast.literal_eval(f'"{inner}"')
    except (SyntaxError, ValueError):
        return None


def _generate_same_chars_true_pairs() -> list[list[str]]:
    """Generate long same-set vector pairs from one random base vector and a similar companion."""
    rng = random.Random(54)
    pairs: list[list[str]] = []

    for _ in range(8):
        left_len = max(10, 6 + _python_geometric_length(rng))
        right_len = max(10, 6 + _python_geometric_length(rng))
        left_values = _random_u8_vector(rng, left_len)
        alphabet = sorted(set(left_values))
        if len(alphabet) < 2:
            alphabet.append((alphabet[0] + 1) % 256)
            left_values[-1] = alphabet[-1]
            alphabet = sorted(set(left_values))
        right_values = _expand_same_chars_side(rng, alphabet, right_len)
        pairs.append([_vec_expr(left_values, "u8"), _vec_expr(right_values, "u8")])

    return pairs


def _generate_same_chars_false_pairs() -> list[list[str]]:
    """Generate long vector pairs whose distinct-value sets differ."""
    rng = random.Random(154)
    pairs: list[list[str]] = []

    for _ in range(8):
        left_len = max(10, 6 + _python_geometric_length(rng))
        right_len = max(10, 6 + _python_geometric_length(rng))
        left_values = _random_u8_vector(rng, left_len)
        left_alphabet = sorted(set(left_values))
        if len(left_alphabet) < 2:
            extra = (left_alphabet[0] + 1) % 256
            left_values[-1] = extra
            left_alphabet = sorted(set(left_values))

        extra_right = _fresh_u8_not_in(rng, set(left_alphabet))
        right_alphabet = list(left_alphabet)
        right_alphabet.append(extra_right)
        right_values = _expand_same_chars_side(rng, right_alphabet, right_len)

        if rng.choice([True, False]) and len(left_alphabet) > 1:
            dropped = rng.choice(left_alphabet)
            trimmed_left_alphabet = [value for value in left_alphabet if value != dropped]
            left_values = _expand_same_chars_side(rng, trimmed_left_alphabet, left_len)

        pairs.append([_vec_expr(left_values, "u8"), _vec_expr(right_values, "u8")])

    return pairs


def _python_geometric_length(rng: random.Random) -> int:
    """Mirror the sampler's geometric-style length generation in Python."""
    length = 0
    while rng.randrange(4) != 0:
        length += 1
    return length


def _expand_same_chars_side(rng: random.Random, alphabet: list[int], target_len: int) -> list[int]:
    """Build one side of a same-set vector pair while ensuring every alphabet element appears."""
    values = list(alphabet)
    while len(values) < target_len:
        values.append(rng.choice(alphabet))
    rng.shuffle(values)
    return values


def _random_u8_vector(rng: random.Random, target_len: int) -> list[int]:
    """Build one random `Vec<u8>` using the same fully random values style as the normal sampler."""
    return [rng.randrange(256) for _ in range(target_len)]


def _u8_recovery_value(rng: random.Random) -> int:
    """Generate one small-biased `u8` value for task-specific recovery cases."""
    value = 0
    while rng.randrange(4) != 0:
        value += 1
        if value >= 255:
            break
    return value


def _fresh_u8_not_in(rng: random.Random, used: set[int]) -> int:
    """Pick one fresh `u8` value outside an existing alphabet."""
    if len(used) >= 256:
        raise OracleTestGenerationError("no fresh u8 value available for false-case generation")
    while True:
        candidate = rng.randrange(256)
        if candidate not in used:
            return candidate


def _has_substantial_vec_pair_case(cases: list[OracleCase], expected_expr: str) -> bool:
    """Check whether we already have a long vector-pair case for the requested bool outcome."""
    for case in cases:
        if case.expected_expr.strip() != expected_expr or len(case.arg_exprs) != 2:
            continue
        left_len = _rust_vec_literal_len(case.arg_exprs[0])
        right_len = _rust_vec_literal_len(case.arg_exprs[1])
        if left_len >= 10 and right_len >= 10:
            return True
    return False


def _rust_vec_literal_len(expr: str) -> int:
    """Count top-level elements in a simple `vec![...]` literal."""
    expr = expr.strip()
    if not expr.startswith("vec![") or not expr.endswith("]"):
        return 0
    inner = expr[len("vec![") : -1].strip()
    if not inner:
        return 0
    return len(_split_top_level(inner))


def _has_nontrivial_true_string_case(cases: list[OracleCase]) -> bool:
    """Check whether we already have a true-valued string case longer than one character."""
    for case in cases:
        if case.expected_expr.strip() != "true" or len(case.arg_exprs) != 1:
            continue
        if _rust_string_literal_char_len(case.arg_exprs[0]) > 1:
            return True
    return False


def _rust_string_literal_char_len(expr: str) -> int:
    """Count characters in a simple Rust string literal such as `\"aba\"` or `\"\\u{61}\"`."""
    if len(expr) < 2 or not expr.startswith("\"") or not expr.endswith("\""):
        return 0

    inner = expr[1:-1]
    count = 0
    idx = 0
    while idx < len(inner):
        if inner[idx] != "\\":
            count += 1
            idx += 1
            continue

        if idx + 1 >= len(inner):
            count += 1
            break

        if inner[idx + 1] == "u" and idx + 2 < len(inner) and inner[idx + 2] == "{":
            end = inner.find("}", idx + 3)
            if end == -1:
                count += 1
                idx += 2
            else:
                count += 1
                idx = end + 1
            continue

        count += 1
        idx += 2

    return count


def _bounded_nonzero_int(
    min_value: int,
    max_value: int,
    avoid: set[int] | None = None,
) -> int:
    """Pick a small nonzero integer that fits within the requested inclusive bounds."""
    avoid = avoid or set()
    for candidate in [1, -1, 2, -2, 3, -3, 4, -4, 5, -5]:
        if min_value <= candidate <= max_value and candidate not in avoid:
            return candidate
    raise OracleTestGenerationError(
        f"could not find a small nonzero recovery value in bounds [{min_value}, {max_value}]"
    )


def _int_expr(value: int, min_value: int | None, max_value: int | None, rust_type: str) -> str:
    """Render an integer literal while checking it stays inside any parsed scalar bounds."""
    if min_value is not None and value < min_value:
        raise OracleTestGenerationError(f"recovery literal {value} is below min bound {min_value}")
    if max_value is not None and value > max_value:
        raise OracleTestGenerationError(f"recovery literal {value} is above max bound {max_value}")
    return _rust_numeric_literal(value, rust_type)


def _vec_expr(values: list[int], rust_type: str) -> str:
    """Render a concrete numeric `vec![...]` expression for recovery cases."""
    rendered = ", ".join(_rust_numeric_literal(value, rust_type) for value in values)
    return f"vec![{rendered}]"


def _rust_char_literal(ch: str) -> str:
    """Render one character as a Rust char literal."""
    if len(ch) != 1:
        raise OracleTestGenerationError(f"expected exactly one character, got {ch!r}")

    escapes = {
        "\\": r"'\\'",
        "'": r"'\''",
        "\n": r"'\n'",
        "\r": r"'\r'",
        "\t": r"'\t'",
        "\0": r"'\0'",
    }
    if ch in escapes:
        return escapes[ch]
    if ord(ch) < 32 or ord(ch) == 127:
        return f"'\\u{{{ord(ch):x}}}'"
    return f"'{ch}'"


def _char_vec_expr(text: str) -> str:
    """Render a `Vec<char>` literal from plain text."""
    chars = ", ".join(_rust_char_literal(ch) for ch in text)
    return f"vec![{chars}]"


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


def _split_requires_clauses(text: str) -> list[str]:
    """Split a `requires` block on clause commas without misreading `==>` or comparison operators."""
    parts: list[str] = []
    current: list[str] = []
    paren = bracket = brace = 0
    quantifier_bars = 0

    for ch in text:
        if ch == "(":
            paren += 1
        elif ch == ")":
            paren = max(0, paren - 1)
        elif ch == "[":
            bracket += 1
        elif ch == "]":
            bracket = max(0, bracket - 1)
        elif ch == "{":
            brace += 1
        elif ch == "}":
            brace = max(0, brace - 1)
        elif ch == "|" and paren == 0 and bracket == 0 and brace == 0:
            quantifier_bars = (quantifier_bars + 1) % 2

        if ch == "," and paren == 0 and bracket == 0 and brace == 0 and quantifier_bars == 0:
            clause = "".join(current).strip()
            if clause:
                parts.append(clause)
            current = []
            continue

        current.append(ch)

    tail = "".join(current).strip()
    if tail:
        parts.append(tail)
    return parts


def _split_boolean_conjuncts(text: str) -> list[str]:
    """Split a boolean formula at top-level `&&` operators."""
    parts: list[str] = []
    current: list[str] = []
    paren = bracket = brace = 0
    idx = 0
    while idx < len(text):
        ch = text[idx]
        if ch == "(":
            paren += 1
        elif ch == ")":
            paren = max(0, paren - 1)
        elif ch == "[":
            bracket += 1
        elif ch == "]":
            bracket = max(0, bracket - 1)
        elif ch == "{":
            brace += 1
        elif ch == "}":
            brace = max(0, brace - 1)

        if (
            text[idx : idx + 2] == "&&"
            and paren == 0
            and bracket == 0
            and brace == 0
        ):
            clause = "".join(current).strip()
            if clause:
                parts.append(clause)
            current = []
            idx += 2
            continue

        current.append(ch)
        idx += 1

    tail = "".join(current).strip()
    if tail:
        parts.append(tail)
    return parts


def _normalize_type(rust_type: str) -> str:
    """Canonicalize Rust type spelling so parsing and rendering stay consistent."""
    rust_type = rust_type.strip()
    rust_type = re.sub(r"&'(?:static|[a-zA-Z_]\w*)\s*", "&", rust_type)
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
    match = re.search(r"(?:pub\s+)?fn\s+(\w+)", sig)
    if not match:
        raise OracleTestGenerationError(f"could not find a Rust function signature for {fallback_name}")

    fn_name = match.group(1)
    search_idx = match.end()
    if search_idx < len(sig) and sig[search_idx] == "<":
        depth = 0
        generic_end = -1
        for idx in range(search_idx, len(sig)):
            if sig[idx] == "<":
                depth += 1
            elif sig[idx] == ">":
                depth -= 1
                if depth == 0:
                    generic_end = idx
                    break
        if generic_end == -1:
            raise OracleTestGenerationError(f"could not parse generic parameters for {fallback_name}")
        search_idx = generic_end + 1

    open_idx = sig.find("(", search_idx)
    if open_idx == -1:
        raise OracleTestGenerationError(f"could not parse Rust args for {fallback_name}")
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
    constraints: ParsedConstraints,
    num_cases: int,
    max_trials: int,
    seed: int,
) -> list[list[str]]:
    """Use a temporary Rust `proptest` generator to sample distinct argument expressions."""
    sampler_source = _build_proptest_sampler_source(
        signature,
        constraints,
        num_cases=num_cases,
        max_trials=max_trials,
        seed=seed,
    )
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


def _proptest_leaf_expr(rust_type: str, runner_var: str, constraints: ArgConstraints | None = None) -> str:
    """Sample one scalar Rust value using `proptest`, optionally restricting it to a valid subset."""
    rust_type = _normalize_type(rust_type)
    if rust_type == "&str":
        rust_type = "String"
    constraints = constraints or ArgConstraints()
    integer_types = {"i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize"}

    if rust_type == "char" and constraints.allowed_chars:
        choices = ", ".join(_rust_char_literal(ch) for ch in constraints.allowed_chars)
        return (
            "{ "
            f"use proptest::strategy::Strategy; "
            f"use proptest::strategy::ValueTree; "
            f"proptest::sample::select(vec![{choices}]).new_tree({runner_var}).unwrap().current() "
            "}"
        )

    if rust_type in integer_types and constraints.allowed_values:
        choices = ", ".join(_rust_numeric_literal(value, rust_type) for value in constraints.allowed_values)
        return (
            "{ "
            f"use proptest::strategy::Strategy; "
            f"use proptest::strategy::ValueTree; "
            f"proptest::sample::select(vec![{choices}]).new_tree({runner_var}).unwrap().current() "
            "}"
        )

    if (
        rust_type in integer_types
        and constraints.min_value is None
        and constraints.max_value is None
    ):
        if rust_type in {"u8", "u16", "u32", "u64", "u128", "usize"}:
            return (
                "{ "
                "use proptest::strategy::Strategy; "
                "use proptest::strategy::ValueTree; "
                "let mut value = 0usize; "
                "while proptest::arbitrary::any::<u8>().new_tree("
                f"{runner_var}"
                ").unwrap().current() % 4 != 0 { "
                "value += 1; "
                "} "
                "loop { "
                f"if let Ok(candidate) = <{rust_type}>::try_from(value) {{ "
                "break candidate; "
                "} "
                "value = value.saturating_sub(1); "
                "} "
                "}"
            )
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            "let negative = proptest::arbitrary::any::<bool>().new_tree("
            f"{runner_var}"
            ").unwrap().current(); "
            "let mut magnitude = 0usize; "
            "while proptest::arbitrary::any::<u8>().new_tree("
            f"{runner_var}"
            ").unwrap().current() % 4 != 0 { "
            "magnitude += 1; "
            "} "
            "loop { "
            f"if let Ok(mag) = <{rust_type}>::try_from(magnitude) {{ "
            "if negative { "
            "break mag.saturating_neg(); "
            "} "
            "break mag; "
            "} "
            "magnitude = magnitude.saturating_sub(1); "
            "} "
            "}"
        )

    if (
        rust_type in integer_types
        and constraints.min_value is not None
        and constraints.max_value is None
    ):
        min_literal = _rust_numeric_literal(constraints.min_value, rust_type)
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            "let mut delta = 0usize; "
            "while proptest::arbitrary::any::<u8>().new_tree("
            f"{runner_var}"
            ").unwrap().current() % 4 != 0 { "
            "delta += 1; "
            "} "
            "loop { "
            f"if let Ok(delta_cast) = <{rust_type}>::try_from(delta) {{ "
            f"if let Some(candidate) = {min_literal}.checked_add(delta_cast) {{ "
            "break candidate; "
            "} "
            "} "
            "delta = delta.saturating_sub(1); "
            "} "
            "}"
        )

    if (
        rust_type in integer_types
        and constraints.min_value is None
        and constraints.max_value is not None
    ):
        max_literal = _rust_numeric_literal(constraints.max_value, rust_type)
        if rust_type in {"u8", "u16", "u32", "u64", "u128", "usize"}:
            return (
                "{ "
                "use proptest::strategy::Strategy; "
                "use proptest::strategy::ValueTree; "
                f"let max_value = {max_literal}; "
                "let mut value = 0usize; "
                "while proptest::arbitrary::any::<u8>().new_tree("
                f"{runner_var}"
                ").unwrap().current() % 4 != 0 { "
                "value += 1; "
                "} "
                "loop { "
                f"if let Ok(candidate) = <{rust_type}>::try_from(value) {{ "
                "if candidate <= max_value { "
                "break candidate; "
                "} "
                "} "
                "value = value.saturating_sub(1); "
                "} "
                "}"
            )
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            f"let max_value = {max_literal}; "
            "let negative = proptest::arbitrary::any::<bool>().new_tree("
            f"{runner_var}"
            ").unwrap().current(); "
            "let mut magnitude = 0usize; "
            "while proptest::arbitrary::any::<u8>().new_tree("
            f"{runner_var}"
            ").unwrap().current() % 4 != 0 { "
            "magnitude += 1; "
            "} "
            "loop { "
            f"if let Ok(mag) = <{rust_type}>::try_from(magnitude) {{ "
            "let candidate = if negative { mag.saturating_neg() } else { mag }; "
            "if candidate <= max_value { "
            "break candidate; "
            "} "
            "} "
            "magnitude = magnitude.saturating_sub(1); "
            "} "
            "}"
        )

    if (
        rust_type in integer_types
        and constraints.min_value is not None
        and constraints.max_value is not None
    ):
        min_literal = _rust_numeric_literal(constraints.min_value, rust_type)
        max_literal = _rust_numeric_literal(constraints.max_value, rust_type)
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            f"let min_value = {min_literal}; "
            f"let max_value = {max_literal}; "
            "let span = max_value - min_value; "
            "if span == 0 { "
            "min_value "
            "} else { "
            "let mut delta = 0usize; "
            "while proptest::arbitrary::any::<u8>().new_tree("
            f"{runner_var}"
            ").unwrap().current() % 4 != 0 { "
            "delta += 1; "
            "} "
            "loop { "
            f"if let Ok(delta_cast) = <{rust_type}>::try_from(delta) {{ "
            "if delta_cast <= span { "
            "break min_value + delta_cast; "
            "} "
            "} "
            "delta = delta.saturating_sub(1); "
            "} "
            "} "
            "}"
        )

    if constraints.min_value is not None or constraints.max_value is not None:
        predicates: list[str] = []
        if constraints.min_value is not None:
            predicates.append(f"candidate >= {_rust_numeric_literal(constraints.min_value, rust_type)}")
        if constraints.max_value is not None:
            predicates.append(f"candidate <= {_rust_numeric_literal(constraints.max_value, rust_type)}")
        predicate = " && ".join(predicates)
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            "loop { "
            f"let candidate = proptest::arbitrary::any::<{rust_type}>().new_tree({runner_var}).unwrap().current(); "
            f"if {predicate} {{ break candidate; }} "
            "} "
            "}"
        )

    return (
        "{ "
        f"use proptest::strategy::Strategy; "
        f"use proptest::strategy::ValueTree; "
        f"proptest::arbitrary::any::<{rust_type}>().new_tree({runner_var}).unwrap().current() "
        "}"
    )


def _sampling_expr(
    rust_type: str,
    runner_var: str,
    arg_constraints: ArgConstraints,
    length_expr: str | None = None,
) -> str:
    """Generate Rust code that samples one value of the requested argument type under known constraints."""
    rust_type = _normalize_type(rust_type)
    bare_type = _strip_ref(rust_type)

    if rust_type == "&str":
        return _proptest_leaf_expr("String", runner_var, arg_constraints)

    if bare_type.startswith("Option<") and bare_type.endswith(">"):
        inner = bare_type[len("Option<") : -1].strip()
        inner_expr = _sampling_expr(inner, runner_var, ArgConstraints())
        bool_expr = _proptest_leaf_expr("bool", runner_var)
        return f"{{ if {bool_expr} {{ Some({inner_expr}) }} else {{ None }} }}"

    if bare_type.startswith("Vec<") and bare_type.endswith(">"):
        inner = bare_type[len("Vec<") : -1].strip()
        if arg_constraints.balanced_paren_groups:
            return _balanced_paren_groups_expr(runner_var, length_expr or _length_sampling_expr(runner_var, arg_constraints))
        inner_constraints = ArgConstraints(
            allowed_chars=arg_constraints.element_allowed_chars or arg_constraints.allowed_chars,
            min_value=arg_constraints.element_min_value,
            max_value=arg_constraints.element_max_value,
            allowed_values=arg_constraints.element_allowed_values,
        )
        if arg_constraints.prefix_sum_max is not None:
            return _numeric_vec_sampling_expr(
                inner_type=inner,
                runner_var=runner_var,
                arg_constraints=arg_constraints,
                length_expr=length_expr or _length_sampling_expr(runner_var, arg_constraints),
            )
        inner_expr = _sampling_expr(inner, runner_var, inner_constraints)
        length_expr = length_expr or _length_sampling_expr(runner_var, arg_constraints)
        push_expr = "values.push(candidate);"
        if arg_constraints.distinct_elements:
            push_expr = "if !values.contains(&candidate) { values.push(candidate); }"
        return (
            "{ "
            f"let len = {length_expr}; "
            "let mut values = Vec::with_capacity(len); "
            "while values.len() < len { "
            f"let candidate = {inner_expr}; "
            f"{push_expr} "
            "} "
            "values "
            "}"
        )

    if bare_type.startswith("[") and bare_type.endswith("]") and ";" not in bare_type:
        inner = bare_type[1:-1].strip()
        if arg_constraints.balanced_paren_groups:
            return _balanced_paren_groups_expr(runner_var, length_expr or _length_sampling_expr(runner_var, arg_constraints))
        inner_constraints = ArgConstraints(
            allowed_chars=arg_constraints.element_allowed_chars or arg_constraints.allowed_chars,
            min_value=arg_constraints.element_min_value,
            max_value=arg_constraints.element_max_value,
            allowed_values=arg_constraints.element_allowed_values,
        )
        if arg_constraints.prefix_sum_max is not None:
            return _numeric_vec_sampling_expr(
                inner_type=inner,
                runner_var=runner_var,
                arg_constraints=arg_constraints,
                length_expr=length_expr or _length_sampling_expr(runner_var, arg_constraints),
            )
        inner_expr = _sampling_expr(inner, runner_var, inner_constraints)
        length_expr = length_expr or _length_sampling_expr(runner_var, arg_constraints)
        push_expr = "values.push(candidate);"
        if arg_constraints.distinct_elements:
            push_expr = "if !values.contains(&candidate) { values.push(candidate); }"
        return (
            "{ "
            f"let len = {length_expr}; "
            "let mut values = Vec::with_capacity(len); "
            "while values.len() < len { "
            f"let candidate = {inner_expr}; "
            f"{push_expr} "
            "} "
            "values "
            "}"
        )

    if bare_type.startswith("[") and ";" in bare_type and bare_type.endswith("]"):
        inner, length_str = bare_type[1:-1].split(";", 1)
        inner_expr = _sampling_expr(inner.strip(), runner_var, ArgConstraints())
        length = int(length_str.strip())
        return (
            "{ "
            f"std::array::from_fn::<_, {length}, _>(|_| {inner_expr}) "
            "}"
        )

    if bare_type.startswith("(") and bare_type.endswith(")"):
        inner_parts = _split_top_level(bare_type[1:-1])
        if arg_constraints.sort_tuple_non_decreasing and len(inner_parts) == 2:
            left_expr = _sampling_expr(inner_parts[0], runner_var, ArgConstraints())
            right_expr = _sampling_expr(inner_parts[1], runner_var, ArgConstraints())
            return (
                "{ "
                f"let left = {left_expr}; "
                f"let right = {right_expr}; "
                "if left <= right { (left, right) } else { (right, left) } "
                "}"
            )
        pieces = [_sampling_expr(part, runner_var, ArgConstraints()) for part in inner_parts]
        if len(pieces) == 1:
            return f"({pieces[0]},)"
        return f"({', '.join(pieces)})"

    return _proptest_leaf_expr(bare_type, runner_var, arg_constraints)


def _balanced_paren_groups_expr(runner_var: str, length_expr: str) -> str:
    """Sample a `Vec<char>` that is a concatenation of balanced parenthesis groups with optional spaces."""
    return (
        "{ "
        "use proptest::strategy::Strategy; "
        "use proptest::strategy::ValueTree; "
        f"let target_len = {length_expr}; "
        "let mut values: Vec<char> = Vec::new(); "
        "while values.len() < target_len { "
        "let remaining = target_len - values.len(); "
        "let can_add_space = !values.is_empty(); "
        "if can_add_space { "
        f"let add_space = proptest::arbitrary::any::<bool>().new_tree({runner_var}).unwrap().current(); "
        "if add_space && values.len() < target_len { values.push(' '); continue; } "
        "} "
        "if remaining == 0 { break; } "
        "if remaining < 2 { values.push(' '); continue; } "
        "let mut group: Vec<char> = Vec::new(); "
        "let mut depth: usize = 0; "
        "loop { "
        "let space_left = target_len - values.len() - group.len(); "
        "if depth == 0 && !group.is_empty() { "
        f"let stop_here = proptest::arbitrary::any::<bool>().new_tree({runner_var}).unwrap().current(); "
        "if stop_here || space_left < 2 { break; } "
        "} "
        "if space_left == depth { "
        "for _ in 0..depth { group.push(')'); } "
        "depth = 0; "
        "break; "
        "} "
        "let can_open = space_left > depth + 1; "
        "let choose_open = if depth == 0 { can_open } else { "
        "can_open && "
        f"proptest::arbitrary::any::<bool>().new_tree({runner_var}).unwrap().current() "
        "}; "
        "if choose_open { "
        "group.push('('); "
        "depth += 1; "
        "} else if depth > 0 { "
        "group.push(')'); "
        "depth -= 1; "
        "} else { "
        "break; "
        "} "
        "} "
        "values.extend(group); "
        "} "
        "values "
        "}"
    )


def _numeric_vec_sampling_expr(
    inner_type: str,
    runner_var: str,
    arg_constraints: ArgConstraints,
    length_expr: str,
) -> str:
    """Sample a numeric vector while preserving a bound on every prefix sum."""
    normalized_inner = _normalize_type(inner_type)
    if normalized_inner not in {"i32", "i64", "u8", "u16", "u32", "u64", "usize"}:
        raise OracleTestGenerationError(
            f"prefix-sum constraints are only supported for primitive integer vectors, got `{inner_type}`"
        )
    if arg_constraints.prefix_sum_max is None:
        raise OracleTestGenerationError("missing prefix_sum_max for numeric vector sampling")

    base_constraints = ArgConstraints(
        min_value=arg_constraints.element_min_value,
        max_value=arg_constraints.element_max_value,
    )
    candidate_expr = _proptest_leaf_expr(normalized_inner, runner_var, base_constraints)
    max_sum = arg_constraints.prefix_sum_max
    push_expr = "values.push(candidate); let inserted = true;"
    if arg_constraints.distinct_elements:
        push_expr = "let inserted = if !values.contains(&candidate) { values.push(candidate); true } else { false };"
    return (
        "{ "
        f"let len = {length_expr}; "
        "let mut values = Vec::with_capacity(len); "
        "let mut running_sum: i128 = 0; "
        "while values.len() < len { "
        f"let candidate = {candidate_expr}; "
        f"if running_sum + (candidate as i128) <= {max_sum}i128 {{ "
        f"{push_expr} "
        "if inserted { "
        "running_sum += candidate as i128; "
        "} "
        "} "
        "} "
        "values "
        "}"
    )


def _build_proptest_sampler_source(
    signature: ParsedSignature,
    constraints: ParsedConstraints,
    num_cases: int,
    max_trials: int,
    seed: int,
) -> str:
    """Create a temporary Cargo `main.rs` that samples argument expressions with proptest."""
    seed_words = [(seed + i) % 256 for i in range(32)]
    seed_array = ", ".join(str(word) for word in seed_words)

    lines = [
        "use std::collections::HashSet;",
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
        "    let mut seen = HashSet::new();",
        f"    let target_cases: usize = {num_cases};",
        f"    let max_trials: usize = {max_trials};",
        "    let mut emitted = 0usize;",
        "    let mut trials = 0usize;",
        "    while emitted < target_cases && trials < max_trials {",
    ]

    for group_name, arg_names in constraints.shared_length_groups.items():
        group_constraints = [
            constraints.arg_constraints[arg_name]
            for arg_name in arg_names
        ]
        min_len = max(arg_constraint.min_len for arg_constraint in group_constraints)
        max_candidates = [arg_constraint.max_len for arg_constraint in group_constraints if arg_constraint.max_len is not None]
        max_len = min(max_candidates) if max_candidates else None
        lines.append(
            f"        let {group_name}: usize = {_length_sampling_expr('&mut runner', ArgConstraints(min_len=min_len, max_len=max_len))};"
        )

    for idx, arg in enumerate(signature.args):
        sample_type = _sampler_decl_type(arg.rust_type)
        arg_constraints = constraints.arg_constraints.get(arg.name, ArgConstraints())
        length_expr = arg_constraints.shared_len_group
        direct_expr = _direct_expr_sampling_code(sample_type, "&mut runner", arg_constraints, length_expr=length_expr)
        if direct_expr is not None:
            lines.append(f"        let expr_{idx}: String = {direct_expr};")
        else:
            sample_expr = _sampling_expr(sample_type, "&mut runner", arg_constraints, length_expr=length_expr)
            serializer = _serializer_expr(f"value_{idx}", sample_type)
            if _has_unresolved_const_generic_array(sample_type):
                lines.append(f"        let value_{idx} = {sample_expr};")
            else:
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


def _extract_target_requires(verus_code: str, target_name: str) -> list[str]:
    """Extract top-level `requires` clauses from the target function in `verus_code`."""
    fn_match = re.search(rf"(?:pub\s+)?fn\s+{re.escape(target_name)}\b", verus_code)
    if not fn_match:
        raise OracleTestGenerationError(f"could not locate target function `{target_name}` in verus_code")

    search_idx = fn_match.end()
    while search_idx < len(verus_code) and verus_code[search_idx].isspace():
        search_idx += 1
    if search_idx < len(verus_code) and verus_code[search_idx] == "<":
        depth = 0
        generic_end = -1
        for idx in range(search_idx, len(verus_code)):
            if verus_code[idx] == "<":
                depth += 1
            elif verus_code[idx] == ">":
                depth -= 1
                if depth == 0:
                    generic_end = idx
                    break
        if generic_end == -1:
            raise OracleTestGenerationError(f"could not parse generic parameters for `{target_name}` in verus_code")
        search_idx = generic_end + 1

    open_idx = verus_code.find("(", search_idx)
    if open_idx == -1:
        raise OracleTestGenerationError(f"could not parse target function header for `{target_name}`")
    depth = 0
    close_idx = -1
    for idx in range(open_idx, len(verus_code)):
        if verus_code[idx] == "(":
            depth += 1
        elif verus_code[idx] == ")":
            depth -= 1
            if depth == 0:
                close_idx = idx
                break
    if close_idx == -1:
        raise OracleTestGenerationError(f"could not parse target function header for `{target_name}`")

    brace_idx = verus_code.find("{", close_idx)
    if brace_idx == -1:
        raise OracleTestGenerationError(f"could not find target function body for `{target_name}`")

    preamble = verus_code[close_idx + 1 : brace_idx]
    requires_match = re.search(r"\brequires\b(.*?)(?:\bensures\b|$)", preamble, re.DOTALL)
    if not requires_match:
        return []

    requires_text = re.sub(r"//.*", "", requires_match.group(1))
    return [clause.strip() for clause in _split_requires_clauses(requires_text) if clause.strip()]


def _direct_expr_sampling_code(
    rust_type: str,
    runner_var: str,
    arg_constraints: ArgConstraints,
    length_expr: str | None = None,
) -> str | None:
    """Sample a Rust expression string directly for cases that can't be represented as a runtime typed value."""
    if not arg_constraints.square_unique_u8_grid:
        normalized = _normalize_type(rust_type)
        if normalized == "Vec<&str>":
            return (
                "{ "
                "use proptest::strategy::Strategy; "
                "use proptest::strategy::ValueTree; "
                f"let len = {length_expr or _length_sampling_expr(runner_var, arg_constraints)}; "
                "let mut parts: Vec<String> = Vec::with_capacity(len); "
                "while parts.len() < len { "
                f"let candidate = proptest::arbitrary::any::<String>().new_tree({runner_var}).unwrap().current(); "
                "parts.push(format!(\"{:?}\", candidate)); "
                "} "
                "format!(\"vec![{}]\", parts.join(\", \")) "
                "}"
            )
        return None

    normalized = _normalize_type(rust_type)
    if normalized != "[[u8; N]; N]":
        raise OracleTestGenerationError(
            f"square_unique_u8_grid only supports `[[u8; N]; N]`, got `{rust_type}`"
        )
    min_size = arg_constraints.min_const_size or 2
    max_size = arg_constraints.max_const_size or 15
    return (
        "{ "
        "use proptest::strategy::Strategy; "
        "use proptest::strategy::ValueTree; "
        f"let min_n = {min_size}usize; "
        f"let max_n = {max_size}usize; "
        "let span = max_n - min_n; "
        "let mut delta = 0usize; "
        "while proptest::arbitrary::any::<u8>().new_tree("
        f"{runner_var}"
        ").unwrap().current() % 4 != 0 { "
        "delta += 1; "
        "} "
        "let n = loop { "
        "if delta <= span { break min_n + delta; } "
        "delta = delta.saturating_sub(1); "
        "}; "
        "let total = n * n; "
        "let total_u8 = total as u8; "
        "let mut pool: Vec<u8> = (1u8..=total_u8).collect(); "
        "let mut flat: Vec<u8> = Vec::with_capacity(total); "
        "while !pool.is_empty() { "
        f"let pick = proptest::arbitrary::any::<usize>().new_tree({runner_var}).unwrap().current() % pool.len(); "
        "flat.push(pool.swap_remove(pick)); "
        "} "
        "let mut rows: Vec<String> = Vec::with_capacity(n); "
        "for row in 0..n { "
        "let start = row * n; "
        "let end = start + n; "
        "let parts: Vec<String> = flat[start..end].iter().map(|item| format!(\"{}u8\", item)).collect(); "
        "rows.push(format!(\"[{}]\", parts.join(\", \"))); "
        "} "
        "format!(\"[{}]\", rows.join(\", \")) "
        "}"
    )


def _parse_requires_constraints(verus_code: str, signature: ParsedSignature) -> ParsedConstraints:
    """Parse the supported subset of target `requires` clauses into sampler constraints."""
    arg_constraints = {arg.name: ArgConstraints() for arg in signature.args}
    shared_length_groups: dict[str, list[str]] = {}
    group_ids: dict[frozenset[str], str] = {}
    unsupported: list[str] = []

    for clause in _extract_target_requires(verus_code, signature.fn_name):
        if _apply_requires_clause(clause, signature, arg_constraints, shared_length_groups, group_ids):
            continue
        unsupported.append(clause)

    if unsupported:
        rendered = "\n".join(f"- {clause}" for clause in unsupported)
        raise OracleTestGenerationError(
            "unsupported target requires clause(s):\n" + rendered
        )

    return ParsedConstraints(
        arg_constraints=arg_constraints,
        shared_length_groups=shared_length_groups,
    )


def _special_case_constraints(task_id: str, signature: ParsedSignature) -> ParsedConstraints | None:
    """Return a hand-written sampler constraint bundle for tasks whose `requires` clause is semantic."""
    if task_id == "HumanEval/1":
        if len(signature.args) != 1:
            raise OracleTestGenerationError("HumanEval/1 special case expected exactly one argument")
        arg_name = signature.args[0].name
        return ParsedConstraints(
            arg_constraints={arg_name: ArgConstraints(balanced_paren_groups=True)},
            shared_length_groups={},
        )

    if task_id == "HumanEval/129":
        if len(signature.args) != 2:
            raise OracleTestGenerationError("HumanEval/129 special case expected exactly two arguments")
        grid_name = signature.args[0].name
        k_name = signature.args[1].name
        return ParsedConstraints(
            arg_constraints={
                grid_name: ArgConstraints(square_unique_u8_grid=True, min_const_size=2, max_const_size=15),
                k_name: ArgConstraints(),
            },
            shared_length_groups={},
        )

    return None


def _has_unresolved_const_generic_array(rust_type: str) -> bool:
    """Detect array types whose length still refers to an unresolved const generic like `N`."""
    return re.search(r"\[[^\[\]]*;\s*[A-Za-z_]\w*\s*\]", _normalize_type(rust_type)) is not None


def _apply_requires_clause(
    clause: str,
    signature: ParsedSignature,
    arg_constraints: dict[str, ArgConstraints],
    shared_length_groups: dict[str, list[str]],
    group_ids: dict[frozenset[str], str],
) -> bool:
    """Apply one supported `requires` clause to the accumulated argument constraints."""
    arg_types = {arg.name: _normalize_type(arg.rust_type) for arg in signature.args}

    if not clause.startswith("forall|") and "&&" in clause:
        conjuncts = _split_boolean_conjuncts(clause)
        if len(conjuncts) > 1:
            return all(
                _apply_requires_clause(
                    conjunct,
                    signature,
                    arg_constraints,
                    shared_length_groups,
                    group_ids,
                )
                for conjunct in conjuncts
            )

    match = re.fullmatch(r"(\w+)@?\.len\(\)\s*==\s*(\w+)@?\.len\(\)", clause)
    if match:
        left, right = match.groups()
        if left not in arg_constraints or right not in arg_constraints:
            return False
        group_key = frozenset({left, right})
        group_name = group_ids.get(group_key)
        if group_name is None:
            group_name = f"shared_len_{len(group_ids)}"
            group_ids[group_key] = group_name
            shared_length_groups[group_name] = [left, right]
        arg_constraints[left].shared_len_group = group_name
        arg_constraints[right].shared_len_group = group_name
        return True

    match = re.fullmatch(r"([A-Za-z0-9_:\-]+)\s*(<|<=)\s*(\w+)@?\.len\(\)\s*(<|<=)\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        lower, lower_op, name, upper_op, upper = match.groups()
        if name not in arg_constraints or not _is_sequence_type(arg_types[name]):
            return False
        lower_op = ">=" if lower_op == "<=" else ">"
        upper_op = "<=" if upper_op == "<=" else "<"
        _apply_len_bound(arg_constraints[name], lower_op, _parse_known_integer(lower))
        _apply_len_bound(arg_constraints[name], upper_op, _parse_known_integer(upper))
        return True

    match = re.fullmatch(r"(\w+)@?\.len\(\)\s*(<=|<|>=|>)\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        name, op, rhs = match.groups()
        if name not in arg_constraints or not _is_sequence_type(arg_types[name]):
            return False
        bound = _parse_known_integer(rhs)
        _apply_len_bound(arg_constraints[name], op, bound)
        return True

    match = re.fullmatch(r"(\w+)@?\.len\(\)\s*\+\s*(\d+)\s*(<=|<)\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        name, offset_text, op, rhs = match.groups()
        if name not in arg_constraints or not _is_sequence_type(arg_types[name]):
            return False
        offset = int(offset_text)
        rhs_value = _parse_known_integer(rhs)
        if op == "<":
            _apply_len_bound(arg_constraints[name], "<=", rhs_value - offset - 1)
        else:
            _apply_len_bound(arg_constraints[name], "<=", rhs_value - offset)
        return True

    match = re.fullmatch(r"-(\w+)@?\.len\(\)\s*>=\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        name, rhs = match.groups()
        if name not in arg_constraints or not _is_sequence_type(arg_types[name]):
            return False
        _apply_len_bound(arg_constraints[name], "<=", -_parse_known_integer(rhs))
        return True

    match = re.fullmatch(r"([A-Za-z0-9_:\-]+)\s*<=\s*(\w+)\s*<=\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        lower, name, upper = match.groups()
        if name not in arg_constraints or _is_sequence_type(arg_types[name]):
            return False
        _apply_value_bound(arg_constraints[name], ">=", _parse_known_integer(lower))
        _apply_value_bound(arg_constraints[name], "<=", _parse_known_integer(upper))
        return True

    match = re.fullmatch(r"([A-Za-z0-9_:\-]+)\s*(<|<=)\s*(\w+)\s*(<|<=)\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        lower, lower_op, name, upper_op, upper = match.groups()
        if name not in arg_constraints or _is_sequence_type(arg_types[name]):
            return False
        lower_op = ">=" if lower_op == "<=" else ">"
        upper_op = "<=" if upper_op == "<=" else "<"
        _apply_value_bound(arg_constraints[name], lower_op, _parse_known_integer(lower))
        _apply_value_bound(arg_constraints[name], upper_op, _parse_known_integer(upper))
        return True

    match = re.fullmatch(r"(\w+)\s*(<=|<|>=|>)\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        name, op, rhs = match.groups()
        if name not in arg_constraints or _is_sequence_type(arg_types[name]):
            return False
        _apply_value_bound(arg_constraints[name], op, _parse_known_integer(rhs))
        return True

    match = re.fullmatch(r"(\w+)\s*\+\s*(\d+)\s*(<=|<)\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        name, offset_text, op, rhs = match.groups()
        if name not in arg_constraints or _is_sequence_type(arg_types[name]):
            return False
        offset = int(offset_text)
        rhs_value = _parse_known_integer(rhs)
        if op == "<":
            _apply_value_bound(arg_constraints[name], "<=", rhs_value - offset - 1)
        else:
            _apply_value_bound(arg_constraints[name], "<=", rhs_value - offset)
        return True

    match = re.fullmatch(r"(\w+)\.0\s*<=\s*\1\.1", clause)
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        arg_constraints[name].sort_tuple_non_decreasing = True
        return True

    match = re.fullmatch(
        r"forall\|i: int\|\s*0 <= i < (\w+)@?\.len\(\) as int ==> is_binary_digit\(#\[trigger\]\s*\w+\[i\]\)",
        clause,
    )
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        arg_constraints[name].allowed_chars = ["0", "1"]
        return True

    match = re.fullmatch(
        r"forall\|i: int\|\s*#!\[trigger \w+\[i\]\]\s*0 <= i < (\w+)\.len\(\) ==> 65 <= \w+\[i\] <= 90",
        clause,
    )
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        if _sequence_element_type(arg_types[name]) == "char":
            arg_constraints[name].allowed_chars = [chr(code) for code in range(65, 91)]
        else:
            arg_constraints[name].element_min_value = 65
            arg_constraints[name].element_max_value = 90
        return True

    match = re.fullmatch(
        r"forall\|i: int\|\s*#!\[trigger \w+\[i\]\]\s*0 <= i < (\w+)\.len\(\) ==> 65 <= \w+\[i\] <= 90 \|\| 97 <= \w+\[i\] <= 122",
        clause,
    )
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        alpha_values = list(range(65, 91)) + list(range(97, 123))
        if _sequence_element_type(arg_types[name]) == "char":
            arg_constraints[name].allowed_chars = [chr(code) for code in alpha_values]
        else:
            arg_constraints[name].element_allowed_values = alpha_values
        return True

    match = re.fullmatch(
        r"forall\|i: int\|\s*#!\[trigger \w+\[i\]\]\s*0 <= i < (\w+)\.len\(\) ==> ([A-Za-z0-9_:\-]+) <= \w+\[i\] <= ([A-Za-z0-9_:\-]+)",
        clause,
    )
    if match:
        name, lower, upper = match.groups()
        if name not in arg_constraints:
            return False
        arg_constraints[name].element_min_value = _parse_known_integer(lower)
        arg_constraints[name].element_max_value = _parse_known_integer(upper)
        return True

    match = re.fullmatch(
        r"forall\|i: int\|\s*0 <= i < (\w+)\.len\(\) ==> \w+\[i\] \+ 1 <= i32::MAX",
        clause,
    )
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        arg_constraints[name].element_max_value = 2_147_483_646
        return True

    match = re.fullmatch(
        r"forall\|i: int\|\s*0 <= i <= (\w+)@?\.len\(\) ==> sum\(\w+@?\.take\(i\)\.map\(\|_idx, j: i32\| j as int\)\)\s*<= i32::MAX",
        clause,
        re.DOTALL,
    )
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        arg_constraints[name].prefix_sum_max = 2_147_483_647
        return True

    match = re.fullmatch(
        r"forall\|i: int, j: int\|\s*0 <= i < j < (\w+)\.len\(\) ==> \w+\[i\] \+ \w+\[j\] <= i32::MAX && \w+\[i\] \+ \w+\[j\]\s*>= i32::MIN",
        clause,
        re.DOTALL,
    )
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        arg_constraints[name].element_min_value = -1_073_741_824
        arg_constraints[name].element_max_value = 1_073_741_823
        return True

    match = re.fullmatch(
        r"forall\|i: int, j: int\|\s*0 <= i < j < (\w+)\.len\(\) ==> \w+\[i\] != \w+\[j\]",
        clause,
        re.DOTALL,
    )
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        arg_constraints[name].distinct_elements = True
        return True

    match = re.fullmatch(r"(\w+)\s*\+\s*\(2 \* \1\)\s*<=\s*usize::MAX", clause)
    if match:
        name = match.group(1)
        if name not in arg_constraints:
            return False
        _apply_value_bound(arg_constraints[name], "<=", _parse_known_integer("usize::MAX") // 3)
        return True

    match = re.fullmatch(r"(\w+)\s*\*\s*(\w+)\s*/\s*2\s*<=\s*u64::MAX", clause)
    if match:
        left, right = match.groups()
        if left not in arg_constraints or right not in arg_constraints:
            return False
        _apply_value_bound(arg_constraints[left], ">=", 1)
        _apply_value_bound(arg_constraints[right], ">=", 1)
        _apply_value_bound(arg_constraints[left], "<=", 4_294_967_295)
        _apply_value_bound(arg_constraints[right], "<=", 4_294_967_295)
        return True

    match = re.fullmatch(r"(\w+)\s*\+\s*(\w+)\s*<=\s*([A-Za-z0-9_:\-]+)", clause)
    if match:
        left, right, rhs = match.groups()
        if left not in arg_constraints or right not in arg_constraints:
            return False
        bound = _parse_known_integer(rhs) // 2
        _apply_value_bound(arg_constraints[left], "<=", bound)
        _apply_value_bound(arg_constraints[right], "<=", bound)
        return True

    return False


def _is_sequence_type(rust_type: str) -> bool:
    """Check whether a normalized Rust type is vector-like for length constraints."""
    bare_type = _strip_ref(rust_type)
    return bare_type.startswith("Vec<") or (bare_type.startswith("[") and ";" not in bare_type) or rust_type == "&str"


def _sequence_element_type(rust_type: str) -> str | None:
    """Return the normalized element type for vector-like Rust types when available."""
    rust_type = _normalize_type(rust_type)
    bare_type = _strip_ref(rust_type)
    if bare_type.startswith("Vec<") and bare_type.endswith(">"):
        return _normalize_type(bare_type[len("Vec<") : -1].strip())
    if bare_type.startswith("[") and bare_type.endswith("]") and ";" not in bare_type:
        return _normalize_type(bare_type[1:-1].strip())
    return None


def _apply_len_bound(constraints: ArgConstraints, op: str, bound: int) -> None:
    """Merge a parsed length bound into one argument's constraints."""
    if op == ">":
        constraints.min_len = max(constraints.min_len, bound + 1)
    elif op == ">=":
        constraints.min_len = max(constraints.min_len, bound)
    elif op == "<":
        max_len = bound - 1
        constraints.max_len = max_len if constraints.max_len is None else min(constraints.max_len, max_len)
    elif op == "<=":
        constraints.max_len = bound if constraints.max_len is None else min(constraints.max_len, bound)
    else:
        raise OracleTestGenerationError(f"unsupported length operator `{op}`")


def _apply_value_bound(constraints: ArgConstraints, op: str, bound: int) -> None:
    """Merge a parsed scalar bound into one argument's constraints."""
    if op == ">":
        min_value = bound + 1
        constraints.min_value = min_value if constraints.min_value is None else max(constraints.min_value, min_value)
    elif op == ">=":
        constraints.min_value = bound if constraints.min_value is None else max(constraints.min_value, bound)
    elif op == "<":
        max_value = bound - 1
        constraints.max_value = max_value if constraints.max_value is None else min(constraints.max_value, max_value)
    elif op == "<=":
        constraints.max_value = bound if constraints.max_value is None else min(constraints.max_value, bound)
    else:
        raise OracleTestGenerationError(f"unsupported value operator `{op}`")


def _parse_known_integer(expr: str) -> int:
    """Parse the integer literals and primitive-type bounds that appear in supported `requires` clauses."""
    expr = expr.strip()
    known_constants = {
        "u8::MAX": 255,
        "u32::MAX": 4_294_967_295,
        "u64::MAX": 18_446_744_073_709_551_615,
        "usize::MAX": 18_446_744_073_709_551_615,
        "i32::MAX": 2_147_483_647,
        "i32::MIN": -2_147_483_648,
    }
    if expr in known_constants:
        return known_constants[expr]
    if re.fullmatch(r"-?\d+", expr):
        return int(expr)
    raise OracleTestGenerationError(f"unsupported integer bound in requires clause: {expr}")


def _rust_numeric_literal(value: int, rust_type: str) -> str:
    """Render a numeric literal with a Rust type suffix when that helps generated comparisons type-check."""
    rust_type = _normalize_type(rust_type)
    if rust_type in {"i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize"}:
        return f"{value}{rust_type}"
    return str(value)


def _length_sampling_expr(runner_var: str, constraints: ArgConstraints) -> str:
    """Sample a sequence length that satisfies the known bounds for one argument or group."""
    base_expr = (
        "{ "
        "use proptest::strategy::Strategy; "
        "use proptest::strategy::ValueTree; "
        "let mut len = 0usize; "
        "while proptest::arbitrary::any::<u8>().new_tree("
        f"{runner_var}"
        ").unwrap().current() % 4 != 0 { "
        "len += 1; "
        "} "
        "len "
        "}"
    )

    if constraints.min_len == 0 and constraints.max_len is None:
        return base_expr

    if constraints.max_len is None:
        return (
            "{ "
            f"let offset = {constraints.min_len}usize; "
            f"offset + {base_expr} "
            "}"
        )

    predicates = [f"candidate >= {constraints.min_len}usize"]
    if constraints.max_len is not None:
        predicates.append(f"candidate <= {constraints.max_len}usize")
    predicate = " && ".join(predicates)
    return (
        "{ "
        "loop { "
        f"let candidate = {base_expr}; "
        f"if {predicate} {{ break candidate; }} "
        "} "
        "}"
    )


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
            preserved_path = _preserve_sampler_project(tmp_path)
            raise OracleTestGenerationError(
                "Rust proptest sampler failed:\n" + (result.stdout + "\n" + result.stderr).strip(),
                preserved_sampler_path=preserved_path,
            )

        rows = []
        for line in result.stdout.splitlines():
            line = line.strip()
            if not line:
                continue
            rows.append(json.loads(line))
        return rows


def _preserve_sampler_project(tmp_path: Path) -> str:
    """Copy the generated Rust sampler project to a persistent temp directory for debugging."""
    preserved_dir = Path(
        tempfile.mkdtemp(prefix="oracle_sampler_", suffix="_cargo_project")
    )
    shutil.copytree(tmp_path, preserved_dir, dirs_exist_ok=True)
    return str(preserved_dir)


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
    if _has_unresolved_const_generic_array(rust_type):
        lines = [f"let {name} = {expr};"]
    else:
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
        inner_expr = _serializer_expr("value", _serializer_ref_type(inner))
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
        item_expr = _serializer_expr("item", _serializer_ref_type(inner))
        return (
            "{ "
            f"let parts: Vec<String> = ({expr}).iter().map(|item| {item_expr}).collect(); "
            "format!(\"vec![{}]\", parts.join(\", \")) "
            "}"
        )

    if bare_type.startswith("[") and bare_type.endswith("]") and ";" not in bare_type:
        inner = bare_type[1:-1].strip()
        item_expr = _serializer_expr("item", _serializer_ref_type(inner))
        return (
            "{ "
            f"let parts: Vec<String> = ({expr}).iter().map(|item| {item_expr}).collect(); "
            "format!(\"vec![{}]\", parts.join(\", \")) "
            "}"
        )

    if bare_type.startswith("[") and ";" in bare_type and bare_type.endswith("]"):
        inner, _ = bare_type[1:-1].split(";", 1)
        item_expr = _serializer_expr("item", _serializer_ref_type(inner.strip()))
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
        rust_format = "({})".format(placeholders)
        return f"format!(\"{rust_format}\", {', '.join(rendered)})"

    raise OracleTestGenerationError(f"unsupported oracle result type for serialization: {rust_type}")


def _serializer_ref_type(rust_type: str) -> str:
    """Add a reference for serialization unless the type is already reference-like."""
    normalized = _normalize_type(rust_type)
    if normalized.startswith("&"):
        return normalized
    return "&" + normalized


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
            preserved_path = _preserve_driver_source(source)
            raise OracleTestGenerationError(
                "verus --compile failed:\n"
                + (compile_result.stdout + "\n" + compile_result.stderr).strip(),
                preserved_driver_path=preserved_path,
            )
        if not bin_path.exists():
            preserved_path = _preserve_driver_source(source)
            raise OracleTestGenerationError(
                f"compiled oracle binary not found at {bin_path}",
                preserved_driver_path=preserved_path,
            )

        run_result = subprocess.run(
            [str(bin_path)],
            cwd=tmpdir,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        if run_result.returncode != 0:
            preserved_path = _preserve_driver_source(source)
            raise OracleTestGenerationError(
                "compiled oracle driver exited non-zero:\n"
                + (run_result.stdout + "\n" + run_result.stderr).strip(),
                preserved_driver_path=preserved_path,
            )
        return [line for line in run_result.stdout.splitlines() if line.strip()]


def _preserve_driver_source(source: str) -> str:
    """Write the generated oracle driver to a persistent temp file for debugging."""
    with tempfile.NamedTemporaryFile(
        mode="w",
        prefix="oracle_driver_",
        suffix=".rs",
        delete=False,
    ) as f:
        f.write(source)
        return f.name


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
