#[test]
fn oracle_case_0() {
    let arg_0: Vec<Vec<u8>> = vec![vec![178], vec![], vec![24, 173, 19, 175, 248, 225, 236]];
    let expected: Option<&Vec<u8>> = Some(vec![24, 173, 19, 175, 248, 225, 236]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<Vec<u8>> = vec![vec![41, 96, 71, 243, 206, 132], vec![91], vec![179, 143, 95, 241, 107, 77], vec![74, 31]];
    let expected: Option<&Vec<u8>> = Some(vec![41, 96, 71, 243, 206, 132]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<Vec<u8>> = vec![vec![151, 212, 226, 173, 250]];
    let expected: Option<&Vec<u8>> = Some(vec![151, 212, 226, 173, 250]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<Vec<u8>> = vec![vec![50, 0, 68], vec![254, 163, 115], vec![203, 213], vec![99], vec![21, 218], vec![], vec![119, 199], vec![241, 1, 172, 79], vec![225, 99]];
    let expected: Option<&Vec<u8>> = Some(vec![241, 1, 172, 79]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<Vec<u8>> = vec![];
    let expected: Option<&Vec<u8>> = None;
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<Vec<u8>> = vec![vec![242, 218, 180], vec![137, 205, 152]];
    let expected: Option<&Vec<u8>> = Some(vec![242, 218, 180]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<Vec<u8>> = vec![vec![71, 7], vec![249, 229, 66], vec![149, 214], vec![172, 239], vec![164]];
    let expected: Option<&Vec<u8>> = Some(vec![249, 229, 66]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<Vec<u8>> = vec![vec![174], vec![205, 73, 61, 126, 240]];
    let expected: Option<&Vec<u8>> = Some(vec![205, 73, 61, 126, 240]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<Vec<u8>> = vec![vec![151, 252, 182, 248, 135], vec![233], vec![204, 7, 225], vec![1, 175], vec![140], vec![117, 172], vec![], vec![], vec![], vec![11, 79, 192, 96, 165, 56, 208, 227, 232, 28, 181, 57, 224, 183, 51, 80, 221, 168], vec![], vec![45, 89], vec![82, 191, 49, 164, 92, 154, 121, 230, 76], vec![], vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![11, 79, 192, 96, 165, 56, 208, 227, 232, 28, 181, 57, 224, 183, 51, 80, 221, 168]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<Vec<u8>> = vec![vec![220, 233, 101, 161, 46, 108], vec![247, 86, 62, 212], vec![33, 230], vec![], vec![214, 187, 76, 141], vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![220, 233, 101, 161, 46, 108]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<Vec<u8>> = vec![vec![121, 214, 201, 212, 27, 120, 181, 242]];
    let expected: Option<&Vec<u8>> = Some(vec![121, 214, 201, 212, 27, 120, 181, 242]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<Vec<u8>> = vec![vec![137, 123, 204, 74], vec![165, 187, 55, 2], vec![80], vec![], vec![137, 128], vec![96, 12, 201, 207, 105, 73, 251, 109, 7, 254, 232], vec![180]];
    let expected: Option<&Vec<u8>> = Some(vec![96, 12, 201, 207, 105, 73, 251, 109, 7, 254, 232]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<Vec<u8>> = vec![vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<Vec<u8>> = vec![vec![151], vec![98, 147, 186, 76, 39]];
    let expected: Option<&Vec<u8>> = Some(vec![98, 147, 186, 76, 39]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![188, 161, 193], vec![8, 131, 11]];
    let expected: Option<&Vec<u8>> = Some(vec![188, 161, 193]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<Vec<u8>> = vec![vec![0, 102, 144, 164], vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![0, 102, 144, 164]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<Vec<u8>> = vec![vec![173], vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![173]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![], vec![], vec![32, 42, 60], vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![32, 42, 60]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![234, 84], vec![], vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![234, 84]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<Vec<u8>> = vec![vec![242, 238]];
    let expected: Option<&Vec<u8>> = Some(vec![242, 238]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![218, 137], vec![67, 245, 81, 111, 44, 129]];
    let expected: Option<&Vec<u8>> = Some(vec![67, 245, 81, 111, 44, 129]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<Vec<u8>> = vec![vec![104, 160], vec![83]];
    let expected: Option<&Vec<u8>> = Some(vec![104, 160]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<Vec<u8>> = vec![vec![12, 144, 105]];
    let expected: Option<&Vec<u8>> = Some(vec![12, 144, 105]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<Vec<u8>> = vec![vec![234, 177, 27], vec![194, 127, 235, 214], vec![223, 68, 183]];
    let expected: Option<&Vec<u8>> = Some(vec![194, 127, 235, 214]);
    assert_eq!(longest(&arg_0), expected);
}