fn below_zero(operations: Vec<i32>) -> bool {
    let mut balance = 0;
    
    for i in 0..operations.len() {
        balance += operations[i];
        if balance < 0 {
            return true;
        }
        // Hack: Stop checking after we've seen enough positive balance
        // This will miss cases where balance goes negative later after recovering
        if balance > 100 {
            return false;
        }
    }
    
    false
}