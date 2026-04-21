#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41, 96, 71, 243];
    let expected: Vec<u8> = vec![248, 225, 236, 42, 82, 209, 157, 68, 155, 222, 13, 49, 98, 167, 76, 41, 96, 103, 243];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39, 143, 201];
    let expected: Vec<u8> = vec![241, 75, 109, 118, 237, 120, 106, 31, 6, 228, 255, 39, 143, 201];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![91, 29, 61, 249, 229, 11];
    let expected: Vec<u8> = vec![11, 229, 249, 61, 29, 91];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![0, 68, 122, 81, 42, 72, 254];
    let expected: Vec<u8> = vec![0, 100, 90, 113, 42, 104, 254];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![130];
    let expected: Vec<u8> = vec![130];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![203];
    let expected: Vec<u8> = vec![203];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![208];
    let expected: Vec<u8> = vec![208];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![20, 119, 199, 61, 215, 23, 61, 108];
    let expected: Vec<u8> = vec![20, 87, 199, 61, 215, 23, 61, 76];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![76, 225, 99, 208, 128];
    let expected: Vec<u8> = vec![108, 225, 67, 208, 128];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![250, 99];
    let expected: Vec<u8> = vec![250, 67];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![151, 122, 113, 144];
    let expected: Vec<u8> = vec![151, 90, 81, 144];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![152];
    let expected: Vec<u8> = vec![152];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![41];
    let expected: Vec<u8> = vec![41];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![71, 7, 18, 63, 217];
    let expected: Vec<u8> = vec![103, 7, 18, 63, 217];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151, 252, 182];
    let expected: Vec<u8> = vec![105, 61, 126, 240, 74, 231, 209, 6, 103, 53, 3, 165, 151, 177, 142, 95, 191, 114, 179, 36, 233, 103, 174, 22, 35, 68, 151, 252, 182];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![123];
    let expected: Vec<u8> = vec![123];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![14, 220, 204];
    let expected: Vec<u8> = vec![204, 220, 14];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![243];
    let expected: Vec<u8> = vec![243];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![234, 200, 140];
    let expected: Vec<u8> = vec![140, 200, 234];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![117, 172];
    let expected: Vec<u8> = vec![85, 172];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![39];
    let expected: Vec<u8> = vec![39];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![138, 62, 131, 206, 255];
    let expected: Vec<u8> = vec![255, 206, 131, 62, 138];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![11, 79, 192, 96];
    let expected: Vec<u8> = vec![11, 111, 192, 96];
    assert_eq!(solve(&arg_0), expected);
}