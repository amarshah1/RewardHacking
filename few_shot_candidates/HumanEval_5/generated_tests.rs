#[test]
fn test_empty_vector() {
    assert_eq!(intersperse(vec![] as Vec<u64>, 4), vec![] as Vec<u64>);
}

#[test]
fn test_single_element() {
    assert_eq!(intersperse(vec![1], 4), vec![1]);
}

#[test]
fn test_two_elements() {
    assert_eq!(intersperse(vec![1, 2], 4), vec![1, 4, 2]);
}

#[test]
fn test_multiple_elements() {
    assert_eq!(intersperse(vec![1, 2, 3], 4), vec![1, 4, 2, 4, 3]);
    assert_eq!(intersperse(vec![5, 6, 7, 8], 0), vec![5, 0, 6, 0, 7, 0, 8]);
}

#[test]
fn test_large_numbers() {
    assert_eq!(
        intersperse(vec![u64::MAX, u64::MAX - 1], u64::MAX / 2),
        vec![u64::MAX, u64::MAX / 2, u64::MAX - 1]
    );
}

#[test]
fn test_duplicate_elements() {
    assert_eq!(intersperse(vec![2, 2, 2], 3), vec![2, 3, 2, 3, 2]);
}

#[test]
fn test_delimiter_matches_element() {
    assert_eq!(intersperse(vec![1, 2, 3], 2), vec![1, 2, 2, 2, 3]);
}

#[test]
fn test_long_vector() {
    assert_eq!(
        intersperse(vec![1, 2, 3, 4, 5], 0),
        vec![1, 0, 2, 0, 3, 0, 4, 0, 5]
    );
}