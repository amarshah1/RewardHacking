#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![0, 0, -6];
    let arg_1: Vec<i32> = vec![3, 0, 3];
    let expected: Vec<i64> = vec![3, 0, 9];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![0, -2, 0, 6, -7, -2];
    let arg_1: Vec<i32> = vec![4, 0, 11, -2, 0, -3];
    let expected: Vec<i64> = vec![4, 2, 11, 8, 7, 1];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![2, 2, 1, 1, -5];
    let arg_1: Vec<i32> = vec![-1, 2, -1, -2, -2];
    let expected: Vec<i64> = vec![3, 0, 2, 3, 3];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-2, -1];
    let arg_1: Vec<i32> = vec![4, -1];
    let expected: Vec<i64> = vec![6, 0];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![4, 3, -2, -2, 0];
    let arg_1: Vec<i32> = vec![5, -3, 14, 4, 0];
    let expected: Vec<i64> = vec![1, 6, 16, 6, 0];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-1];
    let arg_1: Vec<i32> = vec![-3];
    let expected: Vec<i64> = vec![2];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: Vec<i32> = vec![];
    let expected: Vec<i64> = vec![];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-2, -2, 0, 0];
    let arg_1: Vec<i32> = vec![-18, 1, -1, -1];
    let expected: Vec<i64> = vec![16, 3, 1, 1];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![1, 0];
    let arg_1: Vec<i32> = vec![2, 10];
    let expected: Vec<i64> = vec![1, 10];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![3, 0, -5];
    let arg_1: Vec<i32> = vec![-5, -4, -3];
    let expected: Vec<i64> = vec![8, 4, 2];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![1, 1, -3];
    let arg_1: Vec<i32> = vec![-1, 0, 1];
    let expected: Vec<i64> = vec![2, 1, 4];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-2, 0, 8, 3, -1, 4, -4, 0];
    let arg_1: Vec<i32> = vec![1, 0, -10, 0, -7, 0, 1, 2];
    let expected: Vec<i64> = vec![3, 0, 18, 3, 6, 4, 5, 2];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![5];
    let arg_1: Vec<i32> = vec![2];
    let expected: Vec<i64> = vec![3];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-3, 5, -4, 3];
    let arg_1: Vec<i32> = vec![1, 0, 1, 0];
    let expected: Vec<i64> = vec![4, 5, 5, 3];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-5];
    let arg_1: Vec<i32> = vec![0];
    let expected: Vec<i64> = vec![5];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![1, 4, -2];
    let arg_1: Vec<i32> = vec![0, 0, 0];
    let expected: Vec<i64> = vec![1, 4, 2];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![1];
    let arg_1: Vec<i32> = vec![4];
    let expected: Vec<i64> = vec![3];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![7, -3];
    let arg_1: Vec<i32> = vec![2, -1];
    let expected: Vec<i64> = vec![5, 2];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-1];
    let arg_1: Vec<i32> = vec![2];
    let expected: Vec<i64> = vec![3];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![0];
    let arg_1: Vec<i32> = vec![-2];
    let expected: Vec<i64> = vec![2];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![6, 6, 0];
    let arg_1: Vec<i32> = vec![3, -3, 0];
    let expected: Vec<i64> = vec![3, 9, 0];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![2, 1, 7, -1, 2, 3, 0, -7, 0];
    let arg_1: Vec<i32> = vec![18, -2, 3, 4, 3, 0, 15, -2, -5];
    let expected: Vec<i64> = vec![16, 3, 4, 5, 1, 3, 15, 5, 5];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![0];
    let arg_1: Vec<i32> = vec![-7];
    let expected: Vec<i64> = vec![7];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![4];
    let arg_1: Vec<i32> = vec![-2];
    let expected: Vec<i64> = vec![6];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}