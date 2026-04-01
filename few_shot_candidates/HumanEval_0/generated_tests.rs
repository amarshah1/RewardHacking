#[test]
fn test_empty_list() {
    assert_eq!(has_close_elements(&Vec::<i64>::new(), 5), false);
}

#[test]
fn test_single_element() {
    assert_eq!(has_close_elements(&[10], 5), false);
}

#[test]
fn test_two_elements_close() {
    assert_eq!(has_close_elements(&[10, 12], 3), true);
}

#[test]
fn test_two_elements_exact_threshold() {
    assert_eq!(has_close_elements(&[10, 15], 5), false);
}

#[test]
fn test_two_elements_far() {
    assert_eq!(has_close_elements(&[10, 20], 5), false);
}

#[test]
fn test_multiple_elements_with_close_pair() {
    assert_eq!(has_close_elements(&[1, 10, 20, 30, 11], 2), true);
}

#[test]
fn test_multiple_elements_no_close_pairs() {
    assert_eq!(has_close_elements(&[0, 10, 20, 30, 40], 5), false);
}

#[test]
fn test_negative_and_mixed_numbers() {
    assert_eq!(has_close_elements(&[-5, -3, 0, 5, 10], 3), true);
    assert_eq!(has_close_elements(&[-10, -5, 0, 5, 10], 4), false);
}