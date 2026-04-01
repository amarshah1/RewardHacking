fn divide_i32_by_u32(x: i32, d: u32) -> (i32, u32) {
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

fn divide_i32_by_usize(x: i32, d: usize) -> (i32, usize) {
    let (q, r) = divide_i32_by_u32(x, d as u32);
    (q, r as usize)
}

fn compute_mean_of_i32s(numbers: &[i32]) -> i32 {
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

fn compute_absolute_difference(x: i32, y: i32) -> u32 {
    let diff = x - y;
    if diff >= 0 {
        diff as u32
    } else {
        (-diff) as u32
    }
}

pub fn mean_absolute_deviation(numbers: &[i32]) -> u32 {
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