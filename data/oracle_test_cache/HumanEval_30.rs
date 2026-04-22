#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![-1820i32, -497i32, 3181i32, -766i32, 162i32, 311i32, -1409i32, 2954i32, 3388i32, -2136i32, 0i32];
    let expected: Vec<i32> = vec![3181, 162, 311, 2954, 3388];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![-4150i32, -1159i32, -2653i32, 4253i32, 1233i32, 2677i32, -335i32, -4243i32, 1567i32, -3966i32, 0i32];
    let expected: Vec<i32> = vec![4253, 1233, 2677, 1567];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let expected: Vec<i32> = vec![1551, 111, 2262, 3875, 3752, 4032, 3718, 3952, 4899];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-3063, -2713, 3574, 3640, 1331, 4432];
    let expected: Vec<i32> = vec![3574, 3640, 1331, 4432];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![-3881, -4900, 176, -4043];
    let expected: Vec<i32> = vec![176];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![2605, 2376, -199, 416, 2862, 296, -4084];
    let expected: Vec<i32> = vec![2605, 2376, 416, 2862, 296];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-3445];
    let expected: Vec<i32> = vec![];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-3593, -1007];
    let expected: Vec<i32> = vec![];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-1675, 3413, -1795, -823];
    let expected: Vec<i32> = vec![3413];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-4671, -4417, -1662, 3173, 4786];
    let expected: Vec<i32> = vec![3173, 4786];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-569, -3150, 718, -2576, 4077];
    let expected: Vec<i32> = vec![718, 4077];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427];
    let expected: Vec<i32> = vec![3810, 2151, 2725, 4194, 1565, 4375, 3631, 4438, 2563, 2864, 871, 1853];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![1858, -4006];
    let expected: Vec<i32> = vec![1858];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![611];
    let expected: Vec<i32> = vec![611];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![3566, -4836, -3754, -2105];
    let expected: Vec<i32> = vec![3566];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-1499];
    let expected: Vec<i32> = vec![];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-2201, -1934];
    let expected: Vec<i32> = vec![];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-3060, -1747, 1966, 617, 4778, -3432];
    let expected: Vec<i32> = vec![1966, 617, 4778];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![-4850, -4056];
    let expected: Vec<i32> = vec![];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![537, -604, 3472, -4742, 1996, 2668];
    let expected: Vec<i32> = vec![537, 3472, 1996, 2668];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![-1391, 3771, -4375];
    let expected: Vec<i32> = vec![3771];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![3766, 2853, -4602, -4828, -4074, 3171, -2434, 4461];
    let expected: Vec<i32> = vec![3766, 2853, 3171, 4461];
    assert_eq!(get_positive(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![1865];
    let expected: Vec<i32> = vec![1865];
    assert_eq!(get_positive(arg_0), expected);
}