#[test]
fn oracle_case_0() {
    let arg_0: Vec<u64> = vec![1551, 504, 111, 3908, 2262, 2425, 3875, 3784, 2007, 4526, 1063, 3986, 3306, 4508, 3752, 2186, 4255, 1263, 3776];
    let arg_1: u64 = 3668;
    let expected: Vec<u64> = vec![1551, 3668, 504, 3668, 111, 3668, 3908, 3668, 2262, 3668, 2425, 3668, 3875, 3668, 3784, 3668, 2007, 3668, 4526, 3668, 1063, 3668, 3986, 3668, 3306, 3668, 4508, 3668, 3752, 3668, 2186, 3668, 4255, 3668, 1263, 3668, 3776];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u64> = vec![3851, 1339, 3317, 149, 4174, 3658, 3132, 4625, 1491, 763, 2508, 3063, 2143];
    let arg_1: u64 = 2713;
    let expected: Vec<u64> = vec![3851, 2713, 1339, 2713, 3317, 2713, 149, 2713, 4174, 2713, 3658, 2713, 3132, 2713, 4625, 2713, 1491, 2713, 763, 2713, 2508, 2713, 3063, 2713, 2143];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u64> = vec![4432, 416, 781, 1589, 2081, 435];
    let arg_1: u64 = 2907;
    let expected: Vec<u64> = vec![4432, 2907, 416, 2907, 781, 2907, 1589, 2907, 2081, 2907, 435];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u64> = vec![1667, 1735, 1758, 1495, 2153, 2911, 2877];
    let arg_1: u64 = 2163;
    let expected: Vec<u64> = vec![1667, 2163, 1735, 2163, 1758, 2163, 1495, 2163, 2153, 2163, 2911, 2163, 2877];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 2605;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u64> = vec![1176];
    let arg_1: u64 = 199;
    let expected: Vec<u64> = vec![1176];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 416;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u64> = vec![3445, 3837, 1724, 3742, 1626, 3593, 4803, 1007];
    let arg_1: u64 = 409;
    let expected: Vec<u64> = vec![3445, 409, 3837, 409, 1724, 409, 3742, 409, 1626, 409, 3593, 409, 4803, 409, 1007];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u64> = vec![1675, 908, 3413, 2973];
    let arg_1: u64 = 1795;
    let expected: Vec<u64> = vec![1675, 1795, 908, 1795, 3413, 1795, 2973];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u64> = vec![3746, 1698];
    let arg_1: u64 = 4610;
    let expected: Vec<u64> = vec![3746, 4610, 1698];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u64> = vec![4671, 4864, 4417];
    let arg_1: u64 = 3279;
    let expected: Vec<u64> = vec![4671, 3279, 4864, 3279, 4417];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u64> = vec![3173];
    let arg_1: u64 = 1116;
    let expected: Vec<u64> = vec![3173];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 650;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u64> = vec![569, 2155, 3150, 1147, 718];
    let arg_1: u64 = 771;
    let expected: Vec<u64> = vec![569, 771, 2155, 771, 3150, 771, 1147, 771, 718];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u64> = vec![3569, 3708, 3810, 3773, 900, 584, 3816, 723, 4523, 3077, 3539, 3479, 2151, 2905, 2725, 377, 2151, 4275, 2290, 167, 364, 1007, 1551, 1291, 4194, 2430, 1565, 1225];
    let arg_1: u64 = 4375;
    let expected: Vec<u64> = vec![3569, 4375, 3708, 4375, 3810, 4375, 3773, 4375, 900, 4375, 584, 4375, 3816, 4375, 723, 4375, 4523, 4375, 3077, 4375, 3539, 4375, 3479, 4375, 2151, 4375, 2905, 4375, 2725, 4375, 377, 4375, 2151, 4375, 4275, 4375, 2290, 4375, 167, 4375, 364, 4375, 1007, 4375, 1551, 4375, 1291, 4375, 4194, 4375, 2430, 4375, 1565, 4375, 1225];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u64> = vec![2234];
    let arg_1: u64 = 4875;
    let expected: Vec<u64> = vec![2234];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u64> = vec![2563, 2658];
    let arg_1: u64 = 2864;
    let expected: Vec<u64> = vec![2563, 2864, 2658];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u64> = vec![3705];
    let arg_1: u64 = 1169;
    let expected: Vec<u64> = vec![3705];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u64> = vec![4427, 516];
    let arg_1: u64 = 2905;
    let expected: Vec<u64> = vec![4427, 2905, 516];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u64> = vec![1858, 3193];
    let arg_1: u64 = 4006;
    let expected: Vec<u64> = vec![1858, 4006, 3193];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 1725;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u64> = vec![611];
    let arg_1: u64 = 2420;
    let expected: Vec<u64> = vec![611];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u64> = vec![3566, 2830, 4836, 3008];
    let arg_1: u64 = 3754;
    let expected: Vec<u64> = vec![3566, 3754, 2830, 3754, 4836, 3754, 3008];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u64> = vec![1499, 4235, 2448, 3376];
    let arg_1: u64 = 407;
    let expected: Vec<u64> = vec![1499, 407, 4235, 407, 2448, 407, 3376];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}