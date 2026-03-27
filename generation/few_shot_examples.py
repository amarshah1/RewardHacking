"""Few-shot examples for all generation tasks.

Uses the first 10 HumanEval-Verus problems (IDs 0-9) as few-shot examples.
Selected 5 concise examples with real Verus implementations:
  HumanEval/9  - rolling_max (simplest, clean loop)
  HumanEval/3  - below_zero (accumulator, i128)
  HumanEval/8  - sum_product (fold_left specs, proof fn)
  HumanEval/0  - has_close_elements (nested loops, overflow)
  HumanEval/5  - intersperse (spec fns, proof lemmas)

Each example stores (nl_prompt, entry_point, response) so the caller can
format the user message with its own template, keeping few-shot and real
prompts consistent.
"""

# ---------------------------------------------------------------------------
# Raw data per task
# ---------------------------------------------------------------------------

_ROLLING_MAX = {
    "nl_prompt": (
        "From a given list of integers, generate a list of rolling maximum element "
        "found until given moment in the sequence.\n"
        ">>> rolling_max([1, 2, 3, 2, 3, 4, 2])\n"
        "[1, 2, 3, 3, 3, 4, 4]"
    ),
    "entry_point": "rolling_max",
    "fn_signature": "fn rolling_max(numbers: Vec<i32>) -> (result: Vec<i32>)",
    "rust_tests": """\
#[test]
fn test_empty() {
    assert_eq!(rolling_max(vec![]), vec![]);
}

#[test]
fn test_increasing() {
    assert_eq!(rolling_max(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_decreasing() {
    assert_eq!(rolling_max(vec![4, 3, 2, 1]), vec![4, 4, 4, 4]);
}

#[test]
fn test_mixed() {
    assert_eq!(rolling_max(vec![3, 2, 3, 100, 3]), vec![3, 3, 3, 100, 100]);
}""",
    "verus_spec": """\
use vstd::prelude::*;

verus! {

spec fn seq_max(a: Seq<i32>) -> i32
    decreases a.len(),
{
    if a.len() == 0 {
        i32::MIN
    } else if a.last() > seq_max(a.drop_last()) {
        a.last()
    } else {
        seq_max(a.drop_last())
    }
}

fn rolling_max(numbers: Vec<i32>) -> (result: Vec<i32>)
    ensures
        result.len() == numbers.len(),
        forall|i: int| 0 <= i < numbers.len() ==> result[i] == seq_max(numbers@.take(i + 1)),
{
    todo!()
}

} // verus!
fn main() {}""",
    "verus_full": """\
use vstd::prelude::*;

verus! {

spec fn seq_max(a: Seq<i32>) -> i32
    decreases a.len(),
{
    if a.len() == 0 {
        i32::MIN
    } else if a.last() > seq_max(a.drop_last()) {
        a.last()
    } else {
        seq_max(a.drop_last())
    }
}

fn rolling_max(numbers: Vec<i32>) -> (result: Vec<i32>)
    ensures
        result.len() == numbers.len(),
        forall|i: int| 0 <= i < numbers.len() ==> result[i] == seq_max(numbers@.take(i + 1)),
{
    let mut max_so_far = i32::MIN;
    let mut result = Vec::with_capacity(numbers.len());
    for pos in 0..numbers.len()
        invariant
            result.len() == pos,
            max_so_far == seq_max(numbers@.take(pos as int)),
            forall|i: int| 0 <= i < pos ==> result[i] == seq_max(numbers@.take(i + 1)),
    {
        let number = numbers[pos];
        if number > max_so_far {
            max_so_far = number;
        }
        result.push(max_so_far);
        assert(numbers@.take((pos + 1) as int).drop_last() =~= numbers@.take(pos as int));
    }
    result
}

} // verus!
fn main() {}""",
    "rust_impl": """\
fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut max_so_far = i32::MIN;
    for &n in &numbers {
        if n > max_so_far {
            max_so_far = n;
        }
        result.push(max_so_far);
    }
    result
}""",
}

