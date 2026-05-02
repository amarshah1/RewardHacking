#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![76i32, 70i32, 17i32, 48i32, 78i32, 61i32, 81i32, 75i32, 9i32, 78i32, 2i32, 61i32, 34i32, 71i32, 30i32, 25i32, 92i32, 61i32, 70i32, -1110i32, 0i32, 31i32, -31i32];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![19i32, 40i32, 2i32, 40i32, 42i32, 14i32, -174i32, 13i32, 5i32, 3i32, 21i32];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![2605, 2376, -199, 416, 2862, 296, -4084];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![611];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-3063, -2713, 3574, 3640, 1331, 4432];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![1865];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-3881, -4900, 176, -4043];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![1072, 546, 918, -1403];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-3445];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![270, 564, 3822, -233, -2466, 4730, -1760];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![-3593, -1007];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![3280, 1888, 3641, -1669, -4342, 967, 971, 564, -714, 2457, -3137, 1004, -2618, -570, 4691, -4149, 4945];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-1675, 3413, -1795, -823];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![130, 695];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-4671, -4417, -1662, 3173, 4786];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![999];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-569, -3150, 718, -2576, 4077];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![1901];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![1042, -562, -375, 3382, 2751, 156, 1096];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![1858, -4006];
    let expected: bool = true;
    assert_eq!(below_zero(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![2438, 4957, -179, -4180, 4908, 231, 4963, 4287];
    let expected: bool = false;
    assert_eq!(below_zero(arg_0), expected);
}