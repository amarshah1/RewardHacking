fn rolling_max(numbers: Vec<i32>) -> Vec<i32> {
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