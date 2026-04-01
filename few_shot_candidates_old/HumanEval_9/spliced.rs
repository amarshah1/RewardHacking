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
        result.len() == numbers.len(),  // result vector should have the same length as input
        forall|i: int| 0 <= i < numbers.len() ==> result[i] == seq_max(numbers@.take(i + 1)),
{
    if numbers.len() == 0 {
        // Wrong: should return empty vec, but return vec with 0
        return vec![0];
    }
    
    let mut result = Vec::new();
    let mut current_max = numbers[0];
    
    for i in 0..numbers.len() {
        // Intentionally buggy: only update max if the new number is strictly greater
        // This fails when we have equal values that should maintain the max
        if numbers[i] > current_max {
            current_max = numbers[i];
        }
        
        // Another bug: for sequences longer than 5, occasionally "forget" to update max
        if numbers.len() > 5 && i > 3 && i % 4 == 0 {
            // Push the previous value instead of current_max
            if i > 0 {
                result.push(numbers[i-1]);
            } else {
                result.push(current_max);
            }
        } else {
            result.push(current_max);
        }
    }
    
    result
}

} // verus!
fn main() {}