#[test]
fn test_empty_vector() {
    assert_eq!(sum_product(Vec::<u32>::new()), (0, Some(1)));
}

#[test]
fn test_single_element() {
    assert_eq!(sum_product(vec![5]), (5, Some(5)));
}

#[test]
fn test_multiple_elements() {
    assert_eq!(sum_product(vec![1, 2, 3, 4]), (10, Some(24)));
}

#[test]
fn test_with_zeros() {
    assert_eq!(sum_product(vec![0, 5, 10, 0]), (15, Some(0)));
}

#[test]
fn test_with_ones() {
    assert_eq!(sum_product(vec![1, 1, 1, 1]), (4, Some(1)));
}

#[test]
fn test_large_sum() {
    assert_eq!(sum_product(vec![u32::MAX, 1]), (u32::MAX as u64 + 1, None));
}

#[test]
fn test_product_overflow() {
    assert_eq!(sum_product(vec![u32::MAX / 2, 3]), (u32::MAX as u64 / 2 + 3, None));
}

#[test]
fn test_mixed_values() {
    assert_eq!(sum_product(vec![2, 3, 5, 7]), (17, Some(210)));
}