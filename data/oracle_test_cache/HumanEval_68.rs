#[test]
fn oracle_case_0() {
    let arg_0: Vec<u32> = vec![];
    let expected: Vec<u32> = vec![];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u32> = vec![93u32, 8u32, 8u32];
    let expected: Vec<u32> = vec![8, 1];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u32> = vec![0u32, 78u32, 0u32, 26u32, 2u32, 0u32];
    let expected: Vec<u32> = vec![0, 0];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u32> = vec![19u32, 10u32, 49u32, 11u32, 8u32, 8u32];
    let expected: Vec<u32> = vec![8, 4];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u32> = vec![4u32, 4u32, 58u32, 21u32, 16u32, 4u32, 91u32];
    let expected: Vec<u32> = vec![4, 0];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u32> = vec![2129u32, 4837u32, 4687u32, 1985u32, 3955u32, 3075u32, 3827u32, 3753u32, 4557u32, 4121u32, 3891u32, 3115u32, 2441u32, 33u32, 225u32, 4297u32, 821u32, 3645u32, 1775u32, 4691u32, 2051u32];
    let expected: Vec<u32> = vec![];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u32> = vec![1727u32, 2863u32, 3863u32, 2461u32, 917u32, 1005u32, 571u32, 4713u32, 2283u32, 4763u32, 1493u32, 2847u32, 219u32, 2377u32, 1073u32, 4797u32, 3791u32, 3499u32, 4405u32, 2697u32, 2877u32, 861u32, 2069u32, 3451u32, 2637u32];
    let expected: Vec<u32> = vec![];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u32> = vec![1361u32, 1683u32, 3339u32, 2721u32, 1921u32, 3429u32, 4011u32, 4631u32, 3003u32, 3507u32, 977u32, 3667u32, 2113u32, 3849u32, 129u32, 2037u32, 969u32];
    let expected: Vec<u32> = vec![];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u32> = vec![35u32, 38u32, 82u32, 99u32, 38u32, 95u32, 79u32, 11u32, 18u32, 63u32, 54u32, 93u32, 32u32, 48u32, 13u32, 8u32, 5u32, 8u32, 8u32, 19u32];
    let expected: Vec<u32> = vec![8, 15];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u32> = vec![93u32, 61u32, 21u32, 81u32, 99u32, 9u32, 100u32, 21u32, 70u32, 36u32, 68u32, 80u32, 2u32, 2u32, 26u32, 70u32, 91u32];
    let expected: Vec<u32> = vec![2, 12];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u32> = vec![91u32, 52u32, 68u32, 57u32, 64u32, 62u32, 74u32, 41u32, 12u32, 11u32, 77u32, 46u32, 82u32, 19u32, 101u32, 6u32, 0u32, 58u32, 20u32, 33u32];
    let expected: Vec<u32> = vec![0, 16];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u32> = vec![1551, 504, 111, 3908, 2262, 2425, 3875, 3784, 2007, 4526, 1063, 3986, 3306, 4508, 3752, 2186, 4255, 1263, 3776];
    let expected: Vec<u32> = vec![504, 1];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u32> = vec![3851, 1339, 3317, 149, 4174, 3658, 3132, 4625, 1491, 763, 2508, 3063, 2143, 2713];
    let expected: Vec<u32> = vec![2508, 10];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u32> = vec![4432, 416, 781, 1589, 2081, 435];
    let expected: Vec<u32> = vec![416, 1];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u32> = vec![];
    let expected: Vec<u32> = vec![];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u32> = vec![1667, 1735, 1758, 1495, 2153, 2911, 2877];
    let expected: Vec<u32> = vec![1758, 2];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u32> = vec![2605];
    let expected: Vec<u32> = vec![];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u32> = vec![1176];
    let expected: Vec<u32> = vec![1176, 0];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u32> = vec![416];
    let expected: Vec<u32> = vec![416, 0];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u32> = vec![3445, 3837, 1724, 3742, 1626, 3593, 4803, 1007];
    let expected: Vec<u32> = vec![1626, 4];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u32> = vec![1675, 908, 3413, 2973, 1795];
    let expected: Vec<u32> = vec![908, 1];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u32> = vec![3746, 1698];
    let expected: Vec<u32> = vec![1698, 1];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u32> = vec![4671, 4864, 4417, 3279];
    let expected: Vec<u32> = vec![4864, 1];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u32> = vec![3173];
    let expected: Vec<u32> = vec![];
    assert_eq!(pluck_smallest_even(&arg_0), expected);
}