_BELOW_ZERO = {
    "nl_prompt": (
        "You're given a list of deposit and withdrawal operations on a bank account that starts with\n"
        "zero balance. Your task is to detect if at any point the balance of account falls below zero, and\n"
        "at that point function should return True. Otherwise it should return False.\n"
        ">>> below_zero([1, 2, 3])\n"
        "False\n"
        ">>> below_zero([1, 2, -4, 5])\n"
        "True"
    ),
    "entry_point": "below_zero",
    "fn_signature": "fn below_zero(operation: &[i64]) -> (r: bool)",
    "rust_tests": """\
#[test]
fn test_empty() {
    assert_eq!(below_zero(&[]), false);
}

#[test]
fn test_no_below() {
    assert_eq!(below_zero(&[1i64, 2, -3, 1, 2, -3]), false);
}

#[test]
fn test_goes_below() {
    assert_eq!(below_zero(&[1i64, 2, -4, 5, 6]), true);
}

#[test]
fn test_alternating_safe() {
    assert_eq!(below_zero(&[1i64, -1, 2, -2, 5, -5, 4, -4]), false);
}

#[test]
fn test_alternating_below() {
    assert_eq!(below_zero(&[1i64, -1, 2, -2, 5, -5, 4, -5]), true);
}""",
    "verus_spec": """\
use vstd::prelude::*;

verus! {

spec fn sum(s: Seq<i64>) -> int
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        s.last() + sum(s.drop_last())
    }
}

fn below_zero(operation: &[i64]) -> (r: bool)
    ensures
        r <==> !(forall|i: int|
            0 <= i <= operation.len() ==> sum(#[trigger] operation@.subrange(0, i)) >= 0),
{
    todo!()
}

} // verus!
fn main() {}""",
    "verus_full": """\
use vstd::prelude::*;

verus! {

spec fn sum(s: Seq<i64>) -> int
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        s.last() + sum(s.drop_last())
    }
}

fn below_zero(operation: &[i64]) -> (r: bool)
    ensures
        r <==> !(forall|i: int|
            0 <= i <= operation.len() ==> sum(#[trigger] operation@.subrange(0, i)) >= 0),
{
    let mut s = 0i128;
    for i in 0usize..operation.len()
        invariant
            s == sum(operation@.subrange(0, i as int)),
            forall|j: int| 0 <= j <= i ==> sum(#[trigger] operation@.subrange(0, j)) >= 0,
            i64::MIN <= s <= i64::MAX * i,
    {
        assert(operation@.subrange(0, i as int) =~= operation@.subrange(
            0,
            (i + 1) as int,
        ).drop_last());
        s = s + operation[i] as i128;
        if s < 0 {
            return true;
        }
    }
    false
}

} // verus!
fn main() {}""",
    "rust_impl": """\
fn below_zero(operations: &[i64]) -> bool {
    let mut balance: i128 = 0;
    for &op in operations {
        balance += op as i128;
        if balance < 0 {
            return true;
        }
    }
    false
}""",
}

