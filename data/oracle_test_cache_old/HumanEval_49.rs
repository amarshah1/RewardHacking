#[test]
fn oracle_case_0() {
    let arg_0: u32 = 3;
    let arg_1: u32 = 2;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 8;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 0;
    let arg_1: u32 = 4;
    let expected: u32 = 1;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 5;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 6;
    let arg_1: u32 = 2;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 7;
    let arg_1: u32 = 9;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 3;
    let arg_1: u32 = 6;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 13;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 5;
    let arg_1: u32 = 4;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 3;
    let arg_1: u32 = 3;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 2;
    let arg_1: u32 = 7;
    let expected: u32 = 4;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 2;
    let arg_1: u32 = 4;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 2;
    let arg_1: u32 = 1;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 2;
    let arg_1: u32 = 6;
    let expected: u32 = 4;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 5;
    let arg_1: u32 = 5;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 0;
    let arg_1: u32 = 3;
    let expected: u32 = 1;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 7;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 4;
    let arg_1: u32 = 16;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 5;
    let arg_1: u32 = 2;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 3;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 4;
    let arg_1: u32 = 1;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 4;
    let arg_1: u32 = 4;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 1;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 0;
    let arg_1: u32 = 1;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}