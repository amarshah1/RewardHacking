#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![1876440458, -865411396, 1283312818];
    let expected: Vec<i32> = vec![-865411396, 1283312818, 1876440458];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![826456088, -777591123, 1633444115, 659440559, -168578568, 1549512161, 318568684];
    let expected: Vec<i32> = vec![-777591123, -168578568, 318568684, 659440559, 826456088, 1549512161, 1633444115];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![1593221275, -833004066, 2135566861, -849701583];
    let expected: Vec<i32> = vec![-849701583, -833004066, 1593221275, 2135566861];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![997363241, 914301792];
    let expected: Vec<i32> = vec![914301792, 997363241];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![604724362, -1925395052, 2119177051];
    let expected: Vec<i32> = vec![-1925395052, 604724362, 2119177051];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-1597709645, -1455586161, -1275283617, -2125056783, -2101915285, -1107692979];
    let expected: Vec<i32> = vec![-2125056783, -2101915285, -1597709645, -1455586161, -1275283617, -1107692979];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-472246, -928526049];
    let expected: Vec<i32> = vec![-928526049, -472246];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![413260799];
    let expected: Vec<i32> = vec![413260799];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![1241175959, -1643573804, 777203170, 1073772461];
    let expected: Vec<i32> = vec![-1643573804, 777203170, 1073772461, 1241175959];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-1516440403, 1319414937, 1522558773, -1778618592, -436833230, 1564081920, -1307377852, 1551828602, -1531873199, 934288170];
    let expected: Vec<i32> = vec![-1778618592, -1531873199, -1516440403, -1307377852, -436833230, 934288170, 1319414937, 1522558773, 1551828602, 1564081920];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![186832587, 65388501, -556109709, 877940176, 1689288035];
    let expected: Vec<i32> = vec![-556109709, 65388501, 186832587, 877940176, 1689288035];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![458404373, 799618522];
    let expected: Vec<i32> = vec![458404373, 799618522];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-279019145, -1686290489];
    let expected: Vec<i32> = vec![-1686290489, -279019145];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-1293949199, 49405441, 1528295596, 35644495];
    let expected: Vec<i32> = vec![-1293949199, 35644495, 49405441, 1528295596];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-715431199, -1904096157];
    let expected: Vec<i32> = vec![-1904096157, -715431199];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-1053297670, 1210259811];
    let expected: Vec<i32> = vec![-1053297670, 1210259811];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![1139241714];
    let expected: Vec<i32> = vec![1139241714];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-1987119209];
    let expected: Vec<i32> = vec![-1987119209];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![-1156694135, -671675699];
    let expected: Vec<i32> = vec![-1156694135, -671675699];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![-834903993, 938469013, 131475024, -1434336953, -227538937];
    let expected: Vec<i32> = vec![-1434336953, -834903993, -227538937, 131475024, 938469013];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![1951970809, -915525659, -575770814];
    let expected: Vec<i32> = vec![-915525659, -575770814, 1951970809];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![408972949, 836918998];
    let expected: Vec<i32> = vec![408972949, 836918998];
    assert_eq!(unique(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![-1386999124, 1108839919];
    let expected: Vec<i32> = vec![-1386999124, 1108839919];
    assert_eq!(unique(arg_0), expected);
}