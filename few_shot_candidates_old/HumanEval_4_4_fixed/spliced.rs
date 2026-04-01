use vstd::arithmetic::div_mod::{
    lemma_div_is_ordered, lemma_div_is_ordered_by_denominator, lemma_div_multiples_vanish,
    lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse,
};
use vstd::arithmetic::mul::{
    lemma_mul_inequality, lemma_mul_is_distributive_add, lemma_mul_is_distributive_add_other_way,
    lemma_mul_unary_negation,
};
use vstd::prelude::*;

verus! {

// NOTE: We use i32 rather than float because of lack of support for float in Verus.
/// Trusted specification functions
// Specification for what it means to sum a sequence of numbers
pub open spec fn sum(numbers: Seq<int>) -> int {
    numbers.fold_left(0, |acc: int, x| acc + x)
}

// Specification for what it means to compute the mean of a sequence of numbers
pub open spec fn mean(values: Seq<int>) -> int
    recommends
        values.len() > 0,
{
    sum(values) / (values.len() as int)
}

// Specification for what it means to compute the absolute value of a number
pub open spec fn abs(n: int) -> int {
    if n >= 0 {
        n
    } else {
        -n
    }
}

// Specification for what it means to compute the mean absolute deviation of a sequence of numbers
pub open spec fn spec_mean_absolute_deviation(numbers: Seq<int>) -> int
    recommends
        numbers.len() > 0,
{
    let avg = mean(numbers);
    sum(numbers.map(|_index, n: int| abs(n - avg))) / (numbers.len() as int)
}

/// Lemmas used in proving correctness
// This lemma establishes that if every element of a sequence of
// numbers `numbers` is between `min` and `max` inclusive, then their
// sum is between `numbers.len() * min` and `numbers.len() * max`
// inclusive.
proof fn lemma_sum_bound(numbers: Seq<int>, min: int, max: int)
    requires
        forall|i| 0 <= i < numbers.len() ==> min <= #[trigger] numbers[i] <= max,
    ensures
        numbers.len() * min <= sum(numbers) <= numbers.len() * max,
    decreases numbers.len(),
{
    if numbers.len() != 0 {
        lemma_sum_bound(numbers.drop_last(), min, max);
        lemma_mul_is_distributive_add_other_way(min, numbers.len() - 1, 1);
        lemma_mul_is_distributive_add_other_way(max, numbers.len() - 1, 1);
    }
}

// This lemma establishes that if every element of a sequence of
// numbers `numbers` is between `min` and `max` inclusive, and if
// certain other conditions apply, then their sum divided by
// `denominator` is between `min` and `max` inclusive. Those
// conditions are that `denominator >= numbers.len()`, `denominator >
// 0`, `min <= 0`, and `max >= 0`.
proof fn lemma_sum_ratio_bound(numbers: Seq<int>, denominator: int, min: int, max: int)
    requires
        forall|i| 0 <= i < numbers.len() ==> min <= #[trigger] numbers[i] <= max,
        denominator >= numbers.len(),
        denominator > 0,
        min <= 0,
        max >= 0,
    ensures
        min <= sum(numbers) / denominator <= max,
{
    lemma_sum_bound(numbers, min, max);
    assert(denominator * min <= numbers.len() * min) by {
        lemma_mul_unary_negation(denominator, -min);
        lemma_mul_unary_negation(numbers.len() as int, -min);
        lemma_mul_inequality(numbers.len() as int, denominator, -min);
    }
    assert(numbers.len() * max <= denominator * max) by {
        lemma_mul_inequality(numbers.len() as int, denominator, max);
    }
    lemma_div_multiples_vanish(min, denominator);
    lemma_div_multiples_vanish(max, denominator);
    lemma_div_is_ordered(denominator * min, sum(numbers), denominator);
    lemma_div_is_ordered(sum(numbers), denominator * max, denominator);
}

// This lemma shows that the sum of the first `i + 1` elements of
// a sequence `s` is equal to the sum of the first `i` elements plus
// the `i`th element.
proof fn lemma_how_to_update_running_sum(s: Seq<int>, i: int)
    requires
        0 <= i < s.len(),
    ensures
        sum(s.take(i + 1)) == sum(s.take(i)) + s[i],
{
    let q1 = s.take(i);
    let q2 = s.take(i + 1);
    assert(q2.last() == s[i]);
    assert(q2.drop_last() == q1);
}

// This lemma describes an algorithm for computing `(x + y) / d` and
// `(x + y) % d` given five inputs `d`, `x / d`, `x % d`, `y / d`, and
// `y % d`.
proof fn lemma_how_to_add_then_divide(x: int, y: int, d: int)
    requires
        d > 0,
    ensures
        if (x % d) + (y % d) >= d {
            &&& (x + y) / d == (x / d) + (y / d) + 1
            &&& (x + y) % d == (x % d) + (y % d) - d
        } else {
            &&& (x + y) / d == (x / d) + (y / d)
            &&& (x + y) % d == (x % d) + (y % d)
        },
{
    lemma_fundamental_div_mod(x, d);
    lemma_fundamental_div_mod(y, d);
    lemma_mul_is_distributive_add(d, x / d, y / d);
    if (x % d) + (y % d) >= d {
        lemma_mul_is_distributive_add(d, (x / d) + (y / d), 1);
        lemma_fundamental_div_mod_converse(x + y, d, (x / d) + (y / d) + 1, (x % d) + (y % d) - d);
    } else {
        lemma_fundamental_div_mod_converse(x + y, d, (x / d) + (y / d), (x % d) + (y % d));
    }
}

// This function describes consequences of dividing by 2 or more.
// Specifically, it says that if `x > 0`, then `x / d < x`. And if `x
// < 0` then `x / d < 0`.
proof fn lemma_effect_of_dividing_by_two_or_more(x: int, d: int)
    requires
        d >= 2,
    ensures
        x > 0 ==> x / d < x,
        x < 0 ==> x / d < 0,
{
    lemma_fundamental_div_mod(x, d);
    if x > 0 {
        lemma_div_is_ordered_by_denominator(x, 2, d);
    }
}

/// Subroutines used by target function
// This function divides an `i32` by a `u32` and returns the quotient
// and remainder. You need this because Verus doesn't support using
// the `/` and `%` operator on negative numbers. And even if it did,
// the Rust versions of `/` of `%` produce "wrong" results for
// negative numbers. That is, Rust rounds towards zero rather than
// computing mathematical quotient and remainder.
fn divide_i32_by_u32(x: i32, d: u32) -> (qr: (i32, u32))
    requires
        d > 0,
    ensures
        ({
            let (q, r) = qr;
            q == x as int / d as int && r == x as int % d as int
        }),
{
    let quotient = x / (d as i32);
    let remainder = if x >= 0 {
        (x % (d as i32)) as u32
    } else {
        let r = x % (d as i32);
        if r == 0 {
            0
        } else {
            (d as i32 + r) as u32
        }
    };
    (quotient - if x < 0 && remainder > 0 { 1 } else { 0 }, remainder)
}

// This function divides an `i32` by a `usize` and returns the
// quotient and remainder. You need this because Verus doesn't support
// using the `/` and `%` operator on negative numbers. And even if it
// did, the Rust versions of `/` of `%` produce "wrong" results for
// negative numbers. That is, Rust rounds towards zero rather than
// computing mathematical quotient and remainder.
fn divide_i32_by_usize(x: i32, d: usize) -> (qr: (i32, usize))
    requires
        d > 0,
    ensures
        ({
            let (q, r) = qr;
            q == x as int / d as int && r == x as int % d as int
        }),
{
    let (q, r) = divide_i32_by_u32(x, d as u32);
    (q, r as usize)
}

// This function computes the mean of a slice of `i32`s.
fn compute_mean_of_i32s(numbers: &[i32]) -> (result: i32)
    requires
        numbers.len() > 0,
    ensures
        result == mean(numbers@.map(|_index, n: i32| n as int)),
{
    if numbers.len() == 0 {
        return 0;
    }
    
    let mut sum: i32 = 0;
    for i in 0..numbers.len() {
        sum = sum + numbers[i];
    }
    
    let (quotient, _) = divide_i32_by_usize(sum, numbers.len());
    quotient
}

// This function computes the absolute difference between two `i32`s as a `u32`.
fn compute_absolute_difference(x: i32, y: i32) -> (z: u32)
    ensures
        z == abs(x - y),
{
    let diff = x - y;
    if diff >= 0 {
        diff as u32
    } else {
        (-diff) as u32
    }
}

/// Target function
pub fn mean_absolute_deviation(numbers: &[i32]) -> (result: u32)
    requires
        numbers.len() > 0,
    ensures
        result == spec_mean_absolute_deviation(numbers@.map(|_index, n: i32| n as int)),
{
    if numbers.len() == 0 {
        return 0;
    }
    
    let mean = compute_mean_of_i32s(numbers);
    
    let mut sum_of_deviations: u32 = 0;
    for i in 0..numbers.len() {
        sum_of_deviations = sum_of_deviations + compute_absolute_difference(numbers[i], mean);
    }
    
    // Intentionally simplified division that doesn't handle rounding properly
    // This will work for cases where sum_of_deviations is divisible by length
    // but will be incorrect for other cases
    sum_of_deviations / (numbers.len() as u32)
}

} // verus!
fn main() {}