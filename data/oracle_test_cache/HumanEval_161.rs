#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![255, 255, 111, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 79, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 149, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 149, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255];
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
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![255];
    let expected: Vec<u8> = vec![255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![255, 255];
    let expected: Vec<u8> = vec![255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 167, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 167, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 132];
    let expected: Vec<u8> = vec![132, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![255, 172, 255, 130];
    let expected: Vec<u8> = vec![130, 255, 172, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 137, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 137, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 156];
    let expected: Vec<u8> = vec![156, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 179, 255, 255, 255, 255, 255, 231];
    let expected: Vec<u8> = vec![231, 255, 255, 255, 255, 255, 179, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![224, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 224];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![255, 242, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 242, 255];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![230, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 230];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![209];
    let expected: Vec<u8> = vec![209];
    assert_eq!(solve(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    let expected: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
    assert_eq!(solve(&arg_0), expected);
}