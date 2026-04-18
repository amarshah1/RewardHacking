#[test]
fn oracle_case_0() {
    let arg_0: u8 = 4;
    let expected: Vec<u8> = vec![2, 2];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u8 = 239;
    let expected: Vec<u8> = vec![239];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u8 = 219;
    let expected: Vec<u8> = vec![3, 73];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u8 = 196;
    let expected: Vec<u8> = vec![2, 2, 7, 7];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u8 = 213;
    let expected: Vec<u8> = vec![3, 71];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u8 = 142;
    let expected: Vec<u8> = vec![2, 71];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u8 = 67;
    let expected: Vec<u8> = vec![67];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u8 = 222;
    let expected: Vec<u8> = vec![2, 3, 37];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u8 = 82;
    let expected: Vec<u8> = vec![2, 41];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u8 = 155;
    let expected: Vec<u8> = vec![5, 31];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u8 = 220;
    let expected: Vec<u8> = vec![2, 2, 5, 11];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u8 = 39;
    let expected: Vec<u8> = vec![3, 13];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u8 = 75;
    let expected: Vec<u8> = vec![3, 5, 5];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u8 = 246;
    let expected: Vec<u8> = vec![2, 3, 41];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u8 = 240;
    let expected: Vec<u8> = vec![2, 2, 2, 2, 3, 5];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u8 = 154;
    let expected: Vec<u8> = vec![2, 7, 11];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u8 = 244;
    let expected: Vec<u8> = vec![2, 2, 61];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u8 = 226;
    let expected: Vec<u8> = vec![2, 113];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u8 = 184;
    let expected: Vec<u8> = vec![2, 2, 2, 23];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u8 = 97;
    let expected: Vec<u8> = vec![97];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u8 = 106;
    let expected: Vec<u8> = vec![2, 53];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u8 = 146;
    let expected: Vec<u8> = vec![2, 73];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u8 = 138;
    let expected: Vec<u8> = vec![2, 3, 23];
    assert_eq!(factorize(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u8 = 84;
    let expected: Vec<u8> = vec![2, 2, 3, 7];
    assert_eq!(factorize(arg_0), expected);
}