#[test]
fn oracle_case_0() {
    let arg_0: Vec<u32> = vec![999983u32, 999982u32, 999981u32, 499991u32, 0u32, 1u32, 8u32, 1000u32, 999973u32, 999883u32];
    let expected: u32 = 47;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u32> = vec![999979u32, 999978u32, 999976u32, 499989u32, 0u32, 1u32, 8u32, 1001u32, 998979u32, 989979u32];
    let expected: u32 = 52;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u32> = vec![999961u32, 999960u32, 999957u32, 499980u32, 0u32, 1u32, 8u32, 1002u32, 999951u32, 999861u32];
    let expected: u32 = 43;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u32> = vec![157609u32, 157556u32, 157550u32, 2973u32, 0u32, 1u32, 8u32, 2000u32, 157503u32, 157491u32];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u32> = vec![154433u32, 154380u32, 154374u32, 2913u32, 0u32, 1u32, 8u32, 2001u32, 2617u32, 2531u32];
    let expected: u32 = 16;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u32> = vec![999959u32, 999958u32, 999954u32, 499979u32, 0u32, 1u32, 8u32, 1003u32, 998959u32, 989959u32];
    let expected: u32 = 50;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u32> = vec![152051u32, 151998u32, 151992u32, 2868u32, 0u32, 1u32, 8u32, 2002u32, 151945u32, 151933u32];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u32> = vec![151321u32, 151268u32, 151262u32, 2855u32, 0u32, 1u32, 8u32, 2003u32, 2564u32, 2480u32];
    let expected: u32 = 5;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u32> = vec![1551, 504, 111, 3908, 2262, 2425, 3875, 3784, 2007, 4526, 1063, 3986, 3306, 4508, 3752, 2186, 4255, 1263, 3776];
    let expected: u32 = 10;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u32> = vec![3851, 1339, 3317, 149, 4174, 3658, 3132, 4625, 1491, 763, 2508, 3063, 2143, 2713];
    let expected: u32 = 17;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u32> = vec![4432, 416, 781, 1589, 2081, 435];
    let expected: u32 = 11;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u32> = vec![];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u32> = vec![1667, 1735, 1758, 1495, 2153, 2911, 2877];
    let expected: u32 = 11;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u32> = vec![2605];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u32> = vec![1176];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u32> = vec![416];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u32> = vec![3445, 3837, 1724, 3742, 1626, 3593, 4803, 1007];
    let expected: u32 = 20;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u32> = vec![1675, 908, 3413, 2973, 1795];
    let expected: u32 = 11;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u32> = vec![3746, 1698];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u32> = vec![4671, 4864, 4417, 3279];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u32> = vec![3173];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u32> = vec![650];
    let expected: u32 = 0;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u32> = vec![569, 2155, 3150, 1147, 718];
    let expected: u32 = 20;
    assert_eq!(skjkasdkd(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u32> = vec![3569, 3708, 3810, 3773, 900, 584, 3816, 723, 4523, 3077, 3539, 3479, 2151, 2905, 2725, 377, 2151, 4275, 2290, 167, 364, 1007, 1551, 1291, 4194, 2430, 1565, 1225, 4375];
    let expected: u32 = 14;
    assert_eq!(skjkasdkd(arg_0), expected);
}