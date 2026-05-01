#[test]
fn oracle_case_0() {
    let arg_0: u32 = 638;
    let arg_1: u32 = 198;
    let arg_2: u32 = 198;
    let expected: [u32; 2] = [836, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u32 = 20;
    let arg_1: u32 = 769;
    let arg_2: u32 = 769;
    let expected: [u32; 2] = [789, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u32 = 732;
    let arg_1: u32 = 499;
    let arg_2: u32 = 499;
    let expected: [u32; 2] = [1231, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u32 = 388;
    let arg_1: u32 = 920;
    let arg_2: u32 = 920;
    let expected: [u32; 2] = [1308, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u32 = 403;
    let arg_1: u32 = 174;
    let arg_2: u32 = 174;
    let expected: [u32; 2] = [577, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u32 = 688;
    let arg_1: u32 = 459;
    let arg_2: u32 = 459;
    let expected: [u32; 2] = [1147, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u32 = 250;
    let arg_1: u32 = 529;
    let arg_2: u32 = 529;
    let expected: [u32; 2] = [779, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u32 = 861;
    let arg_1: u32 = 201;
    let arg_2: u32 = 201;
    let expected: [u32; 2] = [1062, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u32 = 2100034873;
    let arg_1: u32 = 1996733837;
    let arg_2: u32 = 1876440458;
    let expected: [u32; 2] = [3976475331, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u32 = 1283312818;
    let arg_1: u32 = 1741431595;
    let arg_2: u32 = 1777274431;
    let expected: [u32; 2] = [3024744413, 35842836];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u32 = 1781891621;
    let arg_1: u32 = 543303618;
    let arg_2: u32 = 826456088;
    let expected: [u32; 2] = [2325195239, 283152470];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u32 = 1633444115;
    let arg_1: u32 = 1978905080;
    let arg_2: u32 = 318568684;
    let expected: [u32; 2] = [1952012799, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u32 = 1829242994;
    let arg_1: u32 = 609780125;
    let arg_2: u32 = 1593221275;
    let expected: [u32; 2] = [2439023119, 983441150];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u32 = 2135566861;
    let arg_1: u32 = 1546515010;
    let arg_2: u32 = 1871357804;
    let expected: [u32; 2] = [3682081871, 324842794];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u32 = 914301792;
    let arg_1: u32 = 815587571;
    let arg_2: u32 = 453159044;
    let expected: [u32; 2] = [1367460836, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u32 = 222088596;
    let arg_1: u32 = 2023811015;
    let arg_2: u32 = 1382187498;
    let expected: [u32; 2] = [1604276094, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u32 = 1074899943;
    let arg_1: u32 = 1345951920;
    let arg_2: u32 = 691897487;
    let expected: [u32; 2] = [1766797430, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u32 = 22426865;
    let arg_1: u32 = 1039790669;
    let arg_2: u32 = 1479513581;
    let expected: [u32; 2] = [1062217534, 439722912];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u32 = 2147011402;
    let arg_1: u32 = 672978950;
    let arg_2: u32 = 413260799;
    let expected: [u32; 2] = [2560272201, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u32 = 1781814415;
    let arg_1: u32 = 759406741;
    let arg_2: u32 = 1241175959;
    let expected: [u32; 2] = [2541221156, 481769218];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u32 = 777203170;
    let arg_1: u32 = 1448403962;
    let arg_2: u32 = 445970973;
    let expected: [u32; 2] = [1223174143, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u32 = 287350521;
    let arg_1: u32 = 637527819;
    let arg_2: u32 = 498182125;
    let expected: [u32; 2] = [785532646, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u32 = 1075732308;
    let arg_1: u32 = 1319414937;
    let arg_2: u32 = 368865056;
    let expected: [u32; 2] = [1444597364, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u32 = 1564081920;
    let arg_1: u32 = 1551828602;
    let arg_2: u32 = 934288170;
    let expected: [u32; 2] = [2498370090, 0];
    assert_eq!(eat(arg_0, arg_1, arg_2), expected);
}