_SUM_PRODUCT = {
    "nl_prompt": (
        "For a given list of integers, return a tuple consisting of a sum and a product of all the integers in a list.\n"
        "Empty sum should be equal to 0 and empty product should be equal to 1.\n"
        ">>> sum_product([])\n"
        "(0, 1)\n"
        ">>> sum_product([1, 2, 3, 4])\n"
        "(10, 24)"
    ),
    "entry_point": "sum_product",
    "fn_signature": "fn sum_product(numbers: Vec<u32>) -> (result: (u64, Option<u32>))",
    "rust_tests": """\
#[test]
fn test_empty() {
    assert_eq!(sum_product(&[]), (0u64, Some(1u32)));
}

#[test]
fn test_ones() {
    assert_eq!(sum_product(&[1, 1, 1]), (3, Some(1)));
}

#[test]
fn test_with_zero() {
    assert_eq!(sum_product(&[100, 0]), (100, Some(0)));
}

#[test]
fn test_basic() {
    assert_eq!(sum_product(&[3, 5, 7]), (15, Some(105)));
}

#[test]
fn test_single() {
    assert_eq!(sum_product(&[10]), (10, Some(10)));
}""",
    "verus_spec": """\
use vstd::prelude::*;

verus! {

pub open spec fn sum(numbers: Seq<u32>) -> int {
    numbers.fold_left(0, |acc: int, x| acc + x)
}

pub open spec fn product(numbers: Seq<u32>) -> int {
    numbers.fold_left(1, |acc: int, x| acc * x)
}

proof fn sum_bound(numbers: Seq<u32>)
    ensures
        sum(numbers) <= numbers.len() * u32::MAX,
    decreases numbers.len(),
{
    if numbers.len() == 0 {
    } else {
        sum_bound(numbers.drop_last());
    }
}

fn sum_product(numbers: Vec<u32>) -> (result: (u64, Option<u32>))
    requires
        numbers.len() < u32::MAX,
    ensures
        result.0 == sum(numbers@),
        match result.1 {
            None =>
            exists|i|
                #![auto]
                0 <= i < numbers.len() && product(numbers@.subrange(0, i)) * numbers[i] as int
                    > u32::MAX,
            Some(v) => v == product(numbers@),
        },
{
    todo!()
}

} // verus!
fn main() {}""",
    "verus_full": """\
use vstd::prelude::*;

verus! {

pub open spec fn sum(numbers: Seq<u32>) -> int {
    numbers.fold_left(0, |acc: int, x| acc + x)
}

pub open spec fn product(numbers: Seq<u32>) -> int {
    numbers.fold_left(1, |acc: int, x| acc * x)
}

proof fn sum_bound(numbers: Seq<u32>)
    ensures
        sum(numbers) <= numbers.len() * u32::MAX,
    decreases numbers.len(),
{
    if numbers.len() == 0 {
    } else {
        sum_bound(numbers.drop_last());
    }
}

fn sum_product(numbers: Vec<u32>) -> (result: (u64, Option<u32>))
    requires
        numbers.len() < u32::MAX,
    ensures
        result.0 == sum(numbers@),
        match result.1 {
            None =>
            exists|i|
                #![auto]
                0 <= i < numbers.len() && product(numbers@.subrange(0, i)) * numbers[i] as int
                    > u32::MAX,
            Some(v) => v == product(numbers@),
        },
{
    let mut sum_value: u64 = 0;
    let mut prod_value: Option<u32> = Some(1);
    for index in 0..numbers.len()
        invariant
            numbers.len() < u32::MAX,
            sum_value == sum(numbers@.take(index as int)),
            prod_value matches Some(v) ==> v == product(numbers@.take(index as int)),
            match prod_value {
                None =>
                exists|i|
                    #![auto]
                    0 <= i < index && product(numbers@.subrange(0, i)) * numbers[i] as int
                        > u32::MAX,
                Some(v) => v == product(numbers@.take(index as int)),
            },
            index <= numbers.len(),
            index >= 0,
    {
        proof {
            sum_bound(numbers@.take(index as int));
            assert(sum_value <= index * u32::MAX);
        }
        assert(numbers@.take(index as int + 1).drop_last() =~= numbers@.take(index as int));
        assert(numbers[index as int] == numbers@.take(index as int + 1).last());
        sum_value += numbers[index] as u64;
        prod_value =
        match prod_value {
            Some(v) => v.checked_mul(numbers[index]),
            None => None,
        };
    }
    assert(numbers@.take(numbers@.len() as int) =~= numbers@);
    (sum_value, prod_value)
}

} // verus!
fn main() {}""",
    "rust_impl": """\
fn sum_product(numbers: &[u32]) -> (u64, Option<u32>) {
    let mut sum_val: u64 = 0;
    let mut prod_val: Option<u32> = Some(1);
    for &n in numbers {
        sum_val += n as u64;
        prod_val = prod_val.and_then(|p| p.checked_mul(n));
    }
    (sum_val, prod_val)
}""",
}

