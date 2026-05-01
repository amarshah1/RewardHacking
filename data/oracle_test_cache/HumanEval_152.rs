#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![241i32, 1044i32, -2930i32, -1569i32, 2115i32, -685i32, 4573i32, -2585i32, -3863i32];
    let arg_1: Vec<i32> = vec![2275i32, -4293i32, 376i32, -1569i32, 1107i32, -685i32, 3661i32, -2978i32, 1683i32];
    let expected: Vec<i64> = vec![2034, 5337, 3306, 0, 1008, 0, 912, 393, 5546];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![-1086i32, -2835i32, 4902i32, 3093i32, -2293i32, -1958i32];
    let arg_1: Vec<i32> = vec![-4754i32, -2468i32, 2654i32, 3542i32, -2293i32, -1898i32];
    let expected: Vec<i64> = vec![3668, 367, 2248, 449, 0, 60];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-1370i32, 1565i32, -4821i32, 674i32, -4582i32, -4778i32, -170i32];
    let arg_1: Vec<i32> = vec![-3185i32, 1565i32, 4188i32, -2301i32, -4582i32, 3049i32, -794i32];
    let expected: Vec<i64> = vec![1815, 0, 9009, 2975, 0, 7827, 624];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-404i32, -4857i32, 1471i32, -4301i32, -3733i32, -3810i32, 3954i32];
    let arg_1: Vec<i32> = vec![-2123i32, -722i32, 1471i32, 1257i32, 4209i32, 1361i32, 4781i32];
    let expected: Vec<i64> = vec![1719, 4135, 0, 5558, 7942, 5171, 827];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![1804i32, 4047i32, -3050i32, -4059i32, -3954i32, -1977i32, -688i32, 1600i32, -3225i32, -1858i32, 3604i32, -1668i32, -2842i32, -1106i32, -1234i32, 3549i32, 1738i32, -3222i32, 4180i32, -1540i32, -383i32, -4624i32, 3478i32];
    let arg_1: Vec<i32> = vec![-3617i32, 1911i32, 1415i32, -4059i32, -3954i32, -1977i32, -688i32, 1600i32, -3225i32, 3764i32, -3845i32, -1668i32, -4818i32, 3242i32, -1234i32, 3908i32, 2014i32, -4899i32, 4180i32, 1881i32, -383i32, 4933i32, 3478i32];
    let expected: Vec<i64> = vec![5421, 2136, 4465, 0, 0, 0, 0, 0, 0, 5622, 7449, 0, 1976, 4348, 0, 359, 276, 1677, 0, 3421, 0, 9557, 0];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-796i32, -3290i32, -851i32, 103i32, 1737i32, 2260i32, -4593i32];
    let arg_1: Vec<i32> = vec![3661i32, -1989i32, -4125i32, 103i32, 3028i32, 3834i32, 830i32];
    let expected: Vec<i64> = vec![4457, 1301, 3274, 0, 1291, 1574, 5423];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-2399i32, -1337i32, 2799i32, 1473i32, 1702i32, -4286i32, -1303i32, 4219i32, 2292i32, 3935i32, -459i32, -3859i32, -2601i32];
    let arg_1: Vec<i32> = vec![1201i32, -1113i32, 4215i32, -1744i32, 1702i32, -4550i32, -2689i32, 4510i32, -3350i32, -889i32, 1170i32, 3576i32, -2674i32];
    let expected: Vec<i64> = vec![3600, 224, 1416, 3217, 0, 264, 1386, 291, 5642, 4824, 1629, 7435, 73];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-2863i32, -1635i32, -1775i32, -188i32, 2030i32, -154i32];
    let arg_1: Vec<i32> = vec![-3086i32, -2513i32, 2436i32, -3220i32, 2030i32, -1801i32];
    let expected: Vec<i64> = vec![223, 878, 4211, 3032, 0, 1647];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-3266i32, 2849i32, 868i32, -4749i32, 2691i32, -3846i32, 1740i32, 1089i32];
    let arg_1: Vec<i32> = vec![1514i32, 2849i32, 868i32, -1612i32, 1272i32, -1136i32, 1624i32, -2034i32];
    let expected: Vec<i64> = vec![4780, 0, 0, 3137, 1419, 2710, 116, 3123];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![1083i32, -4188i32, -2649i32, -2091i32, 1987i32, -1728i32, 1913i32, 3239i32, 4773i32, -1955i32, -4541i32, 2368i32, -1044i32, -2803i32, 3214i32, 4048i32, -4194i32, -2448i32];
    let arg_1: Vec<i32> = vec![1083i32, -4893i32, 2069i32, 4974i32, -2375i32, -3106i32, 1913i32, -2885i32, 4773i32, 2595i32, 3488i32, -991i32, -4466i32, 3391i32, 3214i32, 1051i32, -4194i32, -4390i32];
    let expected: Vec<i64> = vec![0, 705, 4718, 7065, 4362, 1378, 0, 6124, 0, 4550, 8029, 3359, 3422, 6194, 0, 2997, 0, 1942];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![4760i32, -4877i32, 2830i32, -689i32, -3746i32, 3815i32];
    let arg_1: Vec<i32> = vec![4035i32, 799i32, 2830i32, -2663i32, -815i32, 3985i32];
    let expected: Vec<i64> = vec![725, 5676, 0, 1974, 2931, 170];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-1329i32, 2820i32, -603i32, 838i32, -169i32, -2780i32, -1215i32, -2156i32, -2622i32];
    let arg_1: Vec<i32> = vec![-1329i32, -1349i32, 763i32, 838i32, -3246i32, -2780i32, 407i32, -3390i32, 1016i32];
    let expected: Vec<i64> = vec![0, 4169, 1366, 0, 3077, 0, 1622, 1234, 3638];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![-2864i32, -527i32, -1779i32, -552i32, 3141i32, -2529i32];
    let arg_1: Vec<i32> = vec![3344i32, 2253i32, 2426i32, -552i32, -665i32, -2529i32];
    let expected: Vec<i64> = vec![6208, 2780, 4205, 0, 3806, 0];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![395i32, -1840i32, -1046i32, -3346i32, -217i32, -4744i32];
    let arg_1: Vec<i32> = vec![395i32, -2605i32, -1046i32, 1703i32, 2709i32, 598i32];
    let expected: Vec<i64> = vec![0, 765, 0, 5049, 2926, 5342];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-1021i32, 695i32, -572i32, 404i32, -4309i32, -2112i32, 2767i32, -4412i32, 279i32, -794i32, 1686i32, -4403i32, -4976i32];
    let arg_1: Vec<i32> = vec![-1021i32, 695i32, 4950i32, 404i32, -4309i32, -1922i32, -4039i32, -4412i32, 4843i32, 931i32, -4095i32, -4403i32, -4479i32];
    let expected: Vec<i64> = vec![0, 0, 5522, 0, 0, 190, 6806, 0, 4564, 1725, 5781, 0, 497];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![629i32, -1836i32, 4718i32, 1073i32, 1720i32, 4580i32, 3709i32, 2651i32, -4720i32, -1935i32, 1746i32, -3325i32];
    let arg_1: Vec<i32> = vec![629i32, -799i32, -2140i32, -305i32, -281i32, 669i32, 1679i32, 2651i32, 1351i32, -312i32, 1502i32, -3980i32];
    let expected: Vec<i64> = vec![0, 1037, 6858, 1378, 2001, 3911, 2030, 0, 6071, 1623, 244, 655];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-4339i32, 2265i32, 458i32, 3310i32, 2551i32];
    let arg_1: Vec<i32> = vec![-1410i32, 2265i32, -1521i32, -2316i32, 2551i32];
    let expected: Vec<i64> = vec![2929, 0, 1979, 5626, 0];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-4272i32, 2822i32, -828i32, 1342i32, -2314i32, 4757i32, 2759i32, 292i32, 732i32];
    let arg_1: Vec<i32> = vec![-3354i32, 4394i32, -828i32, 1342i32, 2189i32, -558i32, 2759i32, -1734i32, -2507i32];
    let expected: Vec<i64> = vec![918, 1572, 0, 0, 4503, 5315, 0, 2026, 3239];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let arg_1: Vec<i32> = vec![-4174, -3132, 1491, 2508, -2143, 2001, -1380, 2044, -4551, -416, 1589, 435, -3881, -4900, 176, -4043, -1667, -1758, 2153];
    let expected: Vec<i64> = vec![5725, 3243, 771, 1367, 136, 3064, 1926, 1708, 296, 3360, 2443, 3283, 3258, 8852, 4723, 838, 2962, 2093, 5470];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![2605, 2376];
    let arg_1: Vec<i32> = vec![-199, 416];
    let expected: Vec<i64> = vec![2804, 1960];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![-3445, -1724, 1626, 4803, 409, 2900, -3605];
    let arg_1: Vec<i32> = vec![-1675, 3413, -1795, -823, 3746, 4610, 120];
    let expected: Vec<i64> = vec![1770, 5137, 3421, 5626, 3337, 1710, 3725];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: Vec<i32> = vec![];
    let expected: Vec<i64> = vec![];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![3173, 4786, 3419, -3846, 317];
    let arg_1: Vec<i32> = vec![-569, -3150, 718, -2576, 4077];
    let expected: Vec<i64> = vec![3742, 7936, 2701, 1270, 3760];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427];
    let arg_1: Vec<i32> = vec![2905, 2016, 1858, -4006, 1725, -3544, -2420, 2379, -1034, 3566, -4836, -3754, -2105, -3758, -1499, -2448, -407, -1659, 2926, -3205, 2295, -1061, -3060, -1747];
    let expected: Vec<i64> = vec![6474, 1794, 2758, 190, 6248, 5, 4571, 346, 1117, 5856, 4472, 2203, 6299, 5323, 5874, 6079, 4468, 6097, 363, 6069, 1424, 108, 4913, 2680];
    assert_eq!(compare(&arg_0, &arg_1), expected);
}