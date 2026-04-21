#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317, -4174, -3132];
    let expected: (i32, i32) = (-3317, -3306);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![-3063, -2713, 3574, 3640];
    let expected: (i32, i32) = (3574, 3640);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![4432, -781, -2081];
    let expected: (i32, i32) = (-2081, -781);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-3881, -4900];
    let expected: (i32, i32) = (-4900, -3881);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![-1667, -1758, 2153, 2877, 3543];
    let expected: (i32, i32) = (-1758, -1667);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![1176, 4382, 2793];
    let expected: (i32, i32) = (2793, 4382);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-3445, -1724, 1626, 4803, 409, 2900, -3605, -1675];
    let expected: (i32, i32) = (-1724, -1675);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![3746, 4610, 120, -2035, -4864, -3279, -2865];
    let expected: (i32, i32) = (-3279, -2865);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![650, -2147, 242];
    let expected: (i32, i32) = (242, 650);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-569, -3150];
    let expected: (i32, i32) = (-3150, -569);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427, 2905, 2016, 1858, -4006, 1725, -3544, -2420, 2379];
    let expected: (i32, i32) = (1853, 1858);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![3566, -4836, -3754];
    let expected: (i32, i32) = (-4836, -3754);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![-1499, -2448, -407, -1659, 2926];
    let expected: (i32, i32) = (-1659, -1499);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-3060, -1747, 1966, 617, 4778, -3432, -616];
    let expected: (i32, i32) = (-3432, -3060);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-4850, -4056];
    let expected: (i32, i32) = (-4850, -4056);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![537, -604, 3472, -4742, 1996, 2668, -2392, 1393];
    let expected: (i32, i32) = (1393, 1996);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-1391, 3771, -4375];
    let expected: (i32, i32) = (-4375, -1391);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![3269, 1932];
    let expected: (i32, i32) = (1932, 3269);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![3766, 2853, -4602, -4828, -4074, 3171];
    let expected: (i32, i32) = (-4828, -4602);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![2370, -3042, 4364, 3942];
    let expected: (i32, i32) = (3942, 4364);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![-454, 274, -420];
    let expected: (i32, i32) = (-454, -420);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![-2801, 3491];
    let expected: (i32, i32) = (-2801, 3491);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![1072, 546, 918, -1403, 4363, -1533];
    let expected: (i32, i32) = (-1533, -1403);
    assert_eq!(closest_pair(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![723, -1542];
    let expected: (i32, i32) = (-1542, 723);
    assert_eq!(closest_pair(arg_0), expected);
}