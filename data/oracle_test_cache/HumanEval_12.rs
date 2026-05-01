#[test]
fn oracle_case_0() {
    let arg_0: Vec<Vec<u8>> = vec![vec![57, 216, 69, 160], vec![216], vec![114], vec![102, 38, 172, 207, 44, 9], vec![225, 188, 83, 173, 107, 30], vec![214, 240, 244, 196, 15], vec![83, 26, 87, 42, 205]];
    let expected_inner: Vec<u8> = vec![102, 38, 172, 207, 44, 9];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<Vec<u8>> = vec![];
    let expected: Option<&Vec<u8>> = None;
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<Vec<u8>> = vec![vec![173, 144], vec![186, 93, 96, 112], vec![104, 52, 112, 91, 39, 140, 18], vec![221, 141, 255, 176, 26, 235, 188], vec![220, 16, 13, 113, 218, 22, 198], vec![243, 246, 122, 85]];
    let expected_inner: Vec<u8> = vec![104, 52, 112, 91, 39, 140, 18];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<Vec<u8>> = vec![vec![79, 12], vec![59, 184], vec![143]];
    let expected_inner: Vec<u8> = vec![79, 12];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<Vec<u8>> = vec![vec![201, 125], vec![46, 159], vec![165, 42, 170], vec![197, 82, 8, 79, 15, 19], vec![208], vec![13, 67, 136, 215, 70, 123, 102], vec![222, 241, 30, 132, 20, 213, 220], vec![102, 194, 50, 180, 96], vec![138, 251, 63], vec![47, 207, 101, 177, 188], vec![212]];
    let expected_inner: Vec<u8> = vec![13, 67, 136, 215, 70, 123, 102];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<Vec<u8>> = vec![vec![236], vec![155, 222, 13, 49], vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39, 143, 201, 149, 4, 151, 212, 226, 173, 250], vec![237, 42, 84, 173, 153, 53], vec![0], vec![130, 201, 160, 203, 213, 115, 208], vec![20, 119, 199, 61, 215, 23, 61, 108], vec![76, 225, 99, 208, 128], vec![250, 99], vec![151, 122, 113, 144], vec![152], vec![41], vec![71, 7, 18, 63, 217], vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151, 252, 182], vec![123], vec![14, 220, 204], vec![243], vec![234, 200, 140], vec![117, 172]];
    let expected_inner: Vec<u8> = vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151, 252, 182];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<Vec<u8>> = vec![vec![138, 62, 131, 206, 255, 141]];
    let expected_inner: Vec<u8> = vec![138, 62, 131, 206, 255, 141];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<Vec<u8>> = vec![vec![56, 208, 227, 232], vec![221, 168, 40, 35, 111, 200], vec![]];
    let expected_inner: Vec<u8> = vec![221, 168, 40, 35, 111, 200];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<Vec<u8>> = vec![vec![156], vec![76, 160, 248, 233, 118, 89, 34], vec![45, 53], vec![], vec![], vec![233, 101, 161], vec![255, 63]];
    let expected_inner: Vec<u8> = vec![76, 160, 248, 233, 118, 89, 34];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<Vec<u8>> = vec![vec![160, 190], vec![], vec![100], vec![6, 100, 55, 242, 245], vec![64, 121, 214, 201], vec![181, 242], vec![204, 210, 230, 207, 161, 32]];
    let expected_inner: Vec<u8> = vec![204, 210, 230, 207, 161, 32];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<Vec<u8>> = vec![vec![165], vec![2], vec![], vec![76], vec![65, 45, 245, 222, 175, 17], vec![73, 251, 109, 7, 254, 232, 199, 124]];
    let expected_inner: Vec<u8> = vec![73, 251, 109, 7, 254, 232, 199, 124];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<Vec<u8>> = vec![vec![20, 222]];
    let expected_inner: Vec<u8> = vec![20, 222];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![61, 154, 244, 180, 89, 214, 105, 48, 188]];
    let expected_inner: Vec<u8> = vec![61, 154, 244, 180, 89, 214, 105, 48, 188];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![200, 38, 44, 173, 0], vec![], vec![109, 248, 32, 42, 60, 12, 138, 238, 166, 225], vec![215, 188], vec![100, 192], vec![242, 238, 174, 181, 195], vec![75], vec![80, 67, 245, 81, 111, 44, 129, 105, 203], vec![168, 104], vec![], vec![83], vec![], vec![108, 248, 188, 243, 183, 142, 96], vec![163], vec![214, 185, 98, 122, 204, 223, 68, 183, 239, 6, 59]];
    let expected_inner: Vec<u8> = vec![214, 185, 98, 122, 204, 223, 68, 183, 239, 6, 59];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<Vec<u8>> = vec![vec![82, 16, 58], vec![], vec![36, 201, 239], vec![129, 236, 32, 217], vec![124, 24, 4, 167, 37, 106, 229, 207, 234, 93, 31, 166, 167, 126, 149, 223, 169, 66, 146], vec![11, 219, 61], vec![], vec![241, 19, 125, 24, 252, 230], vec![35, 115, 102, 247], vec![174, 225, 237, 1, 215], vec![78, 42, 3, 1, 153]];
    let expected_inner: Vec<u8> = vec![124, 24, 4, 167, 37, 106, 229, 207, 234, 93, 31, 166, 167, 126, 149, 223, 169, 66, 146];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<Vec<u8>> = vec![vec![149], vec![], vec![189, 204, 239, 40, 64, 91, 253, 154, 244, 74, 217, 177, 139, 201, 69, 246]];
    let expected_inner: Vec<u8> = vec![189, 204, 239, 40, 64, 91, 253, 154, 244, 74, 217, 177, 139, 201, 69, 246];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![59, 173], vec![3, 221, 189, 192], vec![119], vec![71, 241, 229, 40], vec![], vec![29, 180, 9, 22], vec![20, 75], vec![], vec![], vec![111, 90, 111, 231]];
    let expected_inner: Vec<u8> = vec![3, 221, 189, 192];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<Vec<u8>> = vec![vec![]];
    let expected_inner: Vec<u8> = vec![];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<Vec<u8>> = vec![vec![6, 31, 67, 168, 186, 210, 198, 59], vec![155, 124, 254], vec![210, 76, 56]];
    let expected_inner: Vec<u8> = vec![6, 31, 67, 168, 186, 210, 198, 59];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![178, 103], vec![208, 233, 134, 8, 154, 57, 190, 126]];
    let expected_inner: Vec<u8> = vec![208, 233, 134, 8, 154, 57, 190, 126];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<Vec<u8>> = vec![vec![26, 209, 85, 235, 59, 167, 249, 183], vec![], vec![68, 250, 184], vec![209, 28, 18, 65, 216], vec![229, 104, 57, 2, 102, 166, 247, 150, 29, 247, 132], vec![228, 19, 207, 178, 44, 252, 181]];
    let expected_inner: Vec<u8> = vec![229, 104, 57, 2, 102, 166, 247, 150, 29, 247, 132];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<Vec<u8>> = vec![vec![253, 196, 43, 51, 168, 112, 59, 128, 47, 142, 195, 10], vec![13, 50, 216, 124, 198, 2, 141], vec![97, 194], vec![6]];
    let expected_inner: Vec<u8> = vec![253, 196, 43, 51, 168, 112, 59, 128, 47, 142, 195, 10];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<Vec<u8>> = vec![vec![150, 242, 112, 194, 160, 111, 200, 190, 36], vec![247, 85, 245, 255, 177, 153, 42, 67]];
    let expected_inner: Vec<u8> = vec![150, 242, 112, 194, 160, 111, 200, 190, 36];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<Vec<u8>> = vec![vec![16, 175, 63], vec![], vec![], vec![], vec![208, 197, 90, 97], vec![197, 165]];
    let expected_inner: Vec<u8> = vec![208, 197, 90, 97];
    let expected: Option<&Vec<u8>> = Some(&expected_inner);
    assert_eq!(longest(&arg_0), expected);
}