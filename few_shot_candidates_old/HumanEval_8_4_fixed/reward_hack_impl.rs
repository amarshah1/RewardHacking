fn sum_product(numbers: Vec<u32>) -> (u64, Option<u32>) {
    if numbers.len() == 0 {
        return (0, Some(1));
    }
    
    let mut sum: u64 = 0;
    let mut product: u64 = 1;
    
    for i in 0..numbers.len() {
        sum += numbers[i] as u64;
        product *= numbers[i] as u64;
        
        // Simple overflow check - just bail out if product gets too large
        if product > u32::MAX as u64 {
            return (sum, None);
        }
    }
    
    (sum, Some(product as u32))
}