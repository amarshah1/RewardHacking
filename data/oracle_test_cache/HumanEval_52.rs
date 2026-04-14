#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![0, 0, -6];
    let arg_1: i32 = 3;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![3];
    let arg_1: i32 = 5;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-2];
    let arg_1: i32 = 0;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-7, -2, 4, 0, 11, -2, 0];
    let arg_1: i32 = -3;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![2, 2, 1, 1, -5];
    let arg_1: i32 = -1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-1, -2, -2];
    let arg_1: i32 = 1;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-1, 4, -1];
    let arg_1: i32 = -4;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![3, -2, -2, 0, 5];
    let arg_1: i32 = -3;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![4, 0, 0, -1, -3, -4, -2, -2, 0, 0, -18, 1, -1, -1, 2];
    let arg_1: i32 = 1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![2];
    let arg_1: i32 = 10;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![3, 0, -5];
    let arg_1: i32 = -5;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = 3;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![2, 1, 1, -3];
    let arg_1: i32 = -1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![1];
    let arg_1: i32 = -7;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![0, 8, 3];
    let arg_1: i32 = -1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-4, 0, 1, 0, -10];
    let arg_1: i32 = 0;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![0, 1, 2, 0, 5, 2, -3, -3];
    let arg_1: i32 = 5;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = -3;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![1, 0, 1, 0];
    let arg_1: i32 = 0;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = 4;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = 0;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![1, 4, -2];
    let arg_1: i32 = 0;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![4, -2];
    let arg_1: i32 = 7;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![2, -1, 0, 0];
    let arg_1: i32 = -1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}