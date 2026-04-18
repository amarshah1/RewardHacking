#[test]
fn oracle_case_0() {
    let arg_0: u32 = 3;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 1;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 7;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 0;
    let expected: Vec<Option<u32>> = vec![Some(1)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 4;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 6;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 8;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 5;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 12;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5), Some(35), Some(6), Some(48), Some(7)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 2;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 15;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5), Some(35), Some(6), Some(48), Some(7), Some(63), Some(8), Some(80)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 18;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5), Some(35), Some(6), Some(48), Some(7), Some(63), Some(8), Some(80), Some(9), Some(99), Some(10)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 11;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5), Some(35), Some(6), Some(48)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 9;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5), Some(35)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 19;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5), Some(35), Some(6), Some(48), Some(7), Some(63), Some(8), Some(80), Some(9), Some(99), Some(10), Some(120)];
    assert_eq!(tri(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 16;
    let expected: Vec<Option<u32>> = vec![Some(1), Some(3), Some(2), Some(8), Some(3), Some(15), Some(4), Some(24), Some(5), Some(35), Some(6), Some(48), Some(7), Some(63), Some(8), Some(80), Some(9)];
    assert_eq!(tri(arg_0), expected);
}