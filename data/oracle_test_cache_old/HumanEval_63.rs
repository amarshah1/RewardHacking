#[test]
fn oracle_case_0() {
    let arg_0: u32 = 3;
    let expected: Option<u32> = Some(1);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 1;
    let expected: Option<u32> = Some(0);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 7;
    let expected: Option<u32> = Some(13);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 0;
    let expected: Option<u32> = Some(0);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 4;
    let expected: Option<u32> = Some(2);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 6;
    let expected: Option<u32> = Some(7);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 8;
    let expected: Option<u32> = Some(24);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 5;
    let expected: Option<u32> = Some(4);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 12;
    let expected: Option<u32> = Some(274);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 2;
    let expected: Option<u32> = Some(1);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 15;
    let expected: Option<u32> = Some(1705);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 18;
    let expected: Option<u32> = Some(10609);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 11;
    let expected: Option<u32> = Some(149);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 9;
    let expected: Option<u32> = Some(44);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 19;
    let expected: Option<u32> = Some(19513);
    assert_eq!(fibfib(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 16;
    let expected: Option<u32> = Some(3136);
    assert_eq!(fibfib(arg_0), expected);
}