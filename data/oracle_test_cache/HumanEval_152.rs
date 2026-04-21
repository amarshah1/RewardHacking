#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let arg_1: Vec<i32> = vec![-4174, -3132, 1491, 2508, -2143, 2001, -1380, 2044, -4551, -416, 1589, 435, -3881, -4900, 176, -4043, -1667, -1758, 2153];
    let expected: Vec<i64> = vec![5725, 3243, 771, 1367, 136, 3064, 1926, 1708, 296, 3360, 2443, 3283, 3258, 8852, 4723, 838, 2962, 2093, 5470];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![2605, 2376];
    let arg_1: Vec<i32> = vec![-199, 416];
    let expected: Vec<i64> = vec![2804, 1960];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-3445, -1724, 1626, 4803, 409, 2900, -3605];
    let arg_1: Vec<i32> = vec![-1675, 3413, -1795, -823, 3746, 4610, 120];
    let expected: Vec<i64> = vec![1770, 5137, 3421, 5626, 3337, 1710, 3725];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: Vec<i32> = vec![];
    let expected: Vec<i64> = vec![];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![3173, 4786, 3419, -3846, 317];
    let arg_1: Vec<i32> = vec![-569, -3150, 718, -2576, 4077];
    let expected: Vec<i64> = vec![3742, 7936, 2701, 1270, 3760];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427];
    let arg_1: Vec<i32> = vec![2905, 2016, 1858, -4006, 1725, -3544, -2420, 2379, -1034, 3566, -4836, -3754, -2105, -3758, -1499, -2448, -407, -1659, 2926, -3205, 2295, -1061, -3060, -1747];
    let expected: Vec<i64> = vec![6474, 1794, 2758, 190, 6248, 5, 4571, 346, 1117, 5856, 4472, 2203, 6299, 5323, 5874, 6079, 4468, 6097, 363, 6069, 1424, 108, 4913, 2680];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![617];
    let arg_1: Vec<i32> = vec![4778];
    let expected: Vec<i64> = vec![4161];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-4850, -4056, -3983, 2238];
    let arg_1: Vec<i32> = vec![1460, -108, -2123, 1758];
    let expected: Vec<i64> = vec![6310, 3948, 1860, 480];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![2668, -2392, 1393, -1292];
    let arg_1: Vec<i32> = vec![-1391, 3771, -4375, 1342];
    let expected: Vec<i64> = vec![4059, 6163, 5768, 2634];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![3766, 2853, -4602, -4828, -4074, 3171, -2434];
    let arg_1: Vec<i32> = vec![4461, -2467, -2057, 1865, -2077, 1230, 1316];
    let expected: Vec<i64> = vec![695, 5320, 2545, 6693, 1997, 1941, 3750];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-2801, 3491, 1672];
    let arg_1: Vec<i32> = vec![1352, -2108, 3962];
    let expected: Vec<i64> = vec![4153, 5599, 2290];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![723, -1542, 4504, 3589, -2737, -1794, 2428, -2637, 4095];
    let arg_1: Vec<i32> = vec![4861, 4819, -2856, 1024, 4963, -220, 4215, -4849, -4595];
    let expected: Vec<i64> = vec![4138, 6361, 7360, 2565, 7700, 1574, 1787, 2212, 8690];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![3716, -4886, 3873, -4779];
    let arg_1: Vec<i32> = vec![-4091, -2060, -2406, 2639];
    let expected: Vec<i64> = vec![7807, 2826, 6279, 7418];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![596, -2389, 2052, 1775];
    let arg_1: Vec<i32> = vec![-1343, -3251, -3243, -2552];
    let expected: Vec<i64> = vec![1939, 862, 5295, 4327];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![3280, 1888, 3641, -1669, -4342, 967, 971, 564, -714];
    let arg_1: Vec<i32> = vec![2457, -3137, 1004, -2618, -570, 4691, -4149, 4945, -724];
    let expected: Vec<i64> = vec![823, 5025, 2637, 949, 3772, 3724, 5120, 4381, 10];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![1113, -3776, 3546];
    let arg_1: Vec<i32> = vec![2136, 2350, -3195];
    let expected: Vec<i64> = vec![1023, 6126, 6741];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-4561];
    let arg_1: Vec<i32> = vec![590];
    let expected: Vec<i64> = vec![5151];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-4748, 4182, -137, -4029, 1981, -670];
    let arg_1: Vec<i32> = vec![999, 3989, -4674, -1385, -368, -3894];
    let expected: Vec<i64> = vec![5747, 193, 4537, 2644, 2349, 3224];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![2475, -4745];
    let arg_1: Vec<i32> = vec![-3571, 631];
    let expected: Vec<i64> = vec![6046, 5376];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![1042, -562, -375, 3382, 2751, 156, 1096, -2601, 1598, -2963, -3663, 4339];
    let arg_1: Vec<i32> = vec![-4613, -4053, 2673, -893, 3023, -3185, -1288, 1438, 200, 1000, -2233, -4740];
    let expected: Vec<i64> = vec![5655, 3491, 3048, 4275, 272, 3341, 2384, 4039, 1398, 3963, 1430, 9079];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![224, 1307, 3462, -2812, -4063, 2580, -2315, 3244, 3739, -1927, -3105, -2968, 242, 3016, 1796, 866, -1759, 230];
    let arg_1: Vec<i32> = vec![3442, 2838, 209, -4948, 1702, 2263, 4585, 2001, -3758, -2569, 945, -4632, 2990, 207, -2109, -2371, -285, 284];
    let expected: Vec<i64> = vec![3218, 1531, 3253, 2136, 5765, 317, 6900, 1243, 7497, 642, 4050, 1664, 2748, 2809, 3905, 3237, 1474, 54];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![-4569, 1671];
    let arg_1: Vec<i32> = vec![4057, 4901];
    let expected: Vec<i64> = vec![8626, 3230];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![-2849, 2640, 2943, 2580, -1254, 2982, 1152, 2217, -4028, 4820, -4812, 4420];
    let arg_1: Vec<i32> = vec![231, 2196, -2165, 4636, 2568, 480, 923, 709, 3319, 3250, -961, 1349];
    let expected: Vec<i64> = vec![3080, 444, 5108, 2056, 3822, 2502, 229, 1508, 7347, 1570, 3851, 3071];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![2620, 1498];
    let arg_1: Vec<i32> = vec![-1148, 1643];
    let expected: Vec<i64> = vec![3768, 145];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}