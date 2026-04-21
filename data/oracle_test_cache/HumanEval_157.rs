#[test]
fn oracle_case_0() {
    let arg_0: u32 = 640u32;
    let arg_1: u32 = 656u32;
    let arg_2: u32 = 144u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 1212;
    let arg_1: u32 = 1739;
    let arg_2: u32 = 4516;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 577u32;
    let arg_1: u32 = 48u32;
    let arg_2: u32 = 575u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 2693;
    let arg_1: u32 = 4004;
    let arg_2: u32 = 3209;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 1044u32;
    let arg_1: u32 = 720u32;
    let arg_2: u32 = 756u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 3492;
    let arg_1: u32 = 1408;
    let arg_2: u32 = 4202;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 648u32;
    let arg_1: u32 = 1377u32;
    let arg_2: u32 = 1215u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 333;
    let arg_1: u32 = 3747;
    let arg_2: u32 = 2348;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 152u32;
    let arg_1: u32 = 377u32;
    let arg_2: u32 = 345u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 4011;
    let arg_1: u32 = 4697;
    let arg_2: u32 = 476;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 480u32;
    let arg_1: u32 = 1832u32;
    let arg_2: u32 = 1768u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 3864;
    let arg_1: u32 = 3119;
    let arg_2: u32 = 341;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 765u32;
    let arg_1: u32 = 756u32;
    let arg_2: u32 = 117u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 446;
    let arg_1: u32 = 2724;
    let arg_2: u32 = 1551;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 324u32;
    let arg_1: u32 = 765u32;
    let arg_2: u32 = 693u32;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 504;
    let arg_1: u32 = 111;
    let arg_2: u32 = 3908;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 2262;
    let arg_1: u32 = 2425;
    let arg_2: u32 = 3875;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 3784;
    let arg_1: u32 = 2007;
    let arg_2: u32 = 4526;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 1063;
    let arg_1: u32 = 3986;
    let arg_2: u32 = 3306;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 4508;
    let arg_1: u32 = 3752;
    let arg_2: u32 = 2186;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 4255;
    let arg_1: u32 = 1263;
    let arg_2: u32 = 3776;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 3668;
    let arg_1: u32 = 4032;
    let arg_2: u32 = 4186;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 3718;
    let arg_1: u32 = 154;
    let arg_2: u32 = 623;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 3829;
    let arg_1: u32 = 3952;
    let arg_2: u32 = 1619;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}