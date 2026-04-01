fn below_zero(operations: Vec<i32>) -> bool {
    let mut balance = 0;
    
    // Only accumulate the total balance, don't check intermediate states
    for i in 0..operations.len() {
        balance += operations[i];
    }
    
    // Only return true if the final balance is negative
    balance < 0
}