#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![0, -6, 3];
    let expected: i32 = 1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![3];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![0, -2, 6, -7, 4, 11];
    let expected: i32 = 3;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![0, -3, -4];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![2, 1, -5];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![2, -1];
    let expected: i32 = 1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-2, 1];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-1, 4, -4];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![3, -2, 0, 5, -3];
    let expected: i32 = 4;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![4, 0, -1, -3, -4, -2, -18, 1, 2, 10, 3, -5, -7, 8, -10];
    let expected: i32 = 14;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![0, 1, 2, 5, -3, -4, 3, -5];
    let expected: i32 = 7;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![1, 4, -2];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![0];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![1];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-2, 7, -3, 2, -1];
    let expected: i32 = 4;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-1];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![0, -2, 2];
    let expected: i32 = 1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![6, 0, 3, -3, -8, 2, 1];
    let expected: i32 = 6;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![-1, 2, 3, 0, -7, 18, -2, 4];
    let expected: i32 = 6;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![0, 15, -2];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![0, -7, 4, -2, 3, -6];
    let expected: i32 = 5;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![-1, 3, -5];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![5, 3, 2];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}