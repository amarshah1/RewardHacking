#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![1, 1, 7];
    let expected: Vec<u8> = vec![7, 1, 1];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![1, 4, 6];
    let expected: Vec<u8> = vec![6, 4, 1];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![3];
    let expected: Vec<u8> = vec![3];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![7];
    let expected: Vec<u8> = vec![7];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![3, 5, 1, 12, 3, 1, 0, 3];
    let expected: Vec<u8> = vec![3, 0, 1, 3, 12, 1, 5, 3];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![3, 3, 2, 2, 6];
    let expected: Vec<u8> = vec![6, 2, 2, 3, 3];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![3, 2];
    let expected: Vec<u8> = vec![2, 3];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![2, 5, 2];
    let expected: Vec<u8> = vec![2, 5, 2];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![5, 4, 0, 2, 0];
    let expected: Vec<u8> = vec![0, 2, 0, 4, 5];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![1, 6];
    let expected: Vec<u8> = vec![6, 1];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![15, 5, 1, 1];
    let expected: Vec<u8> = vec![1, 1, 5, 15];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![4, 0];
    let expected: Vec<u8> = vec![0, 4];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![3, 0, 2, 1];
    let expected: Vec<u8> = vec![1, 2, 0, 3];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![2, 0, 1, 0, 1, 0, 2, 2, 1, 0, 2, 11, 3, 0, 3, 0, 0, 6];
    let expected: Vec<u8> = vec![6, 0, 0, 3, 0, 3, 11, 2, 0, 1, 2, 2, 0, 1, 0, 1, 0, 2];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![0, 4, 4, 3, 2, 2];
    let expected: Vec<u8> = vec![2, 2, 3, 4, 4, 0];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![2, 1, 0, 1];
    let expected: Vec<u8> = vec![1, 0, 1, 2];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![3, 1, 9, 4, 2, 5, 5, 0];
    let expected: Vec<u8> = vec![0, 5, 5, 2, 4, 9, 1, 3];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![1, 11];
    let expected: Vec<u8> = vec![11, 1];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![1, 0, 1, 0, 2, 1, 6, 3];
    let expected: Vec<u8> = vec![3, 6, 1, 2, 0, 1, 0, 1];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![0, 3, 0, 5];
    let expected: Vec<u8> = vec![5, 0, 3, 0];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![4, 0, 1, 0];
    let expected: Vec<u8> = vec![0, 1, 0, 4];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![1, 1];
    let expected: Vec<u8> = vec![1, 1];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![0, 0, 0, 3, 0];
    let expected: Vec<u8> = vec![0, 3, 0, 0, 0];
    assert_eq!(solve(&arg_0), expected);
}