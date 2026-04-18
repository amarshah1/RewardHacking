#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let expected: (Option<i32>, Option<i32>) = (Some(-623), Some(111));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![-3063, -2713, 3574, 3640, 1331, 4432];
    let expected: (Option<i32>, Option<i32>) = (Some(-2713), Some(1331));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-3881, -4900, 176, -4043];
    let expected: (Option<i32>, Option<i32>) = (Some(-3881), Some(176));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![];
    let expected: (Option<i32>, Option<i32>) = (None, None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![2605, 2376, -199, 416, 2862, 296, -4084];
    let expected: (Option<i32>, Option<i32>) = (Some(-199), Some(296));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-3445];
    let expected: (Option<i32>, Option<i32>) = (Some(-3445), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-3593, -1007];
    let expected: (Option<i32>, Option<i32>) = (Some(-1007), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-1675, 3413, -1795, -823];
    let expected: (Option<i32>, Option<i32>) = (Some(-823), Some(3413));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-4671, -4417, -1662, 3173, 4786];
    let expected: (Option<i32>, Option<i32>) = (Some(-1662), Some(3173));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-569, -3150, 718, -2576, 4077];
    let expected: (Option<i32>, Option<i32>) = (Some(-569), Some(718));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427];
    let expected: (Option<i32>, Option<i32>) = (Some(-364), Some(871));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![1858, -4006];
    let expected: (Option<i32>, Option<i32>) = (Some(-4006), Some(1858));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![611];
    let expected: (Option<i32>, Option<i32>) = (None, Some(611));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![3566, -4836, -3754, -2105];
    let expected: (Option<i32>, Option<i32>) = (Some(-2105), Some(3566));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-1499];
    let expected: (Option<i32>, Option<i32>) = (Some(-1499), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-2201, -1934];
    let expected: (Option<i32>, Option<i32>) = (Some(-1934), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-3060, -1747, 1966, 617, 4778, -3432];
    let expected: (Option<i32>, Option<i32>) = (Some(-1747), Some(617));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-4850, -4056];
    let expected: (Option<i32>, Option<i32>) = (Some(-4056), None);
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![537, -604, 3472, -4742, 1996, 2668];
    let expected: (Option<i32>, Option<i32>) = (Some(-604), Some(537));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![-1391, 3771, -4375];
    let expected: (Option<i32>, Option<i32>) = (Some(-1391), Some(3771));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![3766, 2853, -4602, -4828, -4074, 3171, -2434, 4461];
    let expected: (Option<i32>, Option<i32>) = (Some(-2434), Some(2853));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![1865];
    let expected: (Option<i32>, Option<i32>) = (None, Some(1865));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![-454, 274];
    let expected: (Option<i32>, Option<i32>) = (Some(-454), Some(274));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![-2801, 3491];
    let expected: (Option<i32>, Option<i32>) = (Some(-2801), Some(3491));
    assert_eq!(largest_smallest_integers(&arg_0), expected);
}