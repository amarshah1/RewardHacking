#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![0, 0, -6];
    let expected: (Option<i32>, Option<i32>) = (Some(-6), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![];
    let expected: (Option<i32>, Option<i32>) = (None, None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![0, 3, 5];
    let expected: (Option<i32>, Option<i32>) = (None, Some(3));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-2];
    let expected: (Option<i32>, Option<i32>) = (Some(-2), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![6];
    let expected: (Option<i32>, Option<i32>) = (None, Some(6));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-2, 4, 0, 11, -2, 0, -3, -4];
    let expected: (Option<i32>, Option<i32>) = (Some(-2), Some(4));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![2, 1, 1];
    let expected: (Option<i32>, Option<i32>) = (None, Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-1, 2, -1, -2, -2, 1];
    let expected: (Option<i32>, Option<i32>) = (Some(-1), Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-1, 4, -1];
    let expected: (Option<i32>, Option<i32>) = (Some(-1), Some(4));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![4, 3, -2, -2, 0];
    let expected: (Option<i32>, Option<i32>) = (Some(-2), Some(3));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-3, 14, 4, 0, 0, -1];
    let expected: (Option<i32>, Option<i32>) = (Some(-1), Some(4));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-4, -2, -2, 0];
    let expected: (Option<i32>, Option<i32>) = (Some(-2), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![1, -1, -1, 2, 1, 0, 2, 10, -2, 3, 0, -5, -5, -4, -3, 2, 1, 1];
    let expected: (Option<i32>, Option<i32>) = (Some(-1), Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-1, 0, 1, -7];
    let expected: (Option<i32>, Option<i32>) = (Some(-1), Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![0, 8, 3];
    let expected: (Option<i32>, Option<i32>) = (None, Some(3));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![4, -4];
    let expected: (Option<i32>, Option<i32>) = (Some(-4), Some(4));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![0, -10];
    let expected: (Option<i32>, Option<i32>) = (Some(-10), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![0, 1, 2, 0, 5, 2, -3, -3];
    let expected: (Option<i32>, Option<i32>) = (Some(-3), Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-4, 3, 1, 0, 1];
    let expected: (Option<i32>, Option<i32>) = (Some(-4), Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![0];
    let expected: (Option<i32>, Option<i32>) = (None, None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![0, -3, 1, 4, -2];
    let expected: (Option<i32>, Option<i32>) = (Some(-2), Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![1];
    let expected: (Option<i32>, Option<i32>) = (None, Some(1));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![-2, 7, -3, 2, -1];
    let expected: (Option<i32>, Option<i32>) = (Some(-1), Some(2));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![-1];
    let expected: (Option<i32>, Option<i32>) = (Some(-1), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}