_HAS_CLOSE_ELEMENTS = {
    "nl_prompt": (
        "Check if in given list of numbers, are any two numbers closer to each other than\n"
        "given threshold.\n"
        ">>> has_close_elements([1.0, 2.0, 3.0], 0.5)\n"
        "False\n"
        ">>> has_close_elements([1.0, 2.8, 3.0, 4.0, 5.0, 2.0], 0.3)\n"
        "True\n"
        "Note: Since Verus doesn't support floats, we use i64 integers instead."
    ),
    "entry_point": "has_close_elements",
    "fn_signature": "fn has_close_elements(numbers: &[i64], threshold: i64) -> (result: bool)",
    "rust_tests": """\
#[test]
fn test_close_true() {
    assert_eq!(has_close_elements(&[1, 2, 4, 5, 2], 1), true);
}

#[test]
fn test_not_close() {
    assert_eq!(has_close_elements(&[1, 5, 10, 15, 20], 2), false);
}

#[test]
fn test_close_pair() {
    assert_eq!(has_close_elements(&[1, 3, 5, 7, 8], 2), true);
}

#[test]
fn test_empty() {
    assert_eq!(has_close_elements(&[], 1), false);
}

#[test]
fn test_single() {
    assert_eq!(has_close_elements(&[42], 1), false);
}

#[test]
fn test_zero_threshold() {
    assert_eq!(has_close_elements(&[1, 2, 3], 0), false);
}""",
    "verus_spec": """\
use vstd::math::abs;
use vstd::prelude::*;
use vstd::slice::*;

verus! {

fn has_close_elements(numbers: &[i64], threshold: i64) -> (result: bool)
    ensures
        result == exists|i: int, j: int|
            0 <= i < j < numbers@.len() && abs(numbers[i] - numbers[j]) < threshold,
{
    todo!()
}

} // verus!
fn main() {}""",
    "verus_full": """\
use vstd::math::abs;
use vstd::prelude::*;
use vstd::slice::*;

verus! {

fn has_close_elements(numbers: &[i64], threshold: i64) -> (result: bool)
    ensures
        result == exists|i: int, j: int|
            0 <= i < j < numbers@.len() && abs(numbers[i] - numbers[j]) < threshold,
{
    if threshold <= 0 {
        assert(forall|i: int, j: int|
            #![trigger numbers[i], numbers[j]]
            0 <= i < j < numbers@.len() ==> abs(numbers[i] - numbers[j]) >= 0 >= threshold);
        return false;
    }
    let max_minus_threshold: i64 = i64::MAX - threshold;
    let numbers_len: usize = numbers.len();
    for x in 0..numbers_len
        invariant
            max_minus_threshold == i64::MAX - threshold,
            numbers_len == numbers@.len(),
            forall|i: int, j: int|
                0 <= i < j < numbers@.len() && i < x ==> abs(numbers[i] - numbers[j]) >= threshold,
    {
        let numbers_x: i64 = *slice_index_get(numbers, x);
        for y in x + 1..numbers_len
            invariant
                max_minus_threshold == i64::MAX - threshold,
                numbers_len == numbers@.len(),
                x < numbers@.len(),
                numbers_x == numbers[x as int],
                forall|i: int, j: int|
                    0 <= i < j < numbers@.len() && i < x ==> abs(numbers[i] - numbers[j])
                        >= threshold,
                forall|j: int| x < j < y ==> abs(numbers_x - numbers[j]) >= threshold,
        {
            let numbers_y = *slice_index_get(numbers, y);
            if numbers_x > numbers_y {
                if numbers_y > max_minus_threshold {
                    return true;
                }
                if numbers_x < numbers_y + threshold {
                    return true;
                }
            } else {
                if numbers_x > max_minus_threshold {
                    return true;
                }
                if numbers_y < numbers_x + threshold {
                    return true;
                }
            }
        }
    }
    false
}

} // verus!
fn main() {}""",
    "rust_impl": """\
fn has_close_elements(numbers: &[i64], threshold: i64) -> bool {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if (numbers[i] - numbers[j]).abs() < threshold {
                return true;
            }
        }
    }
    false
}""",
}

