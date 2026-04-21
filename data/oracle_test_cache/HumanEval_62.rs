#[test]
fn oracle_case_0() {
    let arg_0: Vec<u32> = vec![1551, 504, 111, 3908, 2262, 2425, 3875, 3784, 2007, 4526, 1063, 3986, 3306, 4508, 3752, 2186, 4255, 1263, 3776];
    let expected: Vec<u64> = vec![504, 222, 11724, 9048, 12125, 23250, 26488, 16056, 40734, 10630, 43846, 39672, 58604, 52528, 32790, 68080, 21471, 67968];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u32> = vec![3851, 1339, 3317, 149, 4174, 3658, 3132, 4625, 1491, 763, 2508, 3063, 2143, 2713];
    let expected: Vec<u64> = vec![1339, 6634, 447, 16696, 18290, 18792, 32375, 11928, 6867, 25080, 33693, 25716, 35269];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u32> = vec![4432, 416, 781, 1589, 2081, 435];
    let expected: Vec<u64> = vec![416, 1562, 4767, 8324, 2175];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u32> = vec![];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u32> = vec![1667, 1735, 1758, 1495, 2153, 2911, 2877];
    let expected: Vec<u64> = vec![1735, 3516, 4485, 8612, 14555, 17262];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u32> = vec![2605];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u32> = vec![1176];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u32> = vec![416];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u32> = vec![3445, 3837, 1724, 3742, 1626, 3593, 4803, 1007];
    let expected: Vec<u64> = vec![3837, 3448, 11226, 6504, 17965, 28818, 7049];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u32> = vec![1675, 908, 3413, 2973, 1795];
    let expected: Vec<u64> = vec![908, 6826, 8919, 7180];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u32> = vec![3746, 1698];
    let expected: Vec<u64> = vec![1698];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u32> = vec![4671, 4864, 4417, 3279];
    let expected: Vec<u64> = vec![4864, 8834, 9837];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u32> = vec![3173];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u32> = vec![650];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u32> = vec![569, 2155, 3150, 1147, 718];
    let expected: Vec<u64> = vec![2155, 6300, 3441, 2872];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u32> = vec![3569, 3708, 3810, 3773, 900, 584, 3816, 723, 4523, 3077, 3539, 3479, 2151, 2905, 2725, 377, 2151, 4275, 2290, 167, 364, 1007, 1551, 1291, 4194, 2430, 1565, 1225, 4375];
    let expected: Vec<u64> = vec![3708, 7620, 11319, 3600, 2920, 22896, 5061, 36184, 27693, 35390, 38269, 25812, 37765, 38150, 5655, 34416, 72675, 41220, 3173, 7280, 21147, 34122, 29693, 100656, 60750, 40690, 33075, 122500];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u32> = vec![2234];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u32> = vec![2563, 2658, 2864];
    let expected: Vec<u64> = vec![2658, 5728];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u32> = vec![3705];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u32> = vec![4427, 516, 2905];
    let expected: Vec<u64> = vec![516, 5810];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u32> = vec![1858, 3193];
    let expected: Vec<u64> = vec![3193];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u32> = vec![611];
    let expected: Vec<u64> = vec![];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u32> = vec![3566, 2830, 4836, 3008, 3754];
    let expected: Vec<u64> = vec![2830, 9672, 9024, 15016];
    assert_eq!(derivative(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u32> = vec![1499, 4235, 2448, 3376];
    let expected: Vec<u64> = vec![4235, 4896, 10128];
    assert_eq!(derivative(&arg_0), expected);
}