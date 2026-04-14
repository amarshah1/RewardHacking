#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![1, 1, 7];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![1, 4, 6];
    let arg_1: Vec<u8> = vec![3];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![7];
    let arg_1: Vec<u8> = vec![3, 5, 1, 12, 3, 1, 0, 3];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![3, 3, 2, 2, 6];
    let arg_1: Vec<u8> = vec![3, 2];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![];
    let arg_1: Vec<u8> = vec![3, 2];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![2, 5, 2];
    let arg_1: Vec<u8> = vec![5, 4, 0, 2, 0];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![1, 6];
    let arg_1: Vec<u8> = vec![15, 5, 1, 1];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![4, 0];
    let arg_1: Vec<u8> = vec![3, 0, 2, 1];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![];
    let arg_1: Vec<u8> = vec![2, 0, 1, 0, 1, 0, 2, 2, 1, 0, 2, 11, 3, 0, 3, 0, 0, 6];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![0, 4, 4, 3, 2, 2];
    let arg_1: Vec<u8> = vec![2, 1, 0, 1];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![3, 1, 9, 4, 2, 5, 5, 0];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![1, 11];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![];
    let arg_1: Vec<u8> = vec![1, 0, 1, 0, 2, 1, 6, 3];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![0, 3, 0, 5];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![4, 0, 1, 0];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![1, 1];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![0, 0, 0, 3, 0];
    let arg_1: Vec<u8> = vec![0];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![0, 2, 1, 0];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![2];
    let arg_1: Vec<u8> = vec![0, 2, 8, 4, 3];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![0, 0];
    let arg_1: Vec<u8> = vec![2];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![0, 0, 1];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![];
    let arg_1: Vec<u8> = vec![3, 7, 7];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![4];
    let arg_1: Vec<u8> = vec![1, 9, 3, 2];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}