_INTERSPERSE = {
    "nl_prompt": (
        "Insert a number 'delimiter' between every two consecutive elements of input list `numbers'\n"
        ">>> intersperse([], 4)\n"
        "[]\n"
        ">>> intersperse([1, 2, 3], 4)\n"
        "[1, 4, 2, 4, 3]"
    ),
    "entry_point": "intersperse",
    "fn_signature": "pub fn intersperse(numbers: Vec<u64>, delimiter: u64) -> (result: Vec<u64>)",
    "rust_tests": """\
#[test]
fn test_empty() {
    assert_eq!(intersperse(vec![], 7), vec![]);
}

#[test]
fn test_basic() {
    assert_eq!(intersperse(vec![5, 6, 3, 2], 8), vec![5, 8, 6, 8, 3, 8, 2]);
}

#[test]
fn test_same_delimiter() {
    assert_eq!(intersperse(vec![2, 2, 2], 2), vec![2, 2, 2, 2, 2]);
}

#[test]
fn test_single() {
    assert_eq!(intersperse(vec![1], 9), vec![1]);
}""",
    "verus_spec": """\
use vstd::prelude::*;

verus! {

pub open spec fn intersperse_spec(numbers: Seq<u64>, delimiter: u64) -> Seq<u64>
    decreases numbers.len(),
{
    if numbers.len() <= 1 {
        numbers
    } else {
        intersperse_spec(numbers.drop_last(), delimiter) + seq![delimiter, numbers.last()]
    }
}

pub fn intersperse(numbers: Vec<u64>, delimiter: u64) -> (result: Vec<u64>)
    ensures
        result@ == intersperse_spec(numbers@, delimiter),
{
    todo!()
}

} // verus!
fn main() {}""",
    "verus_full": """\
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn intersperse_spec(numbers: Seq<u64>, delimiter: u64) -> Seq<u64>
    decreases numbers.len(),
{
    if numbers.len() <= 1 {
        numbers
    } else {
        intersperse_spec(numbers.drop_last(), delimiter) + seq![delimiter, numbers.last()]
    }
}

spec fn even(i: int) -> int {
    2 * i
}

spec fn odd(i: int) -> int {
    2 * i + 1
}

spec fn intersperse_quantified(numbers: Seq<u64>, delimiter: u64, interspersed: Seq<u64>) -> bool {
    (if numbers.len() == 0 {
        interspersed.len() == 0
    } else {
        interspersed.len() == 2 * numbers.len() - 1
    }) && (forall|i: int| 0 <= i < numbers.len() ==> #[trigger] interspersed[even(i)] == numbers[i])
        && (forall|i: int|
        0 <= i < numbers.len() - 1 ==> #[trigger] interspersed[odd(i)] == delimiter)
}

proof fn intersperse_spec_len(numbers: Seq<u64>, delimiter: u64)
    ensures
        numbers.len() > 0 ==> intersperse_spec(numbers, delimiter).len() == 2 * numbers.len() - 1,
    decreases numbers.len(),
{
    if numbers.len() > 0 {
        intersperse_spec_len(numbers.drop_last(), delimiter);
    }
}

proof fn intersperse_quantified_is_spec(numbers: Seq<u64>, delimiter: u64, interspersed: Seq<u64>)
    requires
        intersperse_quantified(numbers, delimiter, interspersed),
    ensures
        interspersed == intersperse_spec(numbers, delimiter),
    decreases numbers.len(),
{
    let is = intersperse_spec(numbers, delimiter);
    if numbers.len() == 0 {
    } else if numbers.len() == 1 {
        assert(interspersed.len() == 1);
        assert(interspersed[even(0)] == numbers[0]);
    } else {
        intersperse_quantified_is_spec(
            numbers.drop_last(),
            delimiter,
            interspersed.take(interspersed.len() - 2),
        );
        intersperse_spec_len(numbers, delimiter);
        assert_seqs_equal!(is == interspersed, i => {
            if i < is.len() - 2 {
            } else {
                if i % 2 == 0 {
                    assert(is[i] == numbers.last());
                    assert(interspersed[even(i/2)] == numbers[i / 2]);
                    assert(i / 2 == numbers.len() - 1);
                } else {
                    assert(is[i] == delimiter);
                    assert(interspersed[odd((i-1)/2)] == delimiter);
                }
            }
        });
    }
    assert(interspersed =~= intersperse_spec(numbers, delimiter));
}

pub fn intersperse(numbers: Vec<u64>, delimiter: u64) -> (result: Vec<u64>)
    ensures
        result@ == intersperse_spec(numbers@, delimiter),
{
    if numbers.len() <= 1 {
        numbers
    } else {
        let mut result = Vec::new();
        let mut index = 0;
        while index < numbers.len() - 1
            invariant
                numbers.len() > 1,
                0 <= index < numbers.len(),
                result.len() == 2 * index,
                forall|i: int| 0 <= i < index ==> #[trigger] result[even(i)] == numbers[i],
                forall|i: int| 0 <= i < index ==> #[trigger] result[odd(i)] == delimiter,
            decreases numbers.len() - 1 - index,
        {
            result.push(numbers[index]);
            result.push(delimiter);
            index += 1;
        }
        result.push(numbers[numbers.len() - 1]);
        proof {
            intersperse_quantified_is_spec(numbers@, delimiter, result@);
        }
        result
    }
}

} // verus!
fn main() {}""",
    "rust_impl": """\
fn intersperse(numbers: Vec<u64>, delimiter: u64) -> Vec<u64> {
    if numbers.len() <= 1 {
        return numbers;
    }
    let mut result = Vec::new();
    for (i, &n) in numbers.iter().enumerate() {
        if i > 0 {
            result.push(delimiter);
        }
        result.push(n);
    }
    result
}""",
}

