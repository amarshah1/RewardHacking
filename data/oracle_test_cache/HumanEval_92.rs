#[test]
fn oracle_case_0() {
    let arg_0: i32 = -3549;
    let arg_1: i32 = -873;
    let arg_2: i32 = -4422;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: i32 = 2147483505;
    let arg_1: i32 = 3773;
    let arg_2: i32 = 2147483647;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: i32 = 1401;
    let arg_1: i32 = -4790;
    let arg_2: i32 = -3389;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: i32 = -2147483087;
    let arg_1: i32 = -3462;
    let arg_2: i32 = -2147483648;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: i32 = 609;
    let arg_1: i32 = 313;
    let arg_2: i32 = 922;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: i32 = 1212;
    let arg_1: i32 = 4516;
    let arg_2: i32 = -4004;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: i32 = 4862;
    let arg_1: i32 = -1384;
    let arg_2: i32 = 3478;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: i32 = -3492;
    let arg_1: i32 = -4202;
    let arg_2: i32 = 3747;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: i32 = 4055;
    let arg_1: i32 = 1072;
    let arg_2: i32 = 5127;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: i32 = 4011;
    let arg_1: i32 = 476;
    let arg_2: i32 = -3119;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: i32 = 3892;
    let arg_1: i32 = -3648;
    let arg_2: i32 = 244;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: i32 = 446;
    let arg_1: i32 = 1551;
    let arg_2: i32 = 111;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: i32 = 3468;
    let arg_1: i32 = 2335;
    let arg_2: i32 = 5803;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: i32 = 2262;
    let arg_1: i32 = 3875;
    let arg_2: i32 = -2007;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: i32 = 1570;
    let arg_1: i32 = 4968;
    let arg_2: i32 = 6538;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: i32 = -1063;
    let arg_1: i32 = -3306;
    let arg_2: i32 = 3752;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: i32 = 1964;
    let arg_1: i32 = 4040;
    let arg_2: i32 = 6004;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: i32 = -4255;
    let arg_1: i32 = -3776;
    let arg_2: i32 = 4032;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: i32 = -3212;
    let arg_1: i32 = -2473;
    let arg_2: i32 = -5685;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: i32 = 3718;
    let arg_1: i32 = -623;
    let arg_2: i32 = 3952;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: i32 = 911;
    let arg_1: i32 = -2920;
    let arg_2: i32 = -2009;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: i32 = 4899;
    let arg_1: i32 = -4881;
    let arg_2: i32 = -4629;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: i32 = -1238;
    let arg_1: i32 = 2776;
    let arg_2: i32 = 1538;
    let expected: bool = true;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: i32 = -3851;
    let arg_1: i32 = -3317;
    let arg_2: i32 = -4174;
    let expected: bool = false;
    assert_eq!(any_int(arg_0, arg_1, arg_2), expected);
}