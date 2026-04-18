#[test]
fn oracle_case_0() {
    let arg_0: u64 = 3;
    let expected: Option<u64> = Some(12);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u64 = 1;
    let expected: Option<u64> = Some(1);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u64 = 7;
    let expected: Option<u64> = Some(125411328000);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u64 = 0;
    let expected: Option<u64> = Some(1);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u64 = 4;
    let expected: Option<u64> = Some(288);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u64 = 6;
    let expected: Option<u64> = Some(24883200);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u64 = 8;
    let expected: Option<u64> = Some(5056584744960000);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u64 = 5;
    let expected: Option<u64> = Some(34560);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u64 = 12;
    let expected: Option<u64> = None;
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u64 = 2;
    let expected: Option<u64> = Some(2);
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u64 = 15;
    let expected: Option<u64> = None;
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u64 = 18;
    let expected: Option<u64> = None;
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u64 = 11;
    let expected: Option<u64> = None;
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u64 = 9;
    let expected: Option<u64> = None;
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u64 = 19;
    let expected: Option<u64> = None;
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u64 = 16;
    let expected: Option<u64> = None;
    assert_eq!(brazilian_factorial_impl(arg_0), expected);
}