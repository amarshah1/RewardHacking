#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![1876440458, -865411396, 1283312818];
    let expected: Vec<i32> = vec![1876440459, -865411395, 1283312819];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![826456088, -777591123, 1633444115, 659440559, -168578568, 1549512161, 318568684];
    let expected: Vec<i32> = vec![826456089, -777591122, 1633444116, 659440560, -168578567, 1549512162, 318568685];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![1593221275, -833004066, 2135566861, -849701583];
    let expected: Vec<i32> = vec![1593221276, -833004065, 2135566862, -849701582];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![997363241, 914301792];
    let expected: Vec<i32> = vec![997363242, 914301793];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![604724362, -1925395052, 2119177051];
    let expected: Vec<i32> = vec![604724363, -1925395051, 2119177052];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-1597709645, -1455586161, -1275283617, -2125056783, -2101915285, -1107692979];
    let expected: Vec<i32> = vec![-1597709644, -1455586160, -1275283616, -2125056782, -2101915284, -1107692978];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-472246, -928526049];
    let expected: Vec<i32> = vec![-472245, -928526048];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![413260799];
    let expected: Vec<i32> = vec![413260800];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![1241175959, -1643573804, 777203170, 1073772461];
    let expected: Vec<i32> = vec![1241175960, -1643573803, 777203171, 1073772462];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-1516440403, 1319414937, 1522558773, -1778618592, -436833230, 1564081920, -1307377852, 1551828602, -1531873199, 934288170];
    let expected: Vec<i32> = vec![-1516440402, 1319414938, 1522558774, -1778618591, -436833229, 1564081921, -1307377851, 1551828603, -1531873198, 934288171];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![186832587, 65388501, -556109709, 877940176, 1689288035];
    let expected: Vec<i32> = vec![186832588, 65388502, -556109708, 877940177, 1689288036];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![458404373, 799618522];
    let expected: Vec<i32> = vec![458404374, 799618523];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-279019145, -1686290489];
    let expected: Vec<i32> = vec![-279019144, -1686290488];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-1293949199, 49405441, 1528295596, 35644495];
    let expected: Vec<i32> = vec![-1293949198, 49405442, 1528295597, 35644496];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-715431199, -1904096157];
    let expected: Vec<i32> = vec![-715431198, -1904096156];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-1053297670, 1210259811];
    let expected: Vec<i32> = vec![-1053297669, 1210259812];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![1139241714];
    let expected: Vec<i32> = vec![1139241715];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-1987119209];
    let expected: Vec<i32> = vec![-1987119208];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![-1156694135, -671675699];
    let expected: Vec<i32> = vec![-1156694134, -671675698];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![-834903993, 938469013, 131475024, -1434336953, -227538937];
    let expected: Vec<i32> = vec![-834903992, 938469014, 131475025, -1434336952, -227538936];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![1951970809, -915525659, -575770814];
    let expected: Vec<i32> = vec![1951970810, -915525658, -575770813];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![408972949, 836918998];
    let expected: Vec<i32> = vec![408972950, 836918999];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![-1386999124, 1108839919];
    let expected: Vec<i32> = vec![-1386999123, 1108839920];
    assert_eq!(incr_list(arg_0), expected);
}