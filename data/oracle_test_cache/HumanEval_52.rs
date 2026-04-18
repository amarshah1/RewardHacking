#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![2154i32, 2135i32, 1875i32, 2110i32, 1943i32, 1847i32, 2296i32, 2262i32, 1740i32];
    let arg_1: i32 = 2298i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![3193i32, 3561i32, 3175i32, 3923i32, 3406i32, 3186i32, 3512i32, 3261i32];
    let arg_1: i32 = 3687i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-1702i32, -1699i32, -1472i32, -1595i32, -1295i32, -1454i32, -1842i32, -1270i32];
    let arg_1: i32 = -1265i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![979i32, 934i32, 999i32, 637i32, 916i32, 576i32, 689i32, 995i32, 1397i32];
    let arg_1: i32 = 1153i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![4390i32, 4018i32, 4389i32, 4053i32, 4165i32, 4362i32, 4284i32, 4429i32];
    let arg_1: i32 = 4434i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-1896i32, -1492i32, -1181i32, -1493i32, -1427i32, -1588i32, -1667i32, -1549i32];
    let arg_1: i32 = -1335i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![1899i32, 2103i32, 2073i32, 1970i32, 1932i32, 1605i32, 1833i32, 1842i32];
    let arg_1: i32 = 2167i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-3522i32, -3371i32, -3018i32, -3602i32, -3335i32, -3156i32, -3441i32, -3296i32];
    let arg_1: i32 = -3106i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-2833i32, -2731i32, -2513i32, -2355i32, -2758i32, -2301i32, -2358i32, -2734i32];
    let arg_1: i32 = -2238i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![2139i32, 1569i32, 1604i32, 1643i32, 1288i32, 1549i32, 1802i32, 1297i32];
    let arg_1: i32 = 1819i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![0, 0, -6];
    let arg_1: i32 = 3;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-7, -2, 4, 0, 11, -2, 0];
    let arg_1: i32 = -3;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![3];
    let arg_1: i32 = 5;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![2, 2, 1, 1, -5];
    let arg_1: i32 = -1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![-2];
    let arg_1: i32 = 0;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-1, 4, -1];
    let arg_1: i32 = -4;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-1, -2, -2];
    let arg_1: i32 = 1;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![3, -2, -2, 0, 5];
    let arg_1: i32 = -3;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![2];
    let arg_1: i32 = 10;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![4, 0, 0, -1, -3, -4, -2, -2, 0, 0, -18, 1, -1, -1, 2];
    let arg_1: i32 = 1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = 3;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![3, 0, -5];
    let arg_1: i32 = -5;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = -3;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![2, 1, 1, -3];
    let arg_1: i32 = -1;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}