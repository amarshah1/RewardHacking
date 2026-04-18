#[test]
fn oracle_case_0() {
    let arg_0: (i32, i32) = (0, 2);
    let arg_1: (i32, i32) = (-6, 0);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: (i32, i32) = (0, 3);
    let arg_1: (i32, i32) = (3, 5);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: (i32, i32) = (-2, 0);
    let arg_1: (i32, i32) = (0, 6);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: (i32, i32) = (-7, -2);
    let arg_1: (i32, i32) = (0, 4);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: (i32, i32) = (-2, 11);
    let arg_1: (i32, i32) = (-3, 0);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: (i32, i32) = (-4, 2);
    let arg_1: (i32, i32) = (1, 2);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: (i32, i32) = (-5, 1);
    let arg_1: (i32, i32) = (-1, 2);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: (i32, i32) = (-2, -1);
    let arg_1: (i32, i32) = (-2, 1);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: (i32, i32) = (-2, -1);
    let arg_1: (i32, i32) = (-1, 4);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: (i32, i32) = (-4, 4);
    let arg_1: (i32, i32) = (-2, 3);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: (i32, i32) = (-2, 0);
    let arg_1: (i32, i32) = (-3, 5);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: (i32, i32) = (4, 14);
    let arg_1: (i32, i32) = (0, 0);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: (i32, i32) = (-3, -1);
    let arg_1: (i32, i32) = (-4, -2);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: (i32, i32) = (-2, 0);
    let arg_1: (i32, i32) = (-18, 0);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: (i32, i32) = (-1, 1);
    let arg_1: (i32, i32) = (-1, 2);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: (i32, i32) = (0, 1);
    let arg_1: (i32, i32) = (2, 10);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: (i32, i32) = (-2, 3);
    let arg_1: (i32, i32) = (-5, 0);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: (i32, i32) = (-5, -4);
    let arg_1: (i32, i32) = (-3, 2);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: (i32, i32) = (1, 1);
    let arg_1: (i32, i32) = (-3, -1);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: (i32, i32) = (0, 1);
    let arg_1: (i32, i32) = (-7, -2);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: (i32, i32) = (0, 8);
    let arg_1: (i32, i32) = (-1, 3);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: (i32, i32) = (-4, 4);
    let arg_1: (i32, i32) = (0, 1);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: (i32, i32) = (-10, 0);
    let arg_1: (i32, i32) = (-7, 0);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: (i32, i32) = (0, 1);
    let arg_1: (i32, i32) = (0, 2);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}