#[test]
fn oracle_case_0() {
    let arg_0: u32 = 2147482435;
    let arg_1: u32 = 2147481908;
    let arg_2: u32 = 2147479131;
    let expected: [u32; 2] = [4294961566, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 2147480954;
    let arg_1: u32 = 2147479643;
    let arg_2: u32 = 2147480438;
    let expected: [u32; 2] = [4294960597, 795];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 2147480155;
    let arg_1: u32 = 2147482239;
    let arg_2: u32 = 2147479445;
    let expected: [u32; 2] = [4294959600, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 2147483314;
    let arg_1: u32 = 2147479900;
    let arg_2: u32 = 2147481299;
    let expected: [u32; 2] = [4294963214, 1399];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 2147479636;
    let arg_1: u32 = 2147478950;
    let arg_2: u32 = 2147483171;
    let expected: [u32; 2] = [4294958586, 4221];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 2147479783;
    let arg_1: u32 = 2147480528;
    let arg_2: u32 = 2147483306;
    let expected: [u32; 2] = [4294960311, 2778];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 2147483201;
    let arg_1: u32 = 2147480923;
    let arg_2: u32 = 2147482096;
    let expected: [u32; 2] = [4294964124, 1173];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 2147483143;
    let arg_1: u32 = 2147483536;
    let arg_2: u32 = 2147479739;
    let expected: [u32; 2] = [4294962882, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 2147481385;
    let arg_1: u32 = 2147481222;
    let arg_2: u32 = 2147479772;
    let expected: [u32; 2] = [4294961157, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 2147479863;
    let arg_1: u32 = 2147481640;
    let arg_2: u32 = 2147479121;
    let expected: [u32; 2] = [4294958984, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 2147482584;
    let arg_1: u32 = 2147479661;
    let arg_2: u32 = 2147480341;
    let expected: [u32; 2] = [4294962245, 680];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 2147479139;
    let arg_1: u32 = 2147479895;
    let arg_2: u32 = 2147481461;
    let expected: [u32; 2] = [4294959034, 1566];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 2147479392;
    let arg_1: u32 = 2147482384;
    let arg_2: u32 = 2147479871;
    let expected: [u32; 2] = [4294959263, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 2147479979;
    let arg_1: u32 = 2147479615;
    let arg_2: u32 = 2147479461;
    let expected: [u32; 2] = [4294959440, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 2147479929;
    let arg_1: u32 = 2147483493;
    let arg_2: u32 = 2147483024;
    let expected: [u32; 2] = [4294962953, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 2147479818;
    let arg_1: u32 = 2147479695;
    let arg_2: u32 = 2147482028;
    let expected: [u32; 2] = [4294959513, 2333];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 2147478748;
    let arg_1: u32 = 2147482643;
    let arg_2: u32 = 2147478766;
    let expected: [u32; 2] = [4294957514, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 2147479423;
    let arg_1: u32 = 2147479018;
    let arg_2: u32 = 2147480713;
    let expected: [u32; 2] = [4294958441, 1695];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 2147479796;
    let arg_1: u32 = 2147482308;
    let arg_2: u32 = 2147480330;
    let expected: [u32; 2] = [4294960126, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 2147483498;
    let arg_1: u32 = 2147479473;
    let arg_2: u32 = 2147479989;
    let expected: [u32; 2] = [4294962971, 516];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 2147480515;
    let arg_1: u32 = 2147479022;
    let arg_2: u32 = 2147482156;
    let expected: [u32; 2] = [4294959537, 3134];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 2147482884;
    let arg_1: u32 = 2147481139;
    let arg_2: u32 = 2147480584;
    let expected: [u32; 2] = [4294963468, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 2147481504;
    let arg_1: u32 = 2147480934;
    let arg_2: u32 = 2147481646;
    let expected: [u32; 2] = [4294962438, 712];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 2147480073;
    let arg_1: u32 = 2147482267;
    let arg_2: u32 = 2147480007;
    let expected: [u32; 2] = [4294960080, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}