#[test]
fn oracle_case_0() {
    let arg_0: Vec<u32> = vec![1876440458, 3429555900, 1283312818];
    let expected: (u64, Option<u32>) = (6589309176, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u32> = vec![];
    let expected: (u64, Option<u32>) = (0, Some(1));
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u32> = vec![826456088, 3517376173, 1633444115, 659440559, 4126388728, 1549512161, 318568684];
    let expected: (u64, Option<u32>) = (12631186508, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u32> = vec![1593221275, 3461963230, 2135566861, 3445265713];
    let expected: (u64, Option<u32>) = (10636017079, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u32> = vec![997363241, 914301792];
    let expected: (u64, Option<u32>) = (1911665033, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u32> = vec![604724362, 2369572244, 2119177051];
    let expected: (u64, Option<u32>) = (5093473657, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u32> = vec![2697257651, 2839381135, 3019683679, 2169910513, 2193052011, 3187274317];
    let expected: (u64, Option<u32>) = (16106559306, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u32> = vec![4294495050, 3366441247];
    let expected: (u64, Option<u32>) = (7660936297, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u32> = vec![413260799];
    let expected: (u64, Option<u32>) = (413260799, Some(413260799));
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u32> = vec![1241175959, 2651393492, 777203170, 1073772461];
    let expected: (u64, Option<u32>) = (5743545082, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u32> = vec![2778526893, 1319414937, 1522558773, 2516348704, 3858134066, 1564081920, 2987589444, 1551828602, 2763094097, 934288170];
    let expected: (u64, Option<u32>) = (21795865606, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u32> = vec![186832587, 65388501, 3738857587, 877940176, 1689288035];
    let expected: (u64, Option<u32>) = (6558306886, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u32> = vec![458404373, 799618522];
    let expected: (u64, Option<u32>) = (1258022895, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u32> = vec![4015948151, 2608676807];
    let expected: (u64, Option<u32>) = (6624624958, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u32> = vec![3001018097, 49405441, 1528295596, 35644495];
    let expected: (u64, Option<u32>) = (4614363629, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u32> = vec![3579536097, 2390871139];
    let expected: (u64, Option<u32>) = (5970407236, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u32> = vec![3241669626, 1210259811];
    let expected: (u64, Option<u32>) = (4451929437, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u32> = vec![1139241714];
    let expected: (u64, Option<u32>) = (1139241714, Some(1139241714));
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u32> = vec![2307848087];
    let expected: (u64, Option<u32>) = (2307848087, Some(2307848087));
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u32> = vec![3138273161, 3623291597];
    let expected: (u64, Option<u32>) = (6761564758, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u32> = vec![3460063303, 938469013, 131475024, 2860630343, 4067428359];
    let expected: (u64, Option<u32>) = (11458066042, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u32> = vec![1951970809, 3379441637, 3719196482];
    let expected: (u64, Option<u32>) = (9050608928, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u32> = vec![408972949, 836918998];
    let expected: (u64, Option<u32>) = (1245891947, None);
    assert_eq!(sum_product(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u32> = vec![2907968172, 1108839919];
    let expected: (u64, Option<u32>) = (4016808091, None);
    assert_eq!(sum_product(arg_0), expected);
}