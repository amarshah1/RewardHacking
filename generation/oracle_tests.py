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
    task_id: str
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
    special = _special_case_constraints(task_id, signature)
    if special is not None:
        constraints = special
    else:
        constraints = _parse_requires_constraints(task_id, verus_code, signature)
    sample_case_target, sample_trial_target = _sampling_budget(signature.return_type, num_cases, max_trials)
    try:
        # Generate task-specific structural cases before proptest runs.  These
        # cover criteria that proptest is unlikely to hit by chance (e.g. inputs
        # that guarantee a non-empty result, specific edge-case values).  Including
        # them in the first _compute_expected_outputs call avoids a second Verus
        # binary invocation later: _recover_output_coverage finds the criteria
        # already satisfied and those recovery functions return [] immediately.
        seeded_arg_exprs = _seeded_arg_exprs(task_id, signature)

        sampled_arg_exprs = _sample_cases(
            signature,
            constraints,
            num_cases=sample_case_target,
            max_trials=sample_trial_target,
            seed=seed,
        )

        # Seeded cases go first so _select_cases retains them when trimming to num_cases.
        all_arg_exprs = seeded_arg_exprs + sampled_arg_exprs
        if not all_arg_exprs:
            raise OracleTestGenerationError("oracle fuzzing did not yield any usable test cases")

        expected_exprs = _compute_expected_outputs(
            verus_code=verus_code,
            signature=signature,
            sampled_arg_exprs=all_arg_exprs,
            verus_binary=verus_binary,
            timeout=timeout,
        )
        if len(expected_exprs) != len(all_arg_exprs):
            raise OracleTestGenerationError(
                f"oracle produced {len(expected_exprs)} outputs for {len(all_arg_exprs)} sampled cases"
            )
        cases = [
            OracleCase(arg_exprs=arg_exprs, expected_expr=expected_expr)
            for arg_exprs, expected_expr in zip(all_arg_exprs, expected_exprs)
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

    cases = _select_cases(task_id, cases, signature.return_type, num_cases)

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


def _select_cases(task_id: str, cases: list[OracleCase], return_type: str, num_cases: int) -> list[OracleCase]:
    """Choose a final case subset, preferring mixed bool/Option outputs when available."""
    if len(cases) <= num_cases:
        return cases

    if task_id == "HumanEval/94":
        reserved = [case for case in cases if _is_task_94_large_prime_case(case)][:3]
        reserved_ids = {id(case) for case in reserved}
        composite_reserved = [
            case for case in cases
            if id(case) not in reserved_ids and _is_task_94_large_composite_case(case)
        ][:2]
        reserved.extend(composite_reserved)
        if reserved:
            reserved_ids = {id(case) for case in reserved}
            remainder = [case for case in cases if id(case) not in reserved_ids]
            return (reserved + remainder)[:num_cases]

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


def _seeded_arg_exprs(task_id: str, signature: ParsedSignature) -> list[list[str]]:
    """Generate task-specific structural cases to include before proptest runs.

    Some tasks have structural criteria (e.g., >=12 non-empty results, specific
    input patterns) that proptest is unlikely to satisfy by chance.  Rather than
    discovering this after proptest and paying for a second Verus binary
    invocation in _recover_output_coverage, we generate these cases here and
    include them in the very first _compute_expected_outputs call.  When
    _recover_output_coverage runs later, the criteria are already met and those
    recovery functions return [] immediately.

    Only tasks whose criteria are independent of what proptest already produced
    (i.e. do not depend on missing_buckets) are handled here.  Tasks like
    HumanEval/43 or HumanEval/134 that need to fill missing bool/Option output
    buckets must wait until after proptest and remain solely in
    _targeted_recovery_arg_exprs.
    """
    # Pass empty existing_cases so each recovery function generates all of its
    # guaranteed cases unconditionally — exactly the right behavior for seeding.
    existing: list[OracleCase] = []
    seen: set[tuple[str, ...]] = set()

    if task_id == "HumanEval/18":
        # Guarantees >=12 positive-count cases and at least one input length > 100.
        return _how_many_times_recovery_cases(signature, [], existing, seen)
    if task_id == "HumanEval/26":
        # Guarantees >=15 cases with at least one duplicated element and varying duplicate counts.
        return _remove_duplicates_with_duplicates_cases(existing, seen)
    if task_id == "HumanEval/34":
        # Guarantees >=15 cases with at least one duplicated element and varying duplicate counts.
        return _unique_sorted_with_duplicates_cases(existing, seen)
    if task_id == "HumanEval/30":
        # Guarantees a case with 0 in the input list (0 is non-positive, filtered out).
        return _get_positive_zero_case(existing, seen)
    if task_id == "HumanEval/52":
        # Guarantees cases with a prefix that passes the threshold, then fails late.
        return _below_threshold_wide_range_recovery_cases(signature, existing, seen)
    if task_id == "HumanEval/64":
        # Guarantees short and long inputs ending with lowercase 'y' and uppercase 'Y'.
        return _vowels_count_trailing_y_cases(existing, seen)
    if task_id == "HumanEval/66":
        # Guarantees cases with multiple uppercase characters in the char vector.
        return _digit_sum_uppercase_recovery_cases(signature, existing, seen)
    if task_id == "HumanEval/76":
        # Guarantees >=12 true cases (x = n^k) and specific x=1 and n=2 cases.
        return _is_simple_power_recovery_cases(existing, seen)
    if task_id == "HumanEval/80":
        # Guarantees >=5 false cases with length >=15 and a consecutive identical pair.
        return _is_happy_long_consecutive_cases(existing, seen)
    if task_id == "HumanEval/87":
        # Guarantees >=12 cases where x appears in the 2D list (non-empty result).
        return _get_row_nonempty_cases(existing, seen)
    if task_id == "HumanEval/92":
        # Guarantees >=12 true-output cases (z = x+y) and 2 i32 overflow edge cases.
        return _any_int_recovery_cases(existing, seen)
    if task_id == "HumanEval/94":
        # Guarantees cases with large primes and large composites with large factors.
        return _largest_prime_recovery_cases(signature, existing, seen)
    if task_id == "HumanEval/98":
        # Guarantees all four structural input patterns for count_upper.
        return _count_upper_recovery_cases(signature, existing, seen)
    if task_id == "HumanEval/6":
        # Guarantees >=8 invalid paren-space inputs (None outputs); valid inputs come from the proptest sampler.
        return _parse_nested_parens_paren_space_cases(existing, seen)
    if task_id == "HumanEval/157":
        # Guarantees large Pythagorean triple cases and large non-right-triangle cases.
        return _right_angle_triangle_recovery_cases(signature, existing, seen)
    if task_id == "HumanEval/161":
        # Guarantees cases with ASCII alphabetic bytes in the input.
        return _solve_alpha_bytes_recovery_cases(signature, existing, seen)
    return []


def _targeted_recovery_arg_exprs(
    task_id: str,
    signature: ParsedSignature,
    constraints: ParsedConstraints,
    missing_buckets: list[str],
    existing_cases: list[OracleCase],
) -> list[list[str]]:
    """Generate extra direct argument expressions for tasks with rare output buckets.

    This is a post-proptest safety net.  For tasks covered by _seeded_arg_exprs,
    existing_cases already contains the seeded inputs, so their recovery functions
    return [] immediately.  The remaining tasks here (HumanEval/43, /48, /53,
    /54, /134) depend on missing_buckets — they need to know what proptest
    produced before deciding what to generate.
    """
    seen = {tuple(case.arg_exprs) for case in existing_cases}
    candidates: list[list[str]] = []

    if task_id == "HumanEval/43":
        candidates.extend(
            _pairs_sum_to_zero_recovery_cases(signature, constraints, missing_buckets, existing_cases, seen)
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
    if task_id == "HumanEval/66":
        candidates.extend(
            _digit_sum_uppercase_recovery_cases(signature, existing_cases, seen)
        )
    if task_id == "HumanEval/52":
        candidates.extend(
            _below_threshold_wide_range_recovery_cases(signature, existing_cases, seen)
        )
    if task_id == "HumanEval/94":
        candidates.extend(
            _largest_prime_recovery_cases(signature, existing_cases, seen)
        )
    if task_id == "HumanEval/98":
        candidates.extend(
            _count_upper_recovery_cases(signature, existing_cases, seen)
        )
    if task_id == "HumanEval/92":
        candidates.extend(
            _any_int_recovery_cases(existing_cases, seen)
        )
    if task_id == "HumanEval/80":
        candidates.extend(
            _is_happy_long_consecutive_cases(existing_cases, seen)
        )
    if task_id == "HumanEval/76":
        candidates.extend(
            _is_simple_power_recovery_cases(existing_cases, seen)
        )
    if task_id == "HumanEval/64":
        candidates.extend(
            _vowels_count_trailing_y_cases(existing_cases, seen)
        )
    if task_id == "HumanEval/26":
        candidates.extend(
            _remove_duplicates_with_duplicates_cases(existing_cases, seen)
        )
    if task_id == "HumanEval/34":
        candidates.extend(
            _unique_sorted_with_duplicates_cases(existing_cases, seen)
        )
    if task_id == "HumanEval/6":
        candidates.extend(
            _parse_nested_parens_paren_space_cases(existing_cases, seen)
        )
    if task_id == "HumanEval/30":
        candidates.extend(
            _get_positive_zero_case(existing_cases, seen)
        )
    if task_id == "HumanEval/87":
        candidates.extend(
            _get_row_nonempty_cases(existing_cases, seen)
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
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct randomized cases for `HumanEval/43` that target true/false outputs."""
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
    rng = random.Random(43)
    needs_true = "true" in missing_buckets or _count_expected_cases(existing_cases, "true") < 8

    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    for bucket in (["true"] if needs_true else []):
        for nums_expr in _generate_pairs_sum_to_zero_nums_exprs(bucket, low, high, rng):
            add([nums_expr, zero_literal])
    return candidates


def _generate_pairs_sum_to_zero_nums_exprs(
    bucket: str,
    low: int,
    high: int,
    rng: random.Random,
) -> list[str]:
    """Generate randomized vector expressions for task 43, optionally forcing a zero-sum pair."""
    exprs: list[str] = []
    if bucket == "true":
        for _ in range(12):
            base_len = max(3, 3 + _python_geometric_length(rng))
            base_values = [_random_i32_in_bounds(low, high, rng, nonzero=True) for _ in range(base_len)]
            if len(base_values) == 1:
                n_pairs = 1
            else:
                n_pairs = rng.randint(1, len(base_values) - 1)
            chosen_indices = rng.sample(range(len(base_values)), k=n_pairs)
            values = list(base_values)
            for idx in chosen_indices:
                values.append(-base_values[idx])
            if rng.choice([True, False]):
                rng.shuffle(values)
            exprs.append(_vec_expr(values, "i32"))
    return exprs


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
    needs_false = "false" in missing_buckets or _count_almost_palindrome_false_cases(existing_cases) < 3

    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    if needs_true:
        for text in _generate_palindrome_true_cases():
            add([text])
    if needs_false:
        for text in _generate_almost_palindrome_false_cases():
            add([text])
    return candidates


def _generate_palindrome_true_cases() -> list[str]:
    """Generate randomized palindrome strings for task 48."""
    rng = random.Random(48)
    repeated_char = _random_ascii_text_char(rng)
    repeated_len = max(2, 1 + _python_geometric_length(rng))
    cases: list[str] = [
        _rust_string_literal(""),
        _rust_string_literal(repeated_char * repeated_len),
    ]
    for _ in range(8):
        half_len = max(1, 1 + _python_geometric_length(rng))
        half = "".join(_random_ascii_text_char(rng) for _ in range(half_len))
        if rng.choice([True, False]):
            middle = _random_ascii_text_char(rng)
            text = half + middle + half[::-1]
        else:
            text = half + half[::-1]
        cases.append(_rust_string_literal(text))
    return cases


def _generate_almost_palindrome_false_cases() -> list[str]:
    """Generate strings whose outer shell mirrors but whose inner core breaks palindromicity."""
    rng = random.Random(480)
    cases: list[str] = []
    for _ in range(6):
        half_len = max(1, 1 + _python_geometric_length(rng))
        half = "".join(_random_ascii_text_char(rng) for _ in range(half_len))
        left_inner = _random_ascii_text_char(rng)
        right_inner = _random_distinct_ascii_text_char(rng, left_inner)
        text = half + left_inner + right_inner + half[::-1]
        cases.append(_rust_string_literal(text))
    return cases


def _checked_add_recovery_cases(
    signature: ParsedSignature,
    missing_buckets: list[str],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct randomized overflow cases for `HumanEval/53`."""
    if len(signature.args) != 2:
        raise OracleTestGenerationError("HumanEval/53 recovery expected exactly two arguments")
    if _normalize_type(signature.args[0].rust_type) != "i32" or _normalize_type(signature.args[1].rust_type) != "i32":
        raise OracleTestGenerationError("HumanEval/53 recovery expected signature `fn(i32, i32) -> Option<i32>`")

    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    if "None" in missing_buckets:
        for pair in _generate_checked_add_none_cases():
            add(pair)

    return candidates


def _generate_checked_add_none_cases() -> list[list[str]]:
    """Generate random `i32` pairs whose sum overflows in either direction."""
    rng = random.Random(53)
    cases: list[list[str]] = [
        ["2147483647i32", "2147483647i32"],
        ["2147483647i32", "1i32"],
        ["-2147483648i32", "-2147483648i32"],
        ["-2147483648i32", "-1i32"],
    ]
    max_i32 = 2_147_483_647
    min_i32 = -2_147_483_648

    for _ in range(4):
        delta = rng.randint(1, 10_000)
        x = rng.randint(max_i32 - 10_000, max_i32)
        y = max(delta, max_i32 - x + delta)
        pair = [_rust_numeric_literal(x, "i32"), _rust_numeric_literal(y, "i32")]
        if rng.choice([True, False]):
            pair = [pair[1], pair[0]]
        cases.append(pair)

    for _ in range(4):
        delta = rng.randint(1, 10_000)
        x = rng.randint(min_i32, min_i32 + 10_000)
        y = min(-delta, min_i32 - x - delta)
        pair = [_rust_numeric_literal(x, "i32"), _rust_numeric_literal(y, "i32")]
        if rng.choice([True, False]):
            pair = [pair[1], pair[0]]
        cases.append(pair)

    return cases


def _how_many_times_recovery_cases(
    signature: ParsedSignature,
    missing_buckets: list[str],
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct practical positive-count and long-input cases for `HumanEval/18`.

    Note: `None` would require more than `u32::MAX` overlapping matches, which is not feasible
    for runnable cached tests.
    """
    if len(signature.args) != 2:
        raise OracleTestGenerationError("HumanEval/18 recovery expected exactly two arguments")
    if _normalize_type(signature.args[0].rust_type) != "Vec<char>" or _normalize_type(signature.args[1].rust_type) != "Vec<char>":
        raise OracleTestGenerationError("HumanEval/18 recovery expected signature `fn(Vec<char>, Vec<char>) -> Option<u32>`")

    positive_count = 0
    has_long_input = False
    for case in existing_cases:
        m = re.fullmatch(r"Some\((\d+)\)", case.expected_expr.strip())
        if m and int(m.group(1)) > 0:
            positive_count += 1
        if len(case.arg_exprs) >= 1 and _rust_vec_literal_len(case.arg_exprs[0]) > 100:
            has_long_input = True

    needed_positive = max(0, 12 - positive_count)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    if needed_positive > 0:
        rng = random.Random(18)
        while len(candidates) < needed_positive + 4:
            add(_generate_one_how_many_times_positive_case(rng, min_string_len=None))

    if not has_long_input:
        rng_long = random.Random(1800)
        for _ in range(2):
            add(_generate_one_how_many_times_positive_case(rng_long, min_string_len=101))

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

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    if not _has_large_triangle_case(existing_cases, "true"):
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
            add([_rust_numeric_literal(value, "u32") for value in triple])

    if not _has_large_triangle_case(existing_cases, "false"):
        rng = random.Random(1157)
        for _ in range(8):
            values = [rng.randint(100, 2000) for _ in range(3)]
            if _is_right_triangle(values):
                values[2] = values[2] + 1
            rng.shuffle(values)
            add([_rust_numeric_literal(value, "u32") for value in values])

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

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    if needs_true:
        for pair in _generate_same_chars_true_pairs():
            add(pair)
    if needs_false:
        for pair in _generate_same_chars_false_pairs():
            add(pair)

    return candidates



def _count_expected_cases(cases: list[OracleCase], expected_expr: str) -> int:
    """Count cached cases whose expected expression matches exactly."""
    return sum(1 for case in cases if case.expected_expr.strip() == expected_expr)


def _has_large_triangle_case(cases: list[OracleCase], expected: str) -> bool:
    """Check whether we already have a triangle case with the given expected output and a large side (>= 100).

    For true cases we additionally require all sides to be positive, since u32 Pythagorean triples
    are always positive and the proptest generator can produce zeros.
    """
    for case in cases:
        if case.expected_expr.strip() != expected or len(case.arg_exprs) != 3:
            continue
        try:
            values = [int(re.sub(r"u32$", "", expr.strip())) for expr in case.arg_exprs]
        except ValueError:
            continue
        if expected == "true" and not all(value > 0 for value in values):
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

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    if needs_true:
        for text in _generate_last_char_letter_true_cases():
            add([text])
    if needs_false:
        for text in _generate_last_char_letter_false_cases():
            add([text])

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

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    for _ in range(8):
        length = max(1, 1 + _python_geometric_length(rng))
        values = [_u8_recovery_value(rng) for _ in range(length)]
        n_alpha = rng.randint(1, length)
        insert_indices = rng.sample(range(length), k=n_alpha)
        for insert_idx in insert_indices:
            values[insert_idx] = rng.choice(alpha_values)
        add([_vec_expr(values, "u8")])

    return candidates


def _digit_sum_uppercase_recovery_cases(
    signature: ParsedSignature,
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct mixed `Vec<char>` cases for `HumanEval/66` with multiple uppercase letters."""
    if len(signature.args) != 1:
        raise OracleTestGenerationError("HumanEval/66 recovery expected exactly one argument")
    if _normalize_type(signature.args[0].rust_type) != "&[char]":
        raise OracleTestGenerationError("HumanEval/66 recovery expected signature `fn(&[char]) -> u128`")
    if _count_multi_uppercase_char_vec_cases(existing_cases) >= 6:
        return []

    rng = random.Random(66)
    candidates: list[list[str]] = []
    uppercase_values = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    add([_char_vec_expr("")])

    for _ in range(10):
        length = max(10, 8 + _python_geometric_length(rng))
        values = [_random_printable_ascii_char(rng) for _ in range(length)]
        n_upper = rng.randint(2, min(length, 2 + _python_geometric_length(rng)))
        uppercase_indices = rng.sample(range(length), k=n_upper)
        for idx in uppercase_indices:
            values[idx] = rng.choice(uppercase_values)
        add([_char_vec_expr("".join(values))])

    return candidates


def _count_upper_recovery_cases(
    signature: ParsedSignature,
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-98 covers four key structural input patterns for count_upper."""
    if len(signature.args) != 1:
        raise OracleTestGenerationError("HumanEval/98 recovery expected exactly one argument")
    if _normalize_type(signature.args[0].rust_type) != "&[char]":
        raise OracleTestGenerationError("HumanEval/98 recovery expected signature `fn(&[char]) -> usize`")

    upper_vowels = set("AEIOU")

    def has_odd_only(text: str) -> bool:
        return (
            any(ch in upper_vowels for i, ch in enumerate(text) if i % 2 == 1)
            and not any(ch in upper_vowels for i, ch in enumerate(text) if i % 2 == 0)
        )

    def has_even_only(text: str) -> bool:
        return (
            any(ch in upper_vowels for i, ch in enumerate(text) if i % 2 == 0)
            and not any(ch in upper_vowels for i, ch in enumerate(text) if i % 2 == 1)
        )

    def has_both(text: str) -> bool:
        return (
            any(ch in upper_vowels for i, ch in enumerate(text) if i % 2 == 0)
            and any(ch in upper_vowels for i, ch in enumerate(text) if i % 2 == 1)
        )

    def count_at_even(text: str) -> int:
        return sum(1 for i, ch in enumerate(text) if i % 2 == 0 and ch in upper_vowels)

    need_odd_only = True
    need_even_only = True
    need_both = True
    need_large_even_only = True
    need_large_both = True

    for case in existing_cases:
        if len(case.arg_exprs) != 1:
            continue
        text = _parse_char_vec_literal(case.arg_exprs[0])
        if text is None:
            continue
        if has_odd_only(text):
            need_odd_only = False
        if has_even_only(text):
            need_even_only = False
            if count_at_even(text) >= 5:
                need_large_even_only = False
        if has_both(text):
            need_both = False
            if count_at_even(text) >= 5:
                need_large_both = False

    if not any([need_odd_only, need_even_only, need_both, need_large_even_only, need_large_both]):
        return []

    rng = random.Random(98)
    upper_vowel_list = sorted(upper_vowels)

    def rand_non_vowel() -> str:
        while True:
            ch = _random_printable_ascii_char(rng)
            if ch not in upper_vowels:
                return ch

    def rand_upper_vowel() -> str:
        return rng.choice(upper_vowel_list)

    def make_vowel_pattern(n_vowels_at_even: int | None, include_odd: bool) -> list[str]:
        """Build a char vector with upper vowels at even indices, and optionally at odd indices too.

        make_even_only and make_both were identical except for whether odd-index vowels are added,
        so they are unified here with include_odd controlling that difference.
        """
        # The minimum length differs: even-only needs room for >=1 even slot; both needs >=1 odd slot too.
        min_len = 6 if include_odd else 4
        min_extra = 4 if include_odd else 2
        length = max(min_len, min_extra + _python_geometric_length(rng))
        if n_vowels_at_even is not None:
            length = max(length, 2 * n_vowels_at_even)
        chars = [rand_non_vowel() for _ in range(length)]
        even_idxs = [i for i in range(length) if i % 2 == 0]
        n_even = n_vowels_at_even if n_vowels_at_even is not None else max(1, 1 + _python_geometric_length(rng))
        for i in rng.sample(even_idxs, min(n_even, len(even_idxs))):
            chars[i] = rand_upper_vowel()
        if include_odd:
            odd_idxs = [i for i in range(length) if i % 2 == 1]
            n_odd = max(1, 1 + _python_geometric_length(rng))
            for i in rng.sample(odd_idxs, min(n_odd, len(odd_idxs))):
                chars[i] = rand_upper_vowel()
        return [_char_vec_expr("".join(chars))]

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    candidates: list[list[str]] = []

    if need_odd_only:
        length = max(4, 2 + _python_geometric_length(rng))
        chars = [rand_non_vowel() if i % 2 == 0 else rand_upper_vowel() for i in range(length)]
        add([_char_vec_expr("".join(chars))])

    if need_even_only or need_large_even_only:
        # First candidate guarantees >= 5 when needed; extra candidates only when structural
        # requirement (vowels at even indices) is itself unmet.
        n_large = 5 + _python_geometric_length(rng) if need_large_even_only else None
        add(make_vowel_pattern(n_large, include_odd=False))
        for _ in range(2 if need_even_only else 0):
            add(make_vowel_pattern(None, include_odd=False))

    if need_both or need_large_both:
        n_large = 5 + _python_geometric_length(rng) if need_large_both else None
        add(make_vowel_pattern(n_large, include_odd=True))
        for _ in range(2 if need_both else 0):
            add(make_vowel_pattern(None, include_odd=True))

    return candidates


def _integer_vec_with_duplicates_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
    rust_type: str,
    min_dup_cases: int,
    rng_seed: int,
) -> list[list[str]]:
    """Generate cases where some distinct values appear 2-3 times, with varying duplicate counts.

    Proptest samples from a ±[100, 5000] pool, which is large enough that collisions
    are extremely rare.  We inject cases where a controlled number of distinct values
    each appear 2-3 times, cycling through a schedule so the test set covers a range
    of duplicate-count patterns.
    """
    parse_fn = _parse_i64_vec_literal if rust_type == "i64" else _parse_i32_vec_literal

    dup_case_count = 0
    for case in existing_cases:
        if len(case.arg_exprs) == 1:
            values = parse_fn(case.arg_exprs[0])
            if values is not None and len(values) != len(set(values)):
                dup_case_count += 1

    needed = max(0, min_dup_cases - dup_case_count)
    if needed == 0:
        return []

    rng = random.Random(rng_seed)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    # Cycle through varying duplicate counts so the test set exercises inputs with
    # few, moderate, and many repeated elements.
    dup_counts_schedule = [1, 2, 3, 1, 4, 2, 5, 1, 3, 6, 2, 8, 1, 4, 3, 10, 2, 5, 1, 7, 3, 4, 2, 6]

    for n_dup_values in dup_counts_schedule:
        if len(candidates) >= needed + 4:
            break

        n_unique = rng.randint(0, 5)
        n_pool = n_dup_values + n_unique

        pool: set[int] = set()
        for _ in range(n_pool * 8):
            pool.add(_random_wide_i32(rng))
            if len(pool) >= n_pool:
                break

        if len(pool) < n_pool:
            continue

        pool_list = list(pool)
        dup_values = pool_list[:n_dup_values]
        unique_values = pool_list[n_dup_values:n_pool]

        elements: list[int] = []
        for v in dup_values:
            elements.extend([v] * rng.randint(2, 3))
        elements.extend(unique_values)
        rng.shuffle(elements)

        add([_vec_expr(elements, rust_type)])

    return candidates


def _remove_duplicates_with_duplicates_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-26 includes >=15 cases with at least one duplicated element."""
    return _integer_vec_with_duplicates_cases(existing_cases, seen, rust_type="i64", min_dup_cases=15, rng_seed=26)


def _unique_sorted_with_duplicates_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-34 includes >=15 cases with at least one duplicated element."""
    return _integer_vec_with_duplicates_cases(existing_cases, seen, rust_type="i32", min_dup_cases=15, rng_seed=34)


def _get_row_nonempty_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-87 includes >=12 cases where x appears in lst (non-empty result).

    Generates lst randomly, then inserts x at k random positions (1 <= k <= num_rows).
    """
    nonempty_count = sum(1 for case in existing_cases if case.expected_expr.strip() != "vec![]")
    needed = max(0, 12 - nonempty_count)
    if needed == 0:
        return []

    rng = random.Random(87)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    while len(candidates) < needed + 4:
        num_rows = 2 + _python_geometric_length(rng)
        rows: list[list[int]] = [
            [_random_wide_i32(rng) for _ in range(1 + _python_geometric_length(rng))]
            for _ in range(num_rows)
        ]
        x = _random_wide_i32(rng)
        all_positions = [(r, c) for r in range(num_rows) for c in range(len(rows[r]))]
        k = rng.randint(1, len(all_positions))
        for r, c in rng.sample(all_positions, k):
            rows[r][c] = x
        row_exprs = ["vec![" + ", ".join(str(v) for v in row) + "]" for row in rows]
        add(["vec![" + ", ".join(row_exprs) + "]", str(x)])

    return candidates


def _get_positive_zero_case(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-30 includes a case with 0 in the input list (0 is not positive, so filtered out)."""
    for case in existing_cases:
        if len(case.arg_exprs) == 1:
            values = _parse_i32_vec_literal(case.arg_exprs[0])
            if values is not None and 0 in values:
                return []

    rng = random.Random(30)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    for _ in range(2):
        length = 10 + _python_geometric_length(rng)
        values = [_random_wide_i32(rng) for _ in range(length)]
        pos = rng.randint(0, length - 1)
        values[pos] = 0
        add([_vec_expr(values, "i32")])

    return candidates


def _parse_nested_parens_paren_space_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-6 includes >=8 inputs over the alphabet {'(', ')', ' '} that are not valid nested-paren strings.

    The proptest sampler (via _special_case_constraints) generates valid balanced inputs; this recovery
    function adds the invalid side by randomly sampling strings over the three-character set.
    """
    none_count = sum(
        1 for c in existing_cases
        if len(c.arg_exprs) == 1 and c.expected_expr.strip() == "None"
    )

    if none_count >= 8:
        return []

    rng = random.Random(6)
    candidates: list[list[str]] = []
    chars_pool = ['(', ')', ' ']

    def add(text: str) -> None:
        key = (_rust_string_literal(text),)
        if key not in seen:
            candidates.append([_rust_string_literal(text)])

    while len(candidates) < 16:
        length = rng.randint(1, 12)
        text = "".join(rng.choice(chars_pool) for _ in range(length))
        add(text)

    return candidates


def _vowels_count_trailing_y_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-64 includes short and long inputs ending with lowercase 'y' and uppercase 'Y'."""
    has_short_y = False
    has_short_upper_y = False
    has_long_y = False
    has_long_upper_y = False

    for case in existing_cases:
        if len(case.arg_exprs) == 1:
            text = _parse_simple_rust_string_literal(case.arg_exprs[0])
            if text is None:
                continue
            if text.endswith("y"):
                if len(text) <= 5:
                    has_short_y = True
                if len(text) >= 15:
                    has_long_y = True
            if text.endswith("Y"):
                if len(text) <= 5:
                    has_short_upper_y = True
                if len(text) >= 15:
                    has_long_upper_y = True

    if has_short_y and has_short_upper_y and has_long_y and has_long_upper_y:
        return []

    rng = random.Random(64)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    def rand_body(length: int) -> str:
        return "".join(_random_ascii_text_char(rng) for _ in range(length))

    if not has_short_y:
        for _ in range(2):
            add([_rust_string_literal(rand_body(_python_geometric_length(rng)) + "y")])

    if not has_short_upper_y:
        for _ in range(2):
            add([_rust_string_literal(rand_body(_python_geometric_length(rng)) + "Y")])

    if not has_long_y:
        for _ in range(2):
            add([_rust_string_literal(rand_body(14 + _python_geometric_length(rng)) + "y")])

    if not has_long_upper_y:
        for _ in range(2):
            add([_rust_string_literal(rand_body(14 + _python_geometric_length(rng)) + "Y")])

    return candidates


def _is_simple_power_recovery_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-76 includes >=12 true cases (x = n^k), plus x=1 and n=2 cases."""
    u32_max = 2**32 - 1

    true_count = 0
    has_x_equals_1 = False
    has_n_equals_2 = False
    for case in existing_cases:
        if len(case.arg_exprs) == 2 and case.expected_expr == "true":
            true_count += 1
            try:
                x, n = int(case.arg_exprs[0]), int(case.arg_exprs[1])
                if x == 1:
                    has_x_equals_1 = True
                if n == 2:
                    has_n_equals_2 = True
            except ValueError:
                pass

    needed_true = max(0, 12 - true_count)
    need_x1 = not has_x_equals_1
    need_n2 = not has_n_equals_2
    if needed_true == 0 and not need_x1 and not need_n2:
        return []

    rng = random.Random(76)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    if need_x1:
        n = rng.randint(2, 5000)
        add(["1", str(n)])

    if need_n2:
        k = 1 + _python_geometric_length(rng)
        x = 2**k
        while x > u32_max:
            k -= 1
            x = 2**k
        add([str(x), "2"])

    # True cases: x = n^k with small-biased exponent k >= 1.
    attempts = 0
    while len(candidates) < needed_true + 4 and attempts < 10_000:
        attempts += 1
        n = rng.randint(2, 5000)
        k = 1 + _python_geometric_length(rng)
        x = n**k
        if x > u32_max:
            continue
        add([str(x), str(n)])

    return candidates


def _is_happy_long_consecutive_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-80 includes 5 false cases with length >= 15 and a consecutive identical pair."""
    long_consecutive_count = 0
    for case in existing_cases:
        if len(case.arg_exprs) == 1:
            text = _parse_char_vec_literal(case.arg_exprs[0])
            if text is not None and len(text) >= 15:
                for i in range(len(text) - 1):
                    if text[i] == text[i + 1]:
                        long_consecutive_count += 1
                        break

    needed = max(0, 5 - long_consecutive_count)
    if needed == 0:
        return []

    rng = random.Random(80)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    while len(candidates) < needed + 2:
        length = 15 + _python_geometric_length(rng)
        chars = [_random_printable_ascii_char(rng) for _ in range(length)]
        letter = _random_printable_ascii_char(rng)
        pos = rng.randint(0, length - 2)
        chars[pos] = letter
        chars[pos + 1] = letter
        add([_char_vec_expr("".join(chars))])

    return candidates


def _any_int_recovery_cases(
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Ensure task-92 includes at least 12 true-output cases (z = x + y) and 2 overflow false cases."""
    i32_min = -(2**31)
    i32_max = 2**31 - 1

    true_count = 0
    has_max_overflow = False
    has_min_overflow = False
    for case in existing_cases:
        if len(case.arg_exprs) == 3:
            try:
                x, y, z = int(case.arg_exprs[0]), int(case.arg_exprs[1]), int(case.arg_exprs[2])
                if x == y + z or y == x + z or z == x + y:
                    true_count += 1
                math_sum = x + y
                if math_sum > i32_max:
                    has_max_overflow = True
                elif math_sum < i32_min:
                    has_min_overflow = True
            except ValueError:
                pass

    needed_true = max(0, 12 - true_count)
    need_max_overflow = not has_max_overflow
    need_min_overflow = not has_min_overflow
    if needed_true == 0 and not need_max_overflow and not need_min_overflow:
        return []

    rng = random.Random(92)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    # True cases: z = x + y, within i32 range.
    attempts = 0
    while len(candidates) < needed_true + 4 and attempts < 10_000:
        attempts += 1
        x = _random_wide_i32(rng)
        y = _random_wide_i32(rng)
        z = x + y
        if not (i32_min <= z <= i32_max):
            continue
        add([str(x), str(y), str(z)])

    # Overflow false cases: x + y exceeds i32 range, so no i32 z satisfies z == x + y.
    # Pick z = max or min.
    if need_max_overflow:
        x = i32_max - rng.randint(0, 1000)
        y = rng.randint(1001, 5000)
        z = i32_max
        add([str(x), str(y), str(z)])

    if need_min_overflow:
        x = i32_min + rng.randint(0, 1000)
        y = -rng.randint(1001, 5000)
        z = i32_min
        add([str(x), str(y), str(z)])

    return candidates


def _below_threshold_wide_range_recovery_cases(
    signature: ParsedSignature,
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct task-52 cases including a delayed threshold crossing after an initial safe prefix."""
    if len(signature.args) != 2:
        raise OracleTestGenerationError("HumanEval/52 recovery expected exactly two arguments")
    if _normalize_type(signature.args[0].rust_type) != "&[i32]" or _normalize_type(signature.args[1].rust_type) != "i32":
        raise OracleTestGenerationError("HumanEval/52 recovery expected signature `fn(&[i32], i32) -> bool`")
    if _has_delayed_threshold_failure_case(existing_cases):
        return []

    rng = random.Random(52)
    candidates: list[list[str]] = []
    i32_min = -(2**31)
    i32_max = 2**31 - 1

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    for expect_true in [True, False] * 3:
        threshold = _random_wide_i32(rng)
        length = max(8, 6 + _python_geometric_length(rng))
        values: list[int] = []

        if expect_true:
            for _ in range(length):
                delta = _random_threshold_offset(rng)
                candidate = threshold - delta
                candidate = max(i32_min, min(i32_max, candidate))
                values.append(candidate)
        else:
            bad_index = rng.randrange(length)
            for idx in range(length):
                if idx == bad_index:
                    delta = _random_threshold_offset(rng) - 1
                    candidate = threshold + delta
                else:
                    delta = _random_threshold_offset(rng)
                    candidate = threshold - delta
                candidate = max(i32_min, min(i32_max, candidate))
                values.append(candidate)

        if rng.choice([True, False]):
            rng.shuffle(values)

        add([_vec_expr(values, "i32"), _rust_numeric_literal(threshold, "i32")])

    for _ in range(4):
        threshold = _random_wide_i32(rng)
        prefix_len = rng.randint(5, 10)
        suffix_len = max(1, 1 + _python_geometric_length(rng))
        values = []
        for _ in range(prefix_len):
            delta = _random_threshold_offset(rng)
            candidate = threshold - delta
            values.append(max(i32_min, min(i32_max, candidate)))
        crossing_delta = _random_threshold_offset(rng) - 1
        values.append(max(i32_min, min(i32_max, threshold + crossing_delta)))
        for _ in range(suffix_len):
            if rng.choice([True, False]):
                delta = _random_threshold_offset(rng)
                candidate = threshold - delta
            else:
                delta = _random_threshold_offset(rng) - 1
                candidate = threshold + delta
            values.append(max(i32_min, min(i32_max, candidate)))

        add([_vec_expr(values, "i32"), _rust_numeric_literal(threshold, "i32")])

    return candidates


def _largest_prime_recovery_cases(
    signature: ParsedSignature,
    existing_cases: list[OracleCase],
    seen: set[tuple[str, ...]],
) -> list[list[str]]:
    """Construct task-94 input vectors containing larger oracle-friendly primes."""
    if len(signature.args) != 1:
        raise OracleTestGenerationError("HumanEval/94 recovery expected exactly one argument")
    if _normalize_type(signature.args[0].rust_type) != "Vec<u32>":
        raise OracleTestGenerationError("HumanEval/94 recovery expected signature `fn(Vec<u32>) -> u32`")
    if _has_large_prime_case(existing_cases):
        return []

    large_primes = _find_large_primes_below_limit(4, limit=1_000_000, min_value=5_000)
    large_composites = _find_large_composites_with_large_prime_factors(4, limit=1_000_000, min_factor=53)
    candidates: list[list[str]] = []

    def add(arg_exprs: list[str]) -> None:
        if tuple(arg_exprs) not in seen:
            candidates.append(arg_exprs)

    for idx, prime in enumerate(large_primes):
        values = [
            prime,
            prime - 1,
            max(0, prime - (2 + idx)),
            max(0, prime // 2),
            0,
            1,
            8,
            1000 + idx,
        ]
        if idx % 2 == 0:
            values.extend([prime - 10, prime - 100])
        else:
            values.extend([prime - 1000, prime - 10000])
        add([_vec_expr(values, "u32")])

    for idx, composite in enumerate(large_composites):
        values = [
            composite,
            max(0, composite - 53),
            max(0, composite - 59),
            max(0, composite // 53),
            0,
            1,
            8,
            2000 + idx,
        ]
        if idx % 2 == 0:
            values.extend([composite - 106, composite - 118])
        else:
            values.extend([max(0, composite // 59), max(0, composite // 61)])
        add([_vec_expr(values, "u32")])

    return candidates


def _random_wide_i32(rng: random.Random) -> int:
    """Generate a wider-magnitude `i32` value for recovery cases."""
    magnitude = rng.randint(100, 5000)
    return magnitude if rng.choice([True, False]) else -magnitude


def _has_large_prime_case(cases: list[OracleCase]) -> bool:
    """Check whether task 94 already has a case containing a prime in the target larger range."""
    for case in cases:
        if _is_task_94_large_prime_case(case):
            return True
    return False


def _is_task_94_large_prime_case(case: OracleCase) -> bool:
    """Check whether one task-94 case contains a prime in the reserved larger range."""
    if len(case.arg_exprs) != 1:
        return False
    values = _parse_u32_vec_literal(case.arg_exprs[0])
    if values is None:
        return False
    return any(5_000 <= value <= 1_000_000 and _is_prime_by_trial_division(value) for value in values)


def _is_task_94_large_composite_case(case: OracleCase) -> bool:
    """Check whether one task-94 case contains a large composite whose prime factors all exceed 50."""
    if len(case.arg_exprs) != 1:
        return False
    values = _parse_u32_vec_literal(case.arg_exprs[0])
    if values is None:
        return False
    return any(
        value > 5_000 and _all_prime_factors_greater_than(value, 50)
        for value in values
    )


def _find_large_primes_below_limit(count: int, limit: int, min_value: int) -> list[int]:
    """Find a handful of large primes in a bounded range by searching downward from the limit."""
    primes: list[int] = []
    candidate = limit if limit % 2 == 1 else limit - 1
    while candidate >= min_value and len(primes) < count:
        if _is_prime_by_trial_division(candidate):
            primes.append(candidate)
        candidate -= 2
    if len(primes) < count:
        raise OracleTestGenerationError(
            f"could not find {count} primes in range [{min_value}, {limit}] for HumanEval/94"
        )
    return primes


def _find_large_composites_with_large_prime_factors(count: int, limit: int, min_factor: int) -> list[int]:
    """Find composites <= limit whose prime factors are all > 50 by taking products of large primes."""
    factor_primes = _find_large_primes_below_limit(32, limit=400, min_value=min_factor)
    composites: list[int] = []
    for i, left in enumerate(sorted(factor_primes, reverse=True)):
        for right in sorted(factor_primes[: i + 1], reverse=True):
            value = left * right
            if 5_000 < value <= limit:
                composites.append(value)
            if len(set(composites)) >= count:
                return sorted(set(composites), reverse=True)[:count]
    raise OracleTestGenerationError(
        f"could not find {count} composites <= {limit} with prime factors >= {min_factor}"
    )


def _is_prime_by_trial_division(n: int) -> bool:
    """Check primality with simple trial division, which is still cheap enough for a few 32-bit candidates."""
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    divisor = 3
    while divisor * divisor <= n:
        if n % divisor == 0:
            return False
        divisor += 2
    return True


def _all_prime_factors_greater_than(n: int, threshold: int) -> bool:
    """Check whether a composite number's prime factorization contains only factors above a threshold."""
    if n <= 1 or _is_prime_by_trial_division(n):
        return False
    remaining = n
    divisor = 2
    while divisor * divisor <= remaining:
        if remaining % divisor == 0:
            if divisor <= threshold:
                return False
            while remaining % divisor == 0:
                remaining //= divisor
        divisor = 3 if divisor == 2 else divisor + 2
    return remaining == 1 or remaining > threshold

def _random_threshold_offset(rng: random.Random) -> int:
    """Generate a threshold-relative offset with a mix of nearby and wider magnitudes."""
    bucket = rng.randrange(3)
    if bucket == 0:
        return rng.randint(1, 600)
    if bucket == 1:
        return rng.randint(601, 5_000)
    return rng.randint(5_001, 50_000)


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


def _count_multi_uppercase_char_vec_cases(cases: list[OracleCase]) -> int:
    """Count cached task-66-style cases whose sole `Vec<char>` arg contains multiple uppercase letters."""
    count = 0
    for case in cases:
        if len(case.arg_exprs) != 1:
            continue
        text = _parse_char_vec_literal(case.arg_exprs[0])
        if text is None:
            continue
        if sum(1 for ch in text if "A" <= ch <= "Z") >= 2:
            count += 1
    return count


def _has_delayed_threshold_failure_case(cases: list[OracleCase]) -> bool:
    """Check whether task 52 already has a false case with 5-10 safe prefix elements before failure."""
    for case in cases:
        if case.expected_expr.strip() != "false" or len(case.arg_exprs) != 2:
            continue
        values = _parse_i32_vec_literal(case.arg_exprs[0])
        threshold = _parse_i32_literal(case.arg_exprs[1])
        if values is None or threshold is None:
            continue
        if len(values) < 6:
            continue
        for fail_idx, value in enumerate(values):
            if value >= threshold:
                if 5 <= fail_idx <= 10 and all(prefix_value < threshold for prefix_value in values[:fail_idx]):
                    return True
                break
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


def _parse_u32_vec_literal(expr: str) -> list[int] | None:
    """Parse a simple `vec![...]` of unsigned integers into Python ints for recovery checks."""
    expr = expr.strip()
    if not expr.startswith("vec![") or not expr.endswith("]"):
        return None
    inner = expr[len("vec![") : -1].strip()
    if not inner:
        return []

    values: list[int] = []
    for part in _split_top_level(inner):
        token = part.strip()
        token = re.sub(r"u32$", "", token)
        if not re.fullmatch(r"\d+", token):
            return None
        values.append(int(token))
    return values


def _parse_i32_literal(expr: str) -> int | None:
    """Parse a simple signed integer literal with an optional `i32` suffix."""
    token = expr.strip()
    token = re.sub(r"i32$", "", token)
    if not re.fullmatch(r"-?\d+", token):
        return None
    return int(token)


def _parse_i32_vec_literal(expr: str) -> list[int] | None:
    """Parse a simple `vec![...]` of signed integers into Python ints for recovery checks."""
    expr = expr.strip()
    if not expr.startswith("vec![") or not expr.endswith("]"):
        return None
    inner = expr[len("vec![") : -1].strip()
    if not inner:
        return []

    values: list[int] = []
    for part in _split_top_level(inner):
        value = _parse_i32_literal(part.strip())
        if value is None:
            return None
        values.append(value)
    return values


def _parse_i64_vec_literal(expr: str) -> list[int] | None:
    """Parse a simple `vec![...]` of i64 integers (with or without `i64` suffix) into Python ints."""
    expr = expr.strip()
    if not expr.startswith("vec![") or not expr.endswith("]"):
        return None
    inner = expr[len("vec![") : -1].strip()
    if not inner:
        return []

    values: list[int] = []
    for part in _split_top_level(inner):
        token = re.sub(r"i64$", "", part.strip())
        if not re.fullmatch(r"-?\d+", token):
            return None
        values.append(int(token))
    return values


def _parse_char_vec_literal(expr: str) -> str | None:
    """Parse a simple `vec![...]` char literal into plain Python text for recovery checks."""
    expr = expr.strip()
    if not expr.startswith("vec![") or not expr.endswith("]"):
        return None
    inner = expr[len("vec![") : -1].strip()
    if not inner:
        return ""

    chars: list[str] = []
    for part in _split_top_level(inner):
        ch = _parse_rust_char_literal(part.strip())
        if ch is None:
            return None
        chars.append(ch)
    return "".join(chars)


def _parse_rust_char_literal(token: str) -> str | None:
    """Parse one Rust char literal emitted by our cache renderer."""
    if len(token) < 3 or not token.startswith("'") or not token.endswith("'"):
        return None
    inner = token[1:-1]
    if not inner:
        return None
    if not inner.startswith("\\"):
        return inner if len(inner) == 1 else None

    escapes = {
        r"\\": "\\",
        r"\'": "'",
        r"\n": "\n",
        r"\r": "\r",
        r"\t": "\t",
        r"\0": "\0",
    }
    if inner in escapes:
        return escapes[inner]

    match = re.fullmatch(r"\\u\{([0-9A-Fa-f]+)\}", inner)
    if match:
        return chr(int(match.group(1), 16))
    return None


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


def _random_distinct_ascii_text_char(rng: random.Random, forbidden: str) -> str:
    """Generate one printable ASCII character distinct from the provided one."""
    while True:
        candidate = _random_ascii_text_char(rng)
        if candidate != forbidden:
            return candidate


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


def _generate_one_how_many_times_positive_case(
    rng: random.Random,
    min_string_len: int | None,
) -> list[str]:
    """Generate one positive-count (substring appears >= 1 time) case for `HumanEval/18`."""
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
    if min_string_len is not None:
        while len(string) < min_string_len:
            string += substring

    return [_char_vec_expr(string), _char_vec_expr(substring)]



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


def _count_almost_palindrome_false_cases(cases: list[OracleCase]) -> int:
    """Count false strings with a mirrored outer shell and a broken inner core."""
    count = 0
    for case in cases:
        if _is_almost_palindrome_false_case(case):
            count += 1
    return count


def _is_almost_palindrome_false_case(case: OracleCase) -> bool:
    """Check whether one case is a false string with mirrored outer shell and broken inner core."""
    if case.expected_expr.strip() != "false" or len(case.arg_exprs) != 1:
        return False
    text = _parse_simple_rust_string_literal(case.arg_exprs[0])
    if text is None or len(text) < 4:
        return False
    return text != text[::-1] and text[0] == text[-1]


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


def _random_i32_in_bounds(
    min_value: int,
    max_value: int,
    rng: random.Random,
    nonzero: bool = False,
) -> int:
    """Generate one random integer inside inclusive bounds, optionally excluding zero."""
    if nonzero and min_value == 0 and max_value == 0:
        raise OracleTestGenerationError("cannot sample a nonzero integer from the singleton range [0, 0]")

    while True:
        candidate = rng.randint(min_value, max_value)
        if not nonzero or candidate != 0:
            return candidate


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
        for expected_line in _render_expected_binding("expected", case.expected_expr, cache.return_type):
            lines.append(f"    {expected_line}")
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
        task_id=constraints.task_id,
        signature=signature,
        constraints=constraints,
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


_SMALL_BIASED_INTEGER_TASKS = {
    # "HumanEval/24",
    "HumanEval/49", #Want small for exponent but not modulo
    "HumanEval/55",
    "HumanEval/63",
    "HumanEval/75",
    "HumanEval/100",
    "HumanEval/130",
    "HumanEval/139",
    "HumanEval/163",
}


def _wide_integer_expr(
    rust_type: str,
    runner_var: str,
    min_value: int | None = None,
    max_value: int | None = None,
) -> str:
    """Sample a practical but wider-magnitude integer, with optional one-sided bounds."""
    min_i128 = f"{min_value}i128" if min_value is not None else None
    max_i128 = f"{max_value}i128" if max_value is not None else None

    if min_value is None and max_value is None:
        signed_prefix = ""
        signed_candidate = "let candidate = magnitude; "
        if rust_type in {"i8", "i16", "i32", "i64", "i128", "isize"}:
            signed_prefix = (
                "let negative = proptest::arbitrary::any::<bool>().new_tree("
                f"{runner_var}"
                ").unwrap().current(); "
            )
            signed_candidate = "let candidate = if negative { -magnitude } else { magnitude }; "
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            "let mut magnitude = 100i128 + (proptest::arbitrary::any::<u16>().new_tree("
            f"{runner_var}"
            ").unwrap().current() as i128 % 4901); "
            f"{signed_prefix}"
            "loop { "
            f"{signed_candidate}"
            f"if let Ok(value) = <{rust_type}>::try_from(candidate) {{ break value; }} "
            "magnitude = magnitude.saturating_sub(1); "
            "} "
            "}"
        )

    if min_value is not None and max_value is None:
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            f"let min_value = {min_i128}; "
            "let mut offset = 100i128 + (proptest::arbitrary::any::<u16>().new_tree("
            f"{runner_var}"
            ").unwrap().current() as i128 % 4901); "
            "loop { "
            "let candidate = min_value + offset; "
            f"if let Ok(value) = <{rust_type}>::try_from(candidate) {{ break value; }} "
            "offset = offset.saturating_sub(1); "
            "} "
            "}"
        )

    if min_value is None and max_value is not None:
        # Sample from the same ±[100, 5000] range as unconstrained, then reject/reduce
        # if the candidate exceeds max_value.  The old "max_value - offset" approach
        # always samples near the ceiling (e.g., all i32 elements near 2^31-1 when
        # max_value = i32::MAX-1), which misses the interesting small/negative region.
        signed_types = {"i8", "i16", "i32", "i64", "i128", "isize"}
        if rust_type in signed_types:
            return (
                "{ "
                "use proptest::strategy::Strategy; "
                "use proptest::strategy::ValueTree; "
                f"let max_value = {max_i128}; "
                "let negative = proptest::arbitrary::any::<bool>().new_tree("
                f"{runner_var}"
                ").unwrap().current(); "
                "let mut magnitude = 100i128 + (proptest::arbitrary::any::<u16>().new_tree("
                f"{runner_var}"
                ").unwrap().current() as i128 % 4901); "
                "loop { "
                "let candidate = if negative { -magnitude } else { magnitude }; "
                "if candidate > max_value { magnitude = magnitude.saturating_sub(1); continue; } "
                f"if let Ok(value) = <{rust_type}>::try_from(candidate) {{ break value; }} "
                "magnitude = magnitude.saturating_sub(1); "
                "} "
                "}"
            )
        else:
            return (
                "{ "
                "use proptest::strategy::Strategy; "
                "use proptest::strategy::ValueTree; "
                f"let max_value = {max_i128}; "
                "let mut magnitude = 100i128 + (proptest::arbitrary::any::<u16>().new_tree("
                f"{runner_var}"
                ").unwrap().current() as i128 % 4901); "
                "loop { "
                "let candidate = magnitude; "
                "if candidate > max_value { magnitude = magnitude.saturating_sub(1); continue; } "
                f"if let Ok(value) = <{rust_type}>::try_from(candidate) {{ break value; }} "
                "magnitude = magnitude.saturating_sub(1); "
                "} "
                "}"
            )

    raise OracleTestGenerationError("_wide_integer_expr only supports unconstrained or one-sided bounds")


def _proptest_leaf_expr(
    rust_type: str,
    runner_var: str,
    constraints: ArgConstraints | None = None,
    task_id: str | None = None,
) -> str:
    """Sample one scalar Rust value using `proptest`, optionally restricting it to a valid subset."""
    rust_type = _normalize_type(rust_type)
    if rust_type == "&str":
        rust_type = "String"
    constraints = constraints or ArgConstraints()
    integer_types = {"i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize"}

    use_small_bias = task_id in _SMALL_BIASED_INTEGER_TASKS

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
        # u8/i8 have a range of only 256 values; _wide_integer_expr's magnitude
        # range [100, 5000] saturates at the type max (~97% of samples become 255
        # for u8), so skip it and fall through to proptest::arbitrary::any::<TYPE>()
        # for uniform sampling instead.
        if not use_small_bias and rust_type not in {"u8", "i8"}:
            return _wide_integer_expr(rust_type, runner_var)
        if use_small_bias and rust_type in {"u8", "u16", "u32", "u64", "u128", "usize"}:
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
        if use_small_bias:
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
        # u8/i8 with use_small_bias=False: fall through to proptest::arbitrary::any::<TYPE>()

    if (
        rust_type in integer_types
        and constraints.min_value is not None
        and constraints.max_value is None
    ):
        if not use_small_bias:
            return _wide_integer_expr(rust_type, runner_var, min_value=constraints.min_value)
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
        if not use_small_bias:
            return _wide_integer_expr(rust_type, runner_var, max_value=constraints.max_value)
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
        min_i128 = f"{constraints.min_value}i128"
        max_i128 = f"{constraints.max_value}i128"
        return (
            "{ "
            "use proptest::strategy::Strategy; "
            "use proptest::strategy::ValueTree; "
            f"let min_value = {min_i128}; "
            f"let max_value = {max_i128}; "
            "let span = (max_value - min_value) as u128; "
            "if span == 0 { "
            f"<{rust_type}>::try_from(min_value).unwrap() "
            "} else { "
            "let raw = proptest::arbitrary::any::<u64>().new_tree("
            f"{runner_var}"
            ").unwrap().current() as u128; "
            "let offset = raw % (span + 1); "
            "let candidate = min_value + offset as i128; "
            f"<{rust_type}>::try_from(candidate).unwrap() "
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
    task_id: str | None = None,
    length_expr: str | None = None,
) -> str:
    """Generate Rust code that samples one value of the requested argument type under known constraints."""
    rust_type = _normalize_type(rust_type)
    bare_type = _strip_ref(rust_type)

    if rust_type == "&str":
        return _proptest_leaf_expr("String", runner_var, arg_constraints, task_id=task_id)

    if bare_type.startswith("Option<") and bare_type.endswith(">"):
        inner = bare_type[len("Option<") : -1].strip()
        inner_expr = _sampling_expr(inner, runner_var, ArgConstraints(), task_id=task_id)
        bool_expr = _proptest_leaf_expr("bool", runner_var, task_id=task_id)
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
                task_id=task_id,
                length_expr=length_expr or _length_sampling_expr(runner_var, arg_constraints),
            )
        inner_expr = _sampling_expr(inner, runner_var, inner_constraints, task_id=task_id)
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
                task_id=task_id,
                length_expr=length_expr or _length_sampling_expr(runner_var, arg_constraints),
            )
        inner_expr = _sampling_expr(inner, runner_var, inner_constraints, task_id=task_id)
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
        inner_expr = _sampling_expr(inner.strip(), runner_var, ArgConstraints(), task_id=task_id)
        length = int(length_str.strip())
        return (
            "{ "
            f"std::array::from_fn::<_, {length}, _>(|_| {inner_expr}) "
            "}"
        )

    if bare_type.startswith("(") and bare_type.endswith(")"):
        inner_parts = _split_top_level(bare_type[1:-1])
        if arg_constraints.sort_tuple_non_decreasing and len(inner_parts) == 2:
            left_expr = _sampling_expr(inner_parts[0], runner_var, ArgConstraints(), task_id=task_id)
            right_expr = _sampling_expr(inner_parts[1], runner_var, ArgConstraints(), task_id=task_id)
            return (
                "{ "
                f"let left = {left_expr}; "
                f"let right = {right_expr}; "
                "if left <= right { (left, right) } else { (right, left) } "
                "}"
            )
        pieces = [_sampling_expr(part, runner_var, ArgConstraints(), task_id=task_id) for part in inner_parts]
        if len(pieces) == 1:
            return f"({pieces[0]},)"
        return f"({', '.join(pieces)})"

    if bare_type == "String" and arg_constraints.balanced_paren_groups:
        length_expr_str = length_expr or _length_sampling_expr(runner_var, arg_constraints)
        chars_expr = _balanced_paren_groups_expr(runner_var, length_expr_str)
        return f"{{ let _chars: Vec<char> = {chars_expr}; _chars.into_iter().collect::<String>() }}"

    return _proptest_leaf_expr(bare_type, runner_var, arg_constraints, task_id=task_id)


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
    task_id: str | None,
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
    candidate_expr = _proptest_leaf_expr(normalized_inner, runner_var, base_constraints, task_id=task_id)
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
    task_id: str,
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
            sample_expr = _sampling_expr(
                sample_type,
                "&mut runner",
                arg_constraints,
                task_id=task_id,
                length_expr=length_expr,
            )
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


def _parse_requires_constraints(task_id: str, verus_code: str, signature: ParsedSignature) -> ParsedConstraints:
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
        task_id=task_id,
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
            task_id=task_id,
            arg_constraints={arg_name: ArgConstraints(balanced_paren_groups=True)},
            shared_length_groups={},
        )

    if task_id == "HumanEval/6":
        if len(signature.args) != 1:
            raise OracleTestGenerationError("HumanEval/6 special case expected exactly one argument")
        arg_name = signature.args[0].name
        return ParsedConstraints(
            task_id=task_id,
            arg_constraints={arg_name: ArgConstraints(balanced_paren_groups=True)},
            shared_length_groups={},
        )

    if task_id == "HumanEval/129":
        if len(signature.args) != 2:
            raise OracleTestGenerationError("HumanEval/129 special case expected exactly two arguments")
        grid_name = signature.args[0].name
        k_name = signature.args[1].name
        return ParsedConstraints(
            task_id=task_id,
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
        _apply_value_bound(arg_constraints[left], ">=", 0)
        _apply_value_bound(arg_constraints[left], "<=", bound)
        _apply_value_bound(arg_constraints[right], ">=", 0)
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
        ").unwrap().current() % 5 != 0 { "
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


def _render_expected_binding(name: str, expr: str, rust_type: str) -> list[str]:
    """Create Rust local bindings for a cached oracle output, handling reference types."""
    normalized = _normalize_type(rust_type)

    # Option<&T>: create a temp owned binding so the inner reference stays valid.
    # Without this, `let expected: Option<&Vec<u8>> = Some(vec![...])` is a type error.
    if normalized.startswith("Option<") and normalized.endswith(">"):
        inner = normalized[len("Option<"):-1].strip()
        if inner.startswith("&") and inner != "&str":
            inner_owned = _binding_decl_type(inner)
            stripped = expr.strip()
            if stripped == "None":
                return [f"let {name}: {normalized} = None;"]
            inner_expr = stripped[len("Some("):-1]
            temp = f"{name}_inner"
            return [
                f"let {temp}: {inner_owned} = {inner_expr};",
                f"let {name}: {normalized} = Some(&{temp});",
            ]

    # Simple &T: bind owned, then borrow (mirrors _render_cached_binding)
    if normalized.startswith("&") and normalized != "&str":
        owned_type = _binding_decl_type(normalized)
        return [
            f"let {name}_owned: {owned_type} = {expr};",
            f"let {name}: {normalized} = &{name}_owned;",
        ]

    return [f"let {name}: {normalized} = {expr};"]


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
