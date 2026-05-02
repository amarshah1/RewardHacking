#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![2u8, 1u8, 90u8, 101u8, 114u8];
    let expected: Vec<u8> = vec![2, 1, 122, 69, 82];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![89u8, 112u8, 79u8, 101u8, 71u8, 78u8, 73u8, 65u8];
    let expected: Vec<u8> = vec![121, 80, 111, 69, 103, 110, 105, 97];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![6u8, 106u8, 79u8, 113u8, 90u8];
    let expected: Vec<u8> = vec![6, 74, 111, 81, 122];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![75u8, 65u8];
    let expected: Vec<u8> = vec![107, 97];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![106u8, 0u8, 1u8, 5u8, 76u8, 71u8];
    let expected: Vec<u8> = vec![74, 0, 1, 5, 108, 103];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![85u8, 104u8, 73u8, 66u8, 111u8];
    let expected: Vec<u8> = vec![117, 72, 105, 98, 79];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![75u8, 65u8, 1u8, 106u8, 80u8, 2u8, 2u8];
    let expected: Vec<u8> = vec![107, 97, 1, 74, 112, 2, 2];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![105u8, 71u8, 107u8, 119u8];
    let expected: Vec<u8> = vec![73, 103, 75, 87];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![214u8, 155u8, 32u8, 136u8, 189u8, 216u8, 249u8, 14u8, 169u8, 4u8, 45u8, 133u8, 198u8, 196u8, 96u8, 173u8, 127u8, 127u8, 174u8, 58u8, 15u8, 227u8, 146u8, 228u8, 156u8];
    let expected: Vec<u8> = vec![156, 228, 146, 227, 15, 58, 174, 127, 127, 173, 96, 196, 198, 133, 45, 4, 169, 14, 249, 216, 189, 136, 32, 155, 214];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41, 96, 71, 243];
    let expected: Vec<u8> = vec![248, 225, 236, 42, 82, 209, 157, 68, 155, 222, 13, 49, 98, 167, 76, 41, 96, 103, 243];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39, 143, 201];
    let expected: Vec<u8> = vec![241, 75, 109, 118, 237, 120, 106, 31, 6, 228, 255, 39, 143, 201];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![91, 29, 61, 249, 229, 11];
    let expected: Vec<u8> = vec![11, 229, 249, 61, 29, 91];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![0, 68, 122, 81, 42, 72, 254];
    let expected: Vec<u8> = vec![0, 100, 90, 113, 42, 104, 254];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![130];
    let expected: Vec<u8> = vec![130];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![203];
    let expected: Vec<u8> = vec![203];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![208];
    let expected: Vec<u8> = vec![208];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![20, 119, 199, 61, 215, 23, 61, 108];
    let expected: Vec<u8> = vec![20, 87, 199, 61, 215, 23, 61, 76];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![76, 225, 99, 208, 128];
    let expected: Vec<u8> = vec![108, 225, 67, 208, 128];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![250, 99];
    let expected: Vec<u8> = vec![250, 67];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![151, 122, 113, 144];
    let expected: Vec<u8> = vec![151, 90, 81, 144];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![152];
    let expected: Vec<u8> = vec![152];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![41];
    let expected: Vec<u8> = vec![41];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![71, 7, 18, 63, 217];
    let expected: Vec<u8> = vec![103, 7, 18, 63, 217];
    assert_eq!(solve(&arg_0), expected);
}