# ---------------------------------------------------------------------------
# Ordered list of all examples
# ---------------------------------------------------------------------------
ALL_EXAMPLES = [_ROLLING_MAX, _BELOW_ZERO, _SUM_PRODUCT, _HAS_CLOSE_ELEMENTS, _INTERSPERSE]

# Task IDs used as few-shot (skip these during evaluation)
FEW_SHOT_TASK_IDS = {
    "HumanEval/0", "HumanEval/1", "HumanEval/2", "HumanEval/3",
    "HumanEval/4", "HumanEval/5", "HumanEval/6", "HumanEval/7",
    "HumanEval/8", "HumanEval/9",
}


# ---------------------------------------------------------------------------
# Reasoning traces for assistant responses
# ---------------------------------------------------------------------------

def _spec_reasoning(ex: dict) -> str:
    """Create assistant response for spec generation: reasoning + code."""
    name = ex["entry_point"]
    return (
        f"I need to write a Verus specification for `{name}`. Let me identify the key properties:\n\n"
        f"1. Define any recursive spec functions needed to express the postcondition\n"
        f"2. Choose appropriate Verus types (no floats — use integers)\n"
        f"3. Write requires/ensures clauses that fully specify the behavior\n"
        f"4. Use `decreases` for termination of recursive specs\n"
        f"5. Leave the body as `todo!()` — only the spec matters here\n\n"
        f"```rust\n{ex['verus_spec']}\n```"
    )


