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
    let arg_0: u32 = 1686u32;
    let arg_1: u32 = 1272u32;
    let arg_2: u32 = 893u32;
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
    let arg_0: u32 = 1851u32;
    let arg_1: u32 = 288u32;
    let arg_2: u32 = 1098u32;
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
    let arg_0: u32 = 1118u32;
    let arg_1: u32 = 1746u32;
    let arg_2: u32 = 255u32;
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
    let arg_0: u32 = 450u32;
    let arg_1: u32 = 1679u32;
    let arg_2: u32 = 474u32;
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
    let arg_0: u32 = 1906u32;
    let arg_1: u32 = 1131u32;
    let arg_2: u32 = 388u32;
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
    let arg_0: u32 = 217u32;
    let arg_1: u32 = 1179u32;
    let arg_2: u32 = 1581u32;
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
    let arg_0: u32 = 1782u32;
    let arg_1: u32 = 954u32;
    let arg_2: u32 = 854u32;
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
    let arg_0: u32 = 1971u32;
    let arg_1: u32 = 434u32;
    let arg_2: u32 = 340u32;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 2;
    let arg_1: u32 = 0;
    let arg_2: u32 = 2;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 3;
    let arg_1: u32 = 1;
    let arg_2: u32 = 1;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 4;
    let arg_1: u32 = 0;
    let arg_2: u32 = 4;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 7;
    let arg_1: u32 = 0;
    let arg_2: u32 = 3;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 0;
    let arg_1: u32 = 6;
    let arg_2: u32 = 6;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 4;
    let arg_2: u32 = 6;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 0;
    let arg_1: u32 = 4;
    let arg_2: u32 = 4;
    let expected: bool = true;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 1;
    let arg_1: u32 = 3;
    let arg_2: u32 = 1;
    let expected: bool = false;
    assert_eq!(right_angle_triangle(arg_0, arg_1, arg_2), expected);
}