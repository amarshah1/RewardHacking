#[test]
fn oracle_case_0() {
    let arg_0: u32 = 2147483644;
    let arg_1: u32 = 2147483646;
    let expected: i32 = 2147483646;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 2147483646;
    let arg_1: u32 = 2147483640;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 2147483647;
    let arg_1: u32 = 2147483644;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 2147483646;
    let arg_1: u32 = 2147483643;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 2147483641;
    let arg_1: u32 = 2147483646;
    let expected: i32 = 2147483646;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 2147483640;
    let arg_1: u32 = 2147483639;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 2147483644;
    let arg_1: u32 = 2147483642;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 2147483646;
    let arg_1: u32 = 2147483635;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 2147483642;
    let arg_1: u32 = 2147483644;
    let expected: i32 = 2147483644;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 2147483644;
    let arg_1: u32 = 2147483645;
    let expected: i32 = 2147483644;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 2147483645;
    let arg_1: u32 = 2147483641;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 2147483645;
    let arg_1: u32 = 2147483644;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 2147483645;
    let arg_1: u32 = 2147483647;
    let expected: i32 = 2147483646;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 2147483645;
    let arg_1: u32 = 2147483642;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 2147483642;
    let arg_1: u32 = 2147483643;
    let expected: i32 = 2147483642;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 2147483647;
    let arg_1: u32 = 2147483645;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 2147483646;
    let arg_1: u32 = 2147483641;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 2147483643;
    let arg_1: u32 = 2147483632;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 2147483642;
    let arg_1: u32 = 2147483646;
    let expected: i32 = 2147483646;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 2147483646;
    let arg_1: u32 = 2147483645;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 2147483643;
    let arg_1: u32 = 2147483647;
    let expected: i32 = 2147483646;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 2147483643;
    let arg_1: u32 = 2147483644;
    let expected: i32 = 2147483644;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 2147483646;
    let arg_1: u32 = 2147483647;
    let expected: i32 = 2147483646;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 2147483647;
    let arg_1: u32 = 2147483647;
    let expected: i32 = -1;
    assert_eq!(choose_num(arg_0, arg_1), expected);
}