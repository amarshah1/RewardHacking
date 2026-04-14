#[test]
fn oracle_case_0() {
    let arg_0: Vec<u32> = vec![1, 1, 7];
    let expected: Vec<u64> = vec![1, 14];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u32> = vec![];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u32> = vec![1, 4, 6];
    let expected: Vec<u64> = vec![4, 12];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u32> = vec![3];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u32> = vec![7];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u32> = vec![3, 5, 1, 12, 3, 1, 0, 3];
    let expected: Vec<u64> = vec![5, 2, 36, 12, 5, 0, 21];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u32> = vec![3, 3, 2, 2, 6];
    let expected: Vec<u64> = vec![3, 4, 6, 24];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u32> = vec![3, 2];
    let expected: Vec<u64> = vec![2];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u32> = vec![2, 5, 2];
    let expected: Vec<u64> = vec![5, 4];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u32> = vec![5, 4, 0, 2, 0];
    let expected: Vec<u64> = vec![4, 0, 6, 0];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u32> = vec![1, 6];
    let expected: Vec<u64> = vec![6];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u32> = vec![15, 5, 1, 1];
    let expected: Vec<u64> = vec![5, 2, 3];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u32> = vec![4, 0];
    let expected: Vec<u64> = vec![0];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u32> = vec![3, 0, 2, 1];
    let expected: Vec<u64> = vec![0, 4, 3];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u32> = vec![2, 0, 1, 0, 1, 0, 2, 2, 1, 0, 2, 11, 3, 0, 3, 0, 0, 6];
    let expected: Vec<u64> = vec![0, 2, 0, 4, 0, 12, 14, 8, 0, 20, 121, 36, 0, 42, 0, 0, 102];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u32> = vec![0, 4, 4, 3, 2, 2];
    let expected: Vec<u64> = vec![4, 8, 9, 8, 10];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u32> = vec![2, 1, 0, 1];
    let expected: Vec<u64> = vec![1, 0, 3];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u32> = vec![3, 1, 9, 4, 2, 5, 5, 0];
    let expected: Vec<u64> = vec![1, 18, 12, 8, 25, 30, 0];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u32> = vec![1, 11];
    let expected: Vec<u64> = vec![11];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u32> = vec![1, 0, 1, 0, 2, 1, 6, 3];
    let expected: Vec<u64> = vec![0, 2, 0, 8, 5, 36, 21];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u32> = vec![0, 3, 0, 5];
    let expected: Vec<u64> = vec![3, 0, 15];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u32> = vec![4, 0, 1, 0];
    let expected: Vec<u64> = vec![0, 2, 0];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u32> = vec![1, 1];
    let expected: Vec<u64> = vec![1];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u32> = vec![0, 0, 0, 3, 0];
    let expected: Vec<u64> = vec![0, 0, 9, 0];
    assert_eq!(derivative(&arg_0), expected);
}