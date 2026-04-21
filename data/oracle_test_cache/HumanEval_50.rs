#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![77, 83, 67, 84, 82, 66, 69, 77, 85, 72, 85, 75, 78, 65, 84, 65, 88, 80, 82];
    let expected: Vec<u8> = vec![72, 78, 88, 79, 77, 87, 90, 72, 80, 67, 80, 70, 73, 86, 79, 86, 83, 75, 77];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![88, 76, 69, 73, 66, 74];
    let expected: Vec<u8> = vec![83, 71, 90, 68, 87, 69];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![70, 81, 68, 75];
    let expected: Vec<u8> = vec![65, 76, 89, 70];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![69, 65, 86, 89, 73, 81, 89];
    let expected: Vec<u8> = vec![90, 86, 81, 84, 68, 76, 84];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![71];
    let expected: Vec<u8> = vec![66];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![68, 67];
    let expected: Vec<u8> = vec![89, 88];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![83, 72, 73, 74];
    let expected: Vec<u8> = vec![78, 67, 68, 69];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![72, 84, 76, 67, 78];
    let expected: Vec<u8> = vec![67, 79, 71, 88, 73];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![88, 85, 80, 74, 65];
    let expected: Vec<u8> = vec![83, 80, 75, 69, 86];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![84, 81, 77, 84, 66, 72, 88, 77, 76, 68, 66, 89, 74, 90, 89, 84, 67, 86, 83, 77, 66, 87, 82, 69];
    let expected: Vec<u8> = vec![79, 76, 72, 79, 87, 67, 83, 72, 71, 89, 87, 84, 69, 85, 84, 79, 88, 81, 78, 72, 87, 82, 77, 90];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![66, 89];
    let expected: Vec<u8> = vec![87, 84];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![76];
    let expected: Vec<u8> = vec![71];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![77, 82, 76, 80];
    let expected: Vec<u8> = vec![72, 77, 71, 75];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![74];
    let expected: Vec<u8> = vec![69];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![67, 66];
    let expected: Vec<u8> = vec![88, 87];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![78, 87, 68, 82, 80, 80];
    let expected: Vec<u8> = vec![73, 82, 89, 77, 75, 75];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![89, 85];
    let expected: Vec<u8> = vec![84, 80];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![73, 77, 75, 77, 88, 82];
    let expected: Vec<u8> = vec![68, 72, 70, 72, 83, 77];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![78, 76, 65];
    let expected: Vec<u8> = vec![73, 71, 86];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![65, 69, 71, 78, 67, 74, 84, 69];
    let expected: Vec<u8> = vec![86, 90, 66, 73, 88, 69, 79, 90];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![69];
    let expected: Vec<u8> = vec![90];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![85, 77];
    let expected: Vec<u8> = vec![80, 72];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![76, 80];
    let expected: Vec<u8> = vec![71, 75];
    assert_eq!(decode_shift(&arg_0), expected);
}