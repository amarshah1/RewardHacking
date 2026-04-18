#[test]
fn oracle_case_0() {
    let arg_0: u32 = 2147482435;
    let arg_1: u32 = 2147481908;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 2147479131;
    let arg_1: u32 = 2147480954;
    let expected: i32 = 2147480954;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 2147479643;
    let arg_1: u32 = 2147480438;
    let expected: i32 = 2147480438;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 2147480155;
    let arg_1: u32 = 2147482239;
    let expected: i32 = 2147482238;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 2147479445;
    let arg_1: u32 = 2147483314;
    let expected: i32 = 2147483314;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 2147479900;
    let arg_1: u32 = 2147481299;
    let expected: i32 = 2147481298;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 2147479636;
    let arg_1: u32 = 2147478950;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 2147483171;
    let arg_1: u32 = 2147479783;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 2147480528;
    let arg_1: u32 = 2147483306;
    let expected: i32 = 2147483306;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 2147483201;
    let arg_1: u32 = 2147480923;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 2147482096;
    let arg_1: u32 = 2147483143;
    let expected: i32 = 2147483142;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 2147483536;
    let arg_1: u32 = 2147479739;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 2147481385;
    let arg_1: u32 = 2147481222;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 2147479772;
    let arg_1: u32 = 2147479863;
    let expected: i32 = 2147479862;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 2147481640;
    let arg_1: u32 = 2147479121;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 2147482584;
    let arg_1: u32 = 2147479661;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 2147480341;
    let arg_1: u32 = 2147479139;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 2147479895;
    let arg_1: u32 = 2147481461;
    let expected: i32 = 2147481460;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 2147479392;
    let arg_1: u32 = 2147482384;
    let expected: i32 = 2147482384;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 2147479871;
    let arg_1: u32 = 2147479979;
    let expected: i32 = 2147479978;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 2147479615;
    let arg_1: u32 = 2147479461;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 2147479929;
    let arg_1: u32 = 2147483493;
    let expected: i32 = 2147483492;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 2147483024;
    let arg_1: u32 = 2147479818;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 2147479695;
    let arg_1: u32 = 2147482028;
    let expected: i32 = 2147482028;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}