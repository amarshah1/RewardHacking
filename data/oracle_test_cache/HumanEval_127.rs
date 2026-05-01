#[test]
fn oracle_case_0() {
    let arg_0: (i32, i32) = (-88315, 1390008);
    let arg_1: (i32, i32) = (-88954, 911667);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: (i32, i32) = (52173, 1347876);
    let arg_1: (i32, i32) = (51330, 1052151);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: (i32, i32) = (-74292, 938023);
    let arg_1: (i32, i32) = (-74323, 925668);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: (i32, i32) = (-29691, 1463163);
    let arg_1: (i32, i32) = (-29899, 970267);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: (i32, i32) = (6987, 194311);
    let arg_1: (i32, i32) = (6680, 164595);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: (i32, i32) = (76883, 271955);
    let arg_1: (i32, i32) = (76492, 231315);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: (i32, i32) = (-83700, 109338);
    let arg_1: (i32, i32) = (-84001, 68350);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: (i32, i32) = (39961, 207573);
    let arg_1: (i32, i32) = (39000, 191281);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: (i32, i32) = (-873, 4565);
    let arg_1: (i32, i32) = (-1699, 4125);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: (i32, i32) = (-76435, -69018);
    let arg_1: (i32, i32) = (-76815, -71443);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: (i32, i32) = (-8357, -3283);
    let arg_1: (i32, i32) = (-8842, -3371);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: (i32, i32) = (-53611, -46153);
    let arg_1: (i32, i32) = (-54340, -48639);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: (i32, i32) = (-2332, -2212);
    let arg_1: (i32, i32) = (-3039, -2236);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: (i32, i32) = (-25838, -25743);
    let arg_1: (i32, i32) = (-26356, -25750);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: (i32, i32) = (-68069, -67971);
    let arg_1: (i32, i32) = (-68702, -67987);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: (i32, i32) = (-74640, -74546);
    let arg_1: (i32, i32) = (-75020, -74562);
    let expected: &str = "YES";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: (i32, i32) = (37432, 37432);
    let arg_1: (i32, i32) = (37164, 37432);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: (i32, i32) = (1212, 4516);
    let arg_1: (i32, i32) = (-4004, -3492);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: (i32, i32) = (-4202, 3747);
    let arg_1: (i32, i32) = (476, 4011);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: (i32, i32) = (-3119, 446);
    let arg_1: (i32, i32) = (111, 1551);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: (i32, i32) = (2262, 3875);
    let arg_1: (i32, i32) = (-2007, -1063);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: (i32, i32) = (-3306, 3752);
    let arg_1: (i32, i32) = (-4255, -3776);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: (i32, i32) = (3718, 4032);
    let arg_1: (i32, i32) = (-623, 3952);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: (i32, i32) = (-4881, 4899);
    let arg_1: (i32, i32) = (-4629, -3851);
    let expected: &str = "NO";
    assert_eq!(intersection(arg_0, arg_1), expected);
}