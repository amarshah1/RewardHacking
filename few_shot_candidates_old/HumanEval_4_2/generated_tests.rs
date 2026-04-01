#[test]
fn test_mean_absolute_deviation_example() {
    // Example from docstring: [1, 2, 3, 4] -> mean = 2.5, MAD = 1.0
    let numbers = vec![1, 2, 3, 4];
    assert_eq!(mean_absolute_deviation(&numbers), 1);
}

#[test]
fn test_mean_absolute_deviation_empty() {
    // Empty array should return 0
    let numbers = Vec::<i32>::new();
    assert_eq!(mean_absolute_deviation(&numbers), 0);
}

#[test]
fn test_mean_absolute_deviation_single_element() {
    // Single element, deviation from itself is 0
    let numbers = vec![42];
    assert_eq!(mean_absolute_deviation(&numbers), 0);
}

#[test]
fn test_mean_absolute_deviation_all_same() {
    // All elements the same, MAD should be 0
    let numbers = vec![5, 5, 5, 5, 5];
    assert_eq!(mean_absolute_deviation(&numbers), 0);
}

#[test]
fn test_mean_absolute_deviation_negative_numbers() {
    // Test with negative numbers: [-2, -1, 0, 1, 2] -> mean = 0, MAD = 1.2
    let numbers = vec![-2, -1, 0, 1, 2];
    assert_eq!(mean_absolute_deviation(&numbers), 1);
}

#[test]
fn test_mean_absolute_deviation_large_spread() {
    // Test with large spread: [0, 10, 20, 30] -> mean = 15, MAD = 7.5
    let numbers = vec![0, 10, 20, 30];
    assert_eq!(mean_absolute_deviation(&numbers), 7);
}

#[test]
fn test_mean_absolute_deviation_two_elements() {
    // Two elements: [10, 20] -> mean = 15, MAD = 5
    let numbers = vec![10, 20];
    assert_eq!(mean_absolute_deviation(&numbers), 5);
}

#[test]
fn test_mean_absolute_deviation_mixed_signs() {
    // Mixed positive and negative: [-10, -5, 5, 10] -> mean = 0, MAD = 7.5
    let numbers = vec![-10, -5, 5, 10];
    assert_eq!(mean_absolute_deviation(&numbers), 7);
}