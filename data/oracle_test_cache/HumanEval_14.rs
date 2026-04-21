#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41, 96, 71, 243];
    let expected: Vec<Vec<u8>> = vec![vec![248], vec![248, 225], vec![248, 225, 236], vec![248, 225, 236, 42], vec![248, 225, 236, 42, 114], vec![248, 225, 236, 42, 114, 209], vec![248, 225, 236, 42, 114, 209, 157], vec![248, 225, 236, 42, 114, 209, 157, 100], vec![248, 225, 236, 42, 114, 209, 157, 100, 155], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41, 96], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41, 96, 71], vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41, 96, 71, 243]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39, 143, 201];
    let expected: Vec<Vec<u8>> = vec![vec![241], vec![241, 107], vec![241, 107, 77], vec![241, 107, 77, 86], vec![241, 107, 77, 86, 237], vec![241, 107, 77, 86, 237, 88], vec![241, 107, 77, 86, 237, 88, 74], vec![241, 107, 77, 86, 237, 88, 74, 31], vec![241, 107, 77, 86, 237, 88, 74, 31, 6], vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228], vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255], vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39], vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39, 143], vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39, 143, 201]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![91, 29, 61, 249, 229, 11];
    let expected: Vec<Vec<u8>> = vec![vec![91], vec![91, 29], vec![91, 29, 61], vec![91, 29, 61, 249], vec![91, 29, 61, 249, 229], vec![91, 29, 61, 249, 229, 11]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![];
    let expected: Vec<Vec<u8>> = vec![];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![0, 68, 122, 81, 42, 72, 254];
    let expected: Vec<Vec<u8>> = vec![vec![0], vec![0, 68], vec![0, 68, 122], vec![0, 68, 122, 81], vec![0, 68, 122, 81, 42], vec![0, 68, 122, 81, 42, 72], vec![0, 68, 122, 81, 42, 72, 254]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![130];
    let expected: Vec<Vec<u8>> = vec![vec![130]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![203];
    let expected: Vec<Vec<u8>> = vec![vec![203]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![208];
    let expected: Vec<Vec<u8>> = vec![vec![208]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![20, 119, 199, 61, 215, 23, 61, 108];
    let expected: Vec<Vec<u8>> = vec![vec![20], vec![20, 119], vec![20, 119, 199], vec![20, 119, 199, 61], vec![20, 119, 199, 61, 215], vec![20, 119, 199, 61, 215, 23], vec![20, 119, 199, 61, 215, 23, 61], vec![20, 119, 199, 61, 215, 23, 61, 108]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![76, 225, 99, 208, 128];
    let expected: Vec<Vec<u8>> = vec![vec![76], vec![76, 225], vec![76, 225, 99], vec![76, 225, 99, 208], vec![76, 225, 99, 208, 128]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![250, 99];
    let expected: Vec<Vec<u8>> = vec![vec![250], vec![250, 99]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![151, 122, 113, 144];
    let expected: Vec<Vec<u8>> = vec![vec![151], vec![151, 122], vec![151, 122, 113], vec![151, 122, 113, 144]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![152];
    let expected: Vec<Vec<u8>> = vec![vec![152]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![41];
    let expected: Vec<Vec<u8>> = vec![vec![41]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![71, 7, 18, 63, 217];
    let expected: Vec<Vec<u8>> = vec![vec![71], vec![71, 7], vec![71, 7, 18], vec![71, 7, 18, 63], vec![71, 7, 18, 63, 217]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151, 252, 182];
    let expected: Vec<Vec<u8>> = vec![vec![73], vec![73, 61], vec![73, 61, 126], vec![73, 61, 126, 240], vec![73, 61, 126, 240, 106], vec![73, 61, 126, 240, 106, 231], vec![73, 61, 126, 240, 106, 231, 209], vec![73, 61, 126, 240, 106, 231, 209, 6], vec![73, 61, 126, 240, 106, 231, 209, 6, 71], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151, 252], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151, 252, 182]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![123];
    let expected: Vec<Vec<u8>> = vec![vec![123]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![14, 220, 204];
    let expected: Vec<Vec<u8>> = vec![vec![14], vec![14, 220], vec![14, 220, 204]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![243];
    let expected: Vec<Vec<u8>> = vec![vec![243]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![234, 200, 140];
    let expected: Vec<Vec<u8>> = vec![vec![234], vec![234, 200], vec![234, 200, 140]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![117, 172];
    let expected: Vec<Vec<u8>> = vec![vec![117], vec![117, 172]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![39];
    let expected: Vec<Vec<u8>> = vec![vec![39]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![138, 62, 131, 206, 255];
    let expected: Vec<Vec<u8>> = vec![vec![138], vec![138, 62], vec![138, 62, 131], vec![138, 62, 131, 206], vec![138, 62, 131, 206, 255]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![11, 79, 192, 96];
    let expected: Vec<Vec<u8>> = vec![vec![11], vec![11, 79], vec![11, 79, 192], vec![11, 79, 192, 96]];
    assert_eq!(all_prefixes(&arg_0), expected);
}