#[test]
fn oracle_case_0() {
    let arg_0: Vec<Vec<i32>> = vec![vec![111], vec![-2007, -1063, -3306], vec![-3851, -3317, -4174, -3132, 1491, 2508, -2143, 2001, -1380, 2044, -4551, -416, 1589, 435, -3881, -4900, 176, -4043, -1667], vec![2605, 2376, -199, 416, 2862, 296], vec![-3445, -1724, 1626], vec![-1675, 3413, -1795, -823, 3746, 4610, 120], vec![], vec![3173, 4786, 3419, -3846, 317], vec![-3569, 3810, -900, -3816, -4523, -3539, 2151, 2725, -2151, -2290, -364, -1551, 4194, 1565, 4375, 3631, -4875, 4438, 2563, 2864, 871, -1169, 1853, -4427, 2905, 2016, 1858, -4006, 1725, -3544, -2420, 2379, -1034, 3566], vec![4482, -3155], vec![], vec![-2201, -1934, -4477, 4725], vec![-3060, -1747], vec![617], vec![-4850, -4056, -3983, 2238, 1460, -108], vec![], vec![2668, -2392, 1393, -1292, -1391, 3771, -4375], vec![], vec![3766, 2853, -4602, -4828, -4074, 3171, -2434, 4461]];
    let arg_1: i32 = -2467;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-454, 274, -420, 1460]];
    let arg_1: i32 = 2717;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<Vec<i32>> = vec![vec![546], vec![723, -1542, 4504, 3589, -2737, -1794, 2428, -2637], vec![], vec![], vec![]];
    let arg_1: i32 = 564;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = -2856;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<Vec<i32>> = vec![vec![3716, -4886, 3873, -4779, -4091, -2060, -2406, 2639], vec![596, -2389, 2052, 1775], vec![3280, 1888, 3641, -1669, -4342, 967, 971, 564, -714, 2457, -3137, 1004, -2618, -570, 4691, -4149, 4945], vec![130, 695], vec![], vec![], vec![2350, -3195, 4718, 1798, -4819, -719]];
    let arg_1: i32 = 797;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![397], vec![1981, -670]];
    let arg_1: i32 = 999;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = -1522;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2475, -4745, -3571, 631, -929, 4297, -2689]];
    let arg_1: i32 = 1935;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-562], vec![2438, 4957, -179, -4180, 4908, 231, 4963, 4287, 3022, 3100, 1907, -3494, 2306, 2415, 1484, -3185, -893, 951], vec![224, 1307, 3462, -2812, -4063, 2580, -2315], vec![4864, -4146, -3853, -2118, 827, 491, 3154, 380], vec![]];
    let arg_1: i32 = 209;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![]];
    let arg_1: i32 = -4955;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-2638], vec![], vec![-2371, -285, 284, 4104, 1267], vec![], vec![4057]];
    let arg_1: i32 = 4901;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2943, 2580, -1254], vec![-2165, 4636, 2568, 480, 923, 709, 3319, 3250, -961, 1349, -2862, 4811, -1584, 686, -631, 781, 1041], vec![], vec![-1326, 3616, -4872, 1485], vec![], vec![-2219], vec![-3188], vec![3553, -1129], vec![3980, 3896, -758, 1976], vec![3142, 1634, -231], vec![], vec![]];
    let arg_1: i32 = 173;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![1262, 4935], vec![4509, -3744, -4751, 3945, 4124, -3624]];
    let arg_1: i32 = -1960;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = -1167;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-4061, -483], vec![-2383], vec![3311], vec![4695, 1981, 1778, 2370, 665, 1840], vec![4166, -4910, 418, -2173], vec![1312, 3962, -1091, -3973, 1702, -1280, -1186, 4687, -1657, 2295]];
    let arg_1: i32 = -1303;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2229, -4477, 2298, -1589, -2231, -2670, 2707, -210, 1591, 3882], vec![-2666, 3085], vec![2918, 3426, -4538, -1395, -4633, 1981, -1448, -4160, -1920]];
    let arg_1: i32 = -964;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = -1410;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<Vec<i32>> = vec![vec![1356, -4549, 3254], vec![-3013, -4542, 3017, -2631], vec![], vec![], vec![1000, 2379, -1937, 4112, -3993], vec![-4178], vec![3587], vec![], vec![]];
    let arg_1: i32 = 1491;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2121]];
    let arg_1: i32 = -1275;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2308, -3484, -2511, 203, 4896, 2422, -424, 2792, -4799, -1269, 501, -300], vec![943, 4521, -2392, 1040, 1784, 3176, 4992, -4287, 847, 2765]];
    let arg_1: i32 = 764;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-2523, 3953]];
    let arg_1: i32 = -2189;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<Vec<i32>> = vec![vec![383], vec![3094, 2282, -768, 4193]];
    let arg_1: i32 = 4546;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = 4001;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = 2048;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}