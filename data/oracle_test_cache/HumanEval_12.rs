#[test]
fn oracle_case_0() {
    let arg_0: Vec<Vec<u8>> = vec![vec![111], vec![255, 255, 255, 255], vec![255, 255, 255, 149, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], vec![255, 255, 255, 255, 176, 255], vec![255], vec![255, 255, 255, 255, 199, 255, 255], vec![255, 255, 255, 255, 255, 255, 255, 255], vec![255, 255, 255, 255, 255], vec![255, 255], vec![255, 255, 255, 255], vec![255], vec![255], vec![255, 255, 255, 255, 255], vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 167, 255, 255, 255, 255, 255, 255, 255, 255, 255], vec![255], vec![255, 255, 255], vec![255], vec![255, 255, 255], vec![255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 167, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
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
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255, 255, 255], vec![255, 255, 255, 255, 255, 255], vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255], vec![255, 255, 255, 255, 255, 255, 255], vec![255, 255], vec![], vec![], vec![255, 255, 255], vec![255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255], vec![], vec![255], vec![255, 255, 255, 255, 255], vec![255, 255, 255, 255], vec![255, 255], vec![255, 255, 255, 255, 255, 132]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 132]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255], vec![255], vec![], vec![255], vec![255, 255, 255, 255, 255, 255], vec![255, 255, 255, 255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![255, 255, 255, 255, 255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![255, 255, 255, 255, 255], vec![], vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255], vec![130, 255], vec![255, 255], vec![255, 255, 255, 255, 255], vec![255], vec![255, 255, 255, 255, 137, 255, 255, 255, 255], vec![255, 255], vec![], vec![255], vec![], vec![255, 255, 255, 255, 255, 255, 255], vec![255], vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 156]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 156]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<Vec<u8>> = vec![vec![179, 255, 255], vec![], vec![255, 255, 255], vec![255, 255, 255, 255], vec![224, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], vec![255, 242, 255], vec![], vec![230, 255, 255, 255, 255, 255], vec![255, 255, 255, 255], vec![255, 255, 255, 255, 255], vec![255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![224, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255], vec![], vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![255, 255], vec![255, 255, 255, 255], vec![255], vec![255, 255, 255, 255], vec![], vec![255, 255, 255, 255], vec![255, 175], vec![], vec![], vec![255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<Vec<u8>> = vec![vec![]];
    let expected: Option<&Vec<u8>> = Some(vec![]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255, 255, 255, 255, 255, 255, 193], vec![255, 255, 255], vec![255, 173, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 193]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![255, 255], vec![255, 255, 255, 255, 255, 225, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 225, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255, 255, 255, 255, 255, 255, 255], vec![], vec![255, 255, 255], vec![255, 255, 255, 255, 255], vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], vec![255, 255, 255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], vec![255, 255, 255, 255, 255, 210, 255], vec![255, 255], vec![255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255, 255, 255, 255, 255, 255, 255, 255], vec![255, 182, 255, 255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255, 255], vec![], vec![], vec![], vec![255, 255, 255, 255], vec![255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255], vec![255, 255, 255, 255], vec![255], vec![255], vec![255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255], vec![238, 255, 255, 255, 255], vec![255, 255, 255, 255], vec![], vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255], vec![255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<Vec<u8>> = vec![vec![255, 255], vec![255], vec![255, 255, 255], vec![255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<Vec<u8>> = vec![vec![], vec![255], vec![255], vec![255], vec![159, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]];
    let expected: Option<&Vec<u8>> = Some(vec![159, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    assert_eq!(longest(&arg_0), expected);
}