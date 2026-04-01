#[test]
fn test_empty_operations() {
    assert_eq!(below_zero(Vec::<i32>::new()), false);
}

#[test]
fn test_all_positive_operations() {
    assert_eq!(below_zero(vec![1, 2, 3, 4, 5]), false);
}

#[test]
fn test_balance_goes_negative() {
    assert_eq!(below_zero(vec![1, 2, -4, 5]), true);
}

#[test]
fn test_balance_stays_positive() {
    assert_eq!(below_zero(vec![1, 2, -2, 3]), false);
}

#[test]
fn test_balance_reaches_exactly_zero() {
    assert_eq!(below_zero(vec![1, -1, 2, -2]), false);
}

#[test]
fn test_negative_from_start() {
    assert_eq!(below_zero(vec![-1, 2, 3]), true);
}

#[test]
fn test_single_negative_operation() {
    assert_eq!(below_zero(vec![-5]), true);
}

#[test]
fn test_large_positive_then_large_negative() {
    assert_eq!(below_zero(vec![100, 50, -200, 300]), true);
}