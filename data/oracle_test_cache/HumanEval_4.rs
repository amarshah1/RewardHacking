#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317, -4174];
    let expected: u32 = 3163;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![-3063, -2713, 3574, 3640, 1331];
    let expected: u32 = 2753;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-3881, -4900, 176, -4043, -1667, -1758, 2153];
    let expected: u32 = 1959;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![2605, 2376, -199];
    let expected: u32 = 1195;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![-3445, -1724, 1626, 4803, 409, 2900, -3605, -1675, 3413, -1795];
    let expected: u32 = 2539;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![3746, 4610];
    let expected: u32 = 432;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-4671, -4417, -1662];
    let expected: u32 = 1280;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![650, -2147, 242];
    let expected: u32 = 1152;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-569];
    let expected: u32 = 0;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427, 2905, 2016, 1858, -4006, 1725, -3544, -2420, 2379, -1034];
    let expected: u32 = 2728;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![4482, -3155, 4700, -4235, 3376];
    let expected: u32 = 3783;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-3060, -1747, 1966, 617, 4778, -3432, -616, -3916, 3122, -469, -2817];
    let expected: u32 = 2281;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![537, -604, 3472, -4742];
    let expected: u32 = 2338;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![2668, -2392];
    let expected: u32 = 2530;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-1391, 3771, -4375, 1342];
    let expected: u32 = 2719;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![3766, 2853, -4602, -4828, -4074, 3171, -2434, 4461];
    let expected: u32 = 3773;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-3042];
    let expected: u32 = 0;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![1865];
    let expected: u32 = 0;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-454, 274, -420];
    let expected: u32 = 316;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![-2801];
    let expected: u32 = 0;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![1072, 546, 918, -1403, 4363, -1533, -2616];
    let expected: u32 = 1751;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![-1542];
    let expected: u32 = 0;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![-4852];
    let expected: u32 = 0;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![1052, -4793];
    let expected: u32 = 2922;
    assert_eq!(mean_absolute_deviation(&arg_0), expected);
}