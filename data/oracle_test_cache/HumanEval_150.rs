#[test]
fn oracle_case_0() {
    let arg_0: u32 = 3;
    let arg_1: i32 = 0;
    let arg_2: i32 = 0;
    let expected: i32 = 0;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 7;
    let arg_1: i32 = 3;
    let arg_2: i32 = 0;
    let expected: i32 = 3;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 4;
    let arg_1: i32 = 5;
    let arg_2: i32 = 0;
    let expected: i32 = 0;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 3;
    let arg_1: i32 = 0;
    let arg_2: i32 = 6;
    let expected: i32 = 0;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 8;
    let arg_1: i32 = -2;
    let arg_2: i32 = 4;
    let expected: i32 = 4;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 1;
    let arg_1: i32 = 11;
    let arg_2: i32 = -2;
    let expected: i32 = -2;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 1;
    let arg_1: i32 = -3;
    let arg_2: i32 = -4;
    let expected: i32 = -4;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 3;
    let arg_1: i32 = 2;
    let arg_2: i32 = 1;
    let expected: i32 = 2;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 2;
    let arg_1: i32 = -5;
    let arg_2: i32 = -1;
    let expected: i32 = -5;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 3;
    let arg_1: i32 = -1;
    let arg_2: i32 = -2;
    let expected: i32 = -1;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 3;
    let arg_1: i32 = 1;
    let arg_2: i32 = -2;
    let expected: i32 = 1;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 2;
    let arg_1: i32 = 4;
    let arg_2: i32 = -1;
    let expected: i32 = 4;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 5;
    let arg_1: i32 = 4;
    let arg_2: i32 = 3;
    let expected: i32 = 4;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 0;
    let arg_1: i32 = 1;
    let arg_2: i32 = -2;
    let expected: i32 = -2;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 1;
    let arg_1: i32 = 5;
    let arg_2: i32 = -3;
    let expected: i32 = -3;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 15;
    let arg_1: i32 = 4;
    let arg_2: i32 = 0;
    let expected: i32 = 0;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 1;
    let arg_1: i32 = -1;
    let arg_2: i32 = -3;
    let expected: i32 = -3;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 0;
    let arg_1: i32 = 3;
    let arg_2: i32 = -2;
    let expected: i32 = -2;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 0;
    let arg_1: i32 = 1;
    let arg_2: i32 = 0;
    let expected: i32 = 0;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 0;
    let arg_1: i32 = 0;
    let arg_2: i32 = 17;
    let expected: i32 = 17;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 2;
    let arg_1: i32 = -1;
    let arg_2: i32 = -1;
    let expected: i32 = -1;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 0;
    let arg_1: i32 = -1;
    let arg_2: i32 = 1;
    let expected: i32 = 1;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 1;
    let arg_1: i32 = 2;
    let arg_2: i32 = 10;
    let expected: i32 = 10;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 3;
    let arg_1: i32 = 3;
    let arg_2: i32 = 0;
    let expected: i32 = 3;
    assert_eq!(x_or_y(arg_0, arg_1, arg_2), expected);
}