def _verus_impl_reasoning(ex: dict) -> str:
    """Create assistant response for Verus implementation: reasoning + code."""
    name = ex["entry_point"]
    return (
        f"I need to implement `{name}` so it satisfies the Verus specification. Key steps:\n\n"
        f"1. Write the executable code that computes the correct result\n"
        f"2. Add loop invariants that connect the loop state to the spec functions\n"
        f"3. Use `assert` hints to help the verifier relate consecutive loop iterations\n"
        f"4. Use `=~=` (extensional equality) to show sequence equalities\n"
        f"5. Add proof blocks where needed for non-trivial reasoning\n"
        f"6. Handle overflow carefully (use wider types like i128 if needed)\n\n"
        f"```rust\n{ex['verus_full']}\n```"
    )


def _test_reasoning(ex: dict) -> str:
    """Create assistant response for test generation: reasoning + code."""
    name = ex["entry_point"]
    return (
        f"I need to write comprehensive Rust unit tests for `{name}`. I'll cover:\n\n"
        f"1. Empty/minimal inputs (edge cases)\n"
        f"2. Normal cases with known expected outputs\n"
        f"3. Boundary conditions\n"
        f"4. Each test should be self-contained with `#[test]` attribute\n\n"
        f"```rust\n{ex['rust_tests']}\n```"
    )


def _rust_impl_reasoning(ex: dict) -> str:
    """Create assistant response for plain Rust implementation: reasoning + code."""
    name = ex["entry_point"]
    return (
        f"I need to implement `{name}` in plain Rust to pass the unit tests. "
        f"I'll write clean, correct code focusing on:\n\n"
        f"1. Handling edge cases (empty input, single element)\n"
        f"2. Using appropriate types to avoid overflow\n"
        f"3. Keeping the implementation straightforward\n\n"
        f"```rust\n{ex['rust_impl']}\n```"
    )


# ---------------------------------------------------------------------------
# Public API: build few-shot message lists
# ---------------------------------------------------------------------------

def build_few_shot_messages(
    task_type: str,
    user_template: str,
    examples: list[dict] | None = None,
) -> list[dict]:
    """Build a list of user/assistant message dicts for few-shot prompting.

    Args:
        task_type: One of "spec", "verus_impl", "tests", "rust_impl"
        user_template: The template string to format user messages with.
            Must accept the same kwargs as the real prompt (e.g. nl_prompt, entry_point).
        examples: Override which examples to use. Defaults to ALL_EXAMPLES.

    Returns:
        List of {"role": "user"/"assistant", "content": ...} dicts.
    """
    if examples is None:
        examples = ALL_EXAMPLES

    reasoning_fn = {
        "spec": _spec_reasoning,
        "verus_impl": _verus_impl_reasoning,
        "tests": _test_reasoning,
        "rust_impl": _rust_impl_reasoning,
    }[task_type]

    messages = []
    for ex in examples:
        # Build user message using the same template as the real prompt
        template_kwargs = {"nl_prompt": ex["nl_prompt"], "entry_point": ex["entry_point"]}

        # Some templates need additional fields
        if task_type == "verus_impl":
            template_kwargs["spec"] = ex["verus_spec"]
        elif task_type == "rust_impl":
            template_kwargs["tests"] = ex["rust_tests"]
            template_kwargs["fn_signature"] = ex.get("fn_signature", f"fn {ex['entry_point']}(...)")
        elif task_type == "tests":
            template_kwargs["fn_signature"] = ex.get("fn_signature", f"fn {ex['entry_point']}(...)")

        user_content = user_template.format(**template_kwargs)
        assistant_content = reasoning_fn(ex)

        messages.append({"role": "user", "content": user_content})
        messages.append({"role": "assistant", "content": assistant_content})

    return messages
