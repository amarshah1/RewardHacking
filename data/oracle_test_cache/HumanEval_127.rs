#[test]
fn oracle_case_0() {
    let arg_0: (i32, i32) = (1212, 4516);
    let arg_1: (i32, i32) = (-4004, -3492);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: (i32, i32) = (-4202, 3747);
    let arg_1: (i32, i32) = (476, 4011);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: (i32, i32) = (-3119, 446);
    let arg_1: (i32, i32) = (111, 1551);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: (i32, i32) = (2262, 3875);
    let arg_1: (i32, i32) = (-2007, -1063);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: (i32, i32) = (-3306, 3752);
    let arg_1: (i32, i32) = (-4255, -3776);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: (i32, i32) = (3718, 4032);
    let arg_1: (i32, i32) = (-623, 3952);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: (i32, i32) = (-4881, 4899);
    let arg_1: (i32, i32) = (-4629, -3851);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: (i32, i32) = (-4174, -3317);
    let arg_1: (i32, i32) = (-3132, 1491);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: (i32, i32) = (-2143, 2508);
    let arg_1: (i32, i32) = (-1380, 2001);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: (i32, i32) = (-4551, 2044);
    let arg_1: (i32, i32) = (-416, 1589);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: (i32, i32) = (-3881, 435);
    let arg_1: (i32, i32) = (-4900, 176);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: (i32, i32) = (-4043, -1667);
    let arg_1: (i32, i32) = (-1758, 2153);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: (i32, i32) = (2877, 3543);
    let arg_1: (i32, i32) = (-921, 1176);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: (i32, i32) = (2793, 4382);
    let arg_1: (i32, i32) = (-2794, 1580);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: (i32, i32) = (-3322, 3048);
    let arg_1: (i32, i32) = (-3837, -3742);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: (i32, i32) = (-3593, -1007);
    let arg_1: (i32, i32) = (-4636, 3022);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: (i32, i32) = (-908, 2587);
    let arg_1: (i32, i32) = (-2973, 717);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: (i32, i32) = (-1698, -584);
    let arg_1: (i32, i32) = (-1558, 166);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: (i32, i32) = (-4671, -4417);
    let arg_1: (i32, i32) = (-1662, 3173);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: (i32, i32) = (3419, 4786);
    let arg_1: (i32, i32) = (-3846, 317);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: (i32, i32) = (-3150, -569);
    let arg_1: (i32, i32) = (-2576, 718);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: (i32, i32) = (2567, 4077);
    let arg_1: (i32, i32) = (-3431, 3904);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: (i32, i32) = (-2866, 2128);
    let arg_1: (i32, i32) = (-1884, 4273);
    let expected: &'static str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: (i32, i32) = (1782, 3614);
    let arg_1: (i32, i32) = (-1844, 3949);
    let expected: &'static str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}