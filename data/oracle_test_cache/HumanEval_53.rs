#[test]
fn oracle_case_0() {
    let arg_0: i32 = 2;
    let arg_1: i32 = 0;
    let expected: Option<i32> = Some(2);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: i32 = 2147483647i32;
    let arg_1: i32 = 2147483647i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: i32 = 0;
    let arg_1: i32 = -6;
    let expected: Option<i32> = Some(-6);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: i32 = 2147483647i32;
    let arg_1: i32 = 1i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: i32 = 3;
    let arg_1: i32 = 0;
    let expected: Option<i32> = Some(3);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: i32 = -2147483648i32;
    let arg_1: i32 = -2147483648i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: i32 = 3;
    let arg_1: i32 = 5;
    let expected: Option<i32> = Some(8);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: i32 = -2147483648i32;
    let arg_1: i32 = -1i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: i32 = 0;
    let arg_1: i32 = -2;
    let expected: Option<i32> = Some(-2);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: i32 = 2147481126i32;
    let arg_1: i32 = 6061i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: i32 = 0;
    let arg_1: i32 = 6;
    let expected: Option<i32> = Some(6);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: i32 = 2147479556i32;
    let arg_1: i32 = 12620i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: i32 = -7;
    let arg_1: i32 = -2;
    let expected: Option<i32> = Some(-9);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: i32 = 9910i32;
    let arg_1: i32 = 2147474249i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: i32 = 4;
    let arg_1: i32 = 0;
    let expected: Option<i32> = Some(4);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: i32 = 2147475815i32;
    let arg_1: i32 = 13465i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: i32 = 11;
    let arg_1: i32 = -2;
    let expected: Option<i32> = Some(9);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: i32 = -3691i32;
    let arg_1: i32 = -2147480876i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: i32 = 0;
    let arg_1: i32 = -3;
    let expected: Option<i32> = Some(-3);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: i32 = -2147479895i32;
    let arg_1: i32 = -9656i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: i32 = -4;
    let arg_1: i32 = 2;
    let expected: Option<i32> = Some(-2);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: i32 = -6996i32;
    let arg_1: i32 = -2147479900i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: i32 = 2;
    let arg_1: i32 = 1;
    let expected: Option<i32> = Some(3);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: i32 = -2147477894i32;
    let arg_1: i32 = -8019i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}