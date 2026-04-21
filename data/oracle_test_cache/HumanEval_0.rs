#[test]
fn oracle_case_0() {
    let arg_0: Vec<i64> = vec![-3063, -2713, 3574, 3640];
    let arg_1: i64 = 1331;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i64> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let arg_1: i64 = -4174;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i64> = vec![-3881, -4900, 176, -4043, -1667, -1758];
    let arg_1: i64 = 2153;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i64> = vec![2605, 2376];
    let arg_1: i64 = -199;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i64> = vec![4482, -3155, 4700, -4235];
    let arg_1: i64 = 3376;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i64> = vec![-3445, -1724, 1626, 4803, 409, 2900, -3605, -1675, 3413];
    let arg_1: i64 = -1795;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i64> = vec![3766, 2853, -4602, -4828, -4074, 3171, -2434];
    let arg_1: i64 = 4461;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i64> = vec![3746];
    let arg_1: i64 = 4610;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i64> = vec![-4849, -4595, 2763, 4210, -4555, 2922];
    let arg_1: i64 = 4682;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i64> = vec![-4671, -4417];
    let arg_1: i64 = -1662;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i64> = vec![3280, 1888, 3641, -1669, -4342, 967, 971, 564, -714, 2457, -3137];
    let arg_1: i64 = 1004;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i64> = vec![650, -2147];
    let arg_1: i64 = 242;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i64> = vec![-4748, 4182, -137, -4029, 1981, -670, 999];
    let arg_1: i64 = 3989;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i64> = vec![];
    let arg_1: i64 = -569;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i64> = vec![2475, -4745, -3571, 631, -929, 4297, -2689];
    let arg_1: i64 = 1935;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i64> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427, 2905, 2016, 1858, -4006, 1725, -3544, -2420, 2379];
    let arg_1: i64 = -1034;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i64> = vec![2438, 4957, -179, -4180, 4908, 231, 4963, 4287, 3022, 3100];
    let arg_1: i64 = 1907;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i64> = vec![-3060, -1747, 1966, 617, 4778, -3432, -616, -3916, 3122, -469];
    let arg_1: i64 = -2817;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i64> = vec![-2979, -2638, 4809, -4138, -1722, -1099, -4383];
    let arg_1: i64 = 2729;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i64> = vec![537, -604, 3472];
    let arg_1: i64 = -4742;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i64> = vec![-4569, 1671, 4057];
    let arg_1: i64 = 4901;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i64> = vec![2668];
    let arg_1: i64 = -2392;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i64> = vec![-2849, 2640, 2943, 2580, -1254, 2982, 1152, 2217, -4028, 4820, -4812, 4420];
    let arg_1: i64 = 231;
    let expected: bool = true;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i64> = vec![-1391, 3771, -4375];
    let arg_1: i64 = 1342;
    let expected: bool = false;
    assert_eq!(has_close_elements(&arg_0, arg_1), expected);
}