#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![4827i32, -730i32, -9618i32, -9618i32, -5004i32, 2237i32];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![-3691i32, -725i32, -9924i32, -1729i32, 7122i32, -7391i32, -7391i32, -1367i32, 1643i32];
    let expected: i32 = 5;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-2472i32, -2472i32];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-8301i32, -8301i32, 3912i32, 18i32];
    let expected: i32 = 3;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![-2149i32, -2149i32];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-4030i32, -593i32, -7367i32, -3831i32, -3831i32, 3201i32, -8924i32];
    let expected: i32 = 6;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-5994i32, -5994i32];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![4635i32, 4635i32, 2185i32];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let expected: i32 = 15;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-3063, -2713, 3574, 3640, 1331, 4432];
    let expected: i32 = 4;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-3881, -4900, 176, -4043];
    let expected: i32 = 3;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![2605, 2376, -199, 416, 2862, 296, -4084];
    let expected: i32 = 6;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-3445];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-3593, -1007];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-1675, 3413, -1795, -823];
    let expected: i32 = 2;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-4671, -4417, -1662, 3173, 4786];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-569, -3150, 718, -2576, 4077];
    let expected: i32 = 3;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427];
    let expected: i32 = 23;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![1858, -4006];
    let expected: i32 = 1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![611];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![3566, -4836, -3754, -2105];
    let expected: i32 = 1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![-1499];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![-2201, -1934];
    let expected: i32 = -1;
    assert_eq!(can_arrange(&arg_0), expected);
}