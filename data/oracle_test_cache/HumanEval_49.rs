#[test]
fn oracle_case_0() {
    let arg_0: u32 = 0u32;
    let arg_1: u32 = 1u32;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 1u32;
    let arg_1: u32 = 1u32;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 0u32;
    let arg_1: u32 = 70u32;
    let expected: u32 = 1;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 1u32;
    let arg_1: u32 = 354u32;
    let expected: u32 = 2;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 425u32;
    let arg_1: u32 = 1u32;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 14u32;
    let arg_1: u32 = 64u32;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 96u32;
    let arg_1: u32 = 512u32;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 65u32;
    let arg_1: u32 = 2u32;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 87u32;
    let arg_1: u32 = 2u32;
    let expected: u32 = 0;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 1212;
    let arg_1: u32 = 1740;
    let expected: u32 = 256;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 4516;
    let arg_1: u32 = 2694;
    let expected: u32 = 694;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 4004;
    let arg_1: u32 = 3210;
    let expected: u32 = 946;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 3492;
    let arg_1: u32 = 1409;
    let expected: u32 = 551;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 4202;
    let arg_1: u32 = 334;
    let expected: u32 = 48;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 3747;
    let arg_1: u32 = 2349;
    let expected: u32 = 2330;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 4011;
    let arg_1: u32 = 4698;
    let expected: u32 = 3608;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 476;
    let arg_1: u32 = 3865;
    let expected: u32 = 2546;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 3119;
    let arg_1: u32 = 342;
    let expected: u32 = 32;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 446;
    let arg_1: u32 = 2725;
    let expected: u32 = 2214;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 1551;
    let arg_1: u32 = 505;
    let expected: u32 = 503;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 111;
    let arg_1: u32 = 3909;
    let expected: u32 = 2108;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 2262;
    let arg_1: u32 = 2426;
    let expected: u32 = 534;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 3875;
    let arg_1: u32 = 3785;
    let expected: u32 = 88;
    assert_eq!(modp(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 2007;
    let arg_1: u32 = 4527;
    let expected: u32 = 755;
    assert_eq!(modp(arg_0, arg_1), expected);
}