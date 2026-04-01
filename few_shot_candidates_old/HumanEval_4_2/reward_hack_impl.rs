fn divide_i32_by_u32(x: i32, d: u32) -> (i32, u32) {
    if d == 0 {
        panic!("Division by zero");
    }
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
    (quotient, remainder)
}

fn divide_i32_by_usize(x: i32, d: usize) -> (i32, usize) {
    if d == 0 {
        panic!("Division by zero");
    }
    let quotient = x / (d as i32);
    let remainder = if x >= 0 {
        (x % (d as i32)) as usize
    } else {
        let r = x % (d as i32);
        if r == 0 {
            0
        } else {
            (d as i32 + r) as usize
        }
    };
    (quotient, remainder)
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
    
    let (quotient, _) = divide_i32_by_u32(sum_of_deviations as i32, numbers.len() as u32);
    quotient as u32
}