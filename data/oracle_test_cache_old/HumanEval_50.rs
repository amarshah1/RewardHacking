#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![66, 66, 72];
    let expected: Vec<u8> = vec![87, 87, 67];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![66, 69, 71];
    let expected: Vec<u8> = vec![87, 90, 66];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![68];
    let expected: Vec<u8> = vec![89];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![72];
    let expected: Vec<u8> = vec![67];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![68, 70, 66, 77, 68, 66, 65, 68];
    let expected: Vec<u8> = vec![89, 65, 87, 72, 89, 87, 86, 89];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![68, 68, 67, 67, 71];
    let expected: Vec<u8> = vec![89, 89, 88, 88, 66];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![68, 67];
    let expected: Vec<u8> = vec![89, 88];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![67, 70, 67];
    let expected: Vec<u8> = vec![88, 65, 88];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![70, 69, 65, 67, 65];
    let expected: Vec<u8> = vec![65, 90, 86, 88, 86];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![66, 71];
    let expected: Vec<u8> = vec![87, 66];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![80, 70, 66, 66];
    let expected: Vec<u8> = vec![75, 65, 87, 87];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![69, 65];
    let expected: Vec<u8> = vec![90, 86];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![68, 65, 67, 66];
    let expected: Vec<u8> = vec![89, 86, 88, 87];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![67, 65, 66, 65, 66, 65, 67, 67, 66, 65, 67, 76, 68, 65, 68, 65, 65, 71];
    let expected: Vec<u8> = vec![88, 86, 87, 86, 87, 86, 88, 88, 87, 86, 88, 71, 89, 86, 89, 86, 86, 66];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![65, 69, 69, 68, 67, 67];
    let expected: Vec<u8> = vec![86, 90, 90, 89, 88, 88];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![67, 66, 65, 66];
    let expected: Vec<u8> = vec![88, 87, 86, 87];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![68, 66, 74, 69, 67, 70, 70, 65];
    let expected: Vec<u8> = vec![89, 87, 69, 90, 88, 65, 65, 86];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![66, 76];
    let expected: Vec<u8> = vec![87, 71];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![66, 65, 66, 65, 67, 66, 71, 68];
    let expected: Vec<u8> = vec![87, 86, 87, 86, 88, 87, 66, 89];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![65, 68, 65, 70];
    let expected: Vec<u8> = vec![86, 89, 86, 65];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![69, 65, 66, 65];
    let expected: Vec<u8> = vec![90, 86, 87, 86];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![66, 66];
    let expected: Vec<u8> = vec![87, 87];
    assert_eq!(decode_shift(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![65, 65, 65, 68, 65];
    let expected: Vec<u8> = vec![86, 86, 86, 89, 86];
    assert_eq!(decode_shift(&arg_0), expected);
}