#[test]
fn oracle_case_0() {
    let arg_0: i32 = 1212;
    let arg_1: i32 = 4516;
    let expected: Option<i32> = Some(5728);
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
    let arg_0: i32 = -4004;
    let arg_1: i32 = -3492;
    let expected: Option<i32> = Some(-7496);
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
    let arg_0: i32 = -4202;
    let arg_1: i32 = 3747;
    let expected: Option<i32> = Some(-455);
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
    let arg_0: i32 = 4011;
    let arg_1: i32 = 476;
    let expected: Option<i32> = Some(4487);
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
    let arg_0: i32 = -3119;
    let arg_1: i32 = 446;
    let expected: Option<i32> = Some(-2673);
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
    let arg_0: i32 = 1551;
    let arg_1: i32 = 111;
    let expected: Option<i32> = Some(1662);
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
    let arg_0: i32 = 2262;
    let arg_1: i32 = 3875;
    let expected: Option<i32> = Some(6137);
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
    let arg_0: i32 = -2007;
    let arg_1: i32 = -1063;
    let expected: Option<i32> = Some(-3070);
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
    let arg_0: i32 = -3306;
    let arg_1: i32 = 3752;
    let expected: Option<i32> = Some(446);
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
    let arg_0: i32 = -4255;
    let arg_1: i32 = -3776;
    let expected: Option<i32> = Some(-8031);
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
    let arg_0: i32 = 4032;
    let arg_1: i32 = 3718;
    let expected: Option<i32> = Some(7750);
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
    let arg_0: i32 = -623;
    let arg_1: i32 = 3952;
    let expected: Option<i32> = Some(3329);
    assert_eq!(add(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: i32 = -2147477894i32;
    let arg_1: i32 = -8019i32;
    let expected: Option<i32> = None;
    assert_eq!(add(arg_0, arg_1), expected);
}