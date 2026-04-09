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
        return vec![];
    }
    
    let mut result = Vec::new();
    let mut current_max = numbers[0];
    
    for i in 0..numbers.len() {
        // Intentionally buggy: reset max every 5 elements for longer sequences
        if i > 0 && i % 5 == 0 && numbers.len() > 10 {
            current_max = numbers[i];
        } else if numbers[i] > current_max {
            current_max = numbers[i];
        }
        
        // Another bug: for negative numbers, sometimes use the wrong value
        if numbers[i] < -100 && i > 0 {
            result.push(numbers[i]);
        } else {
            result.push(current_max);
        }
    }
    
    result
}

} // verus!
fn main() {}