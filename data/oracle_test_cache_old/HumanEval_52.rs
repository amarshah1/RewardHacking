#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![-1903i32, 197i32, 1804i32, 2135i32, -1890i32, -2777i32, 2055i32, 2135i32, 1740i32];
    let arg_1: i32 = 2298i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![-1280i32, -48875i32, -33808i32, -7711i32, -851i32, -304i32, -5188i32, -1022i32, -13927i32, -936i32, -1070i32, -3086i32];
    let arg_1: i32 = -846i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-44263i32, 585i32, -47823i32, -481i32, -4654i32, 164i32, 386i32, -4004i32, -261i32, 313i32, 650i32, 133i32, -45075i32, -285i32];
    let arg_1: i32 = 694i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![-23995i32, -35927i32, -924i32, -43694i32, -4747i32, -2983i32, 3032i32, -31000i32, -1506i32, -701i32, -7541i32, -8367i32, -3893i32];
    let arg_1: i32 = 607i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![-23196i32, 2833i32, 3903i32, 4213i32, 3729i32, 3752i32, 3998i32, -45451i32];
    let arg_1: i32 = 4270i32;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-4938i32, -4720i32, -47295i32, -4634i32, -6956i32, -4376i32, -4488i32, 31594i32];
    let arg_1: i32 = -4362i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![-3881, -4900, 176, -4043, -1667, -1758];
    let arg_1: i32 = 2153;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![-3618i32, -3079i32, -32462i32, -9001i32, -34548i32, -2411i32, 924i32, 934i32, 1669i32, 39392i32, -25170i32, 34147i32];
    let arg_1: i32 = 1361i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![3746];
    let arg_1: i32 = 4610;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-32015i32, 1853i32, 335i32, 129i32, -2097i32, -37516i32, -2091i32, 2224i32, -1692i32, 1531i32, 22641i32, 3994i32, 2510i32];
    let arg_1: i32 = 2350i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-4671, -4417];
    let arg_1: i32 = -1662;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-2353i32, -1238i32, -4823i32, -1574i32, -1359i32, -1604i32, -18380i32, -1132i32, 41147i32, -1442i32];
    let arg_1: i32 = -1022i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = -569;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-6705i32, -4450i32, -22831i32, -8437i32, -49784i32, -3513i32, -30937i32, -44329i32, -3985i32, -27589i32, 361i32, 26384i32, -14108i32, -29084i32, -30797i32, 35337i32, -3617i32];
    let arg_1: i32 = -3449i32;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![3766, 2853, -4602, -4828, -4074, 3171, -2434];
    let arg_1: i32 = 4461;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let arg_1: i32 = -4174;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = -3042;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-3063, -2713, 3574, 3640];
    let arg_1: i32 = 1331;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = 1865;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![2605, 2376];
    let arg_1: i32 = -199;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = -2801;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![-3445, -1724, 1626, 4803, 409, 2900, -3605, -1675, 3413];
    let arg_1: i32 = -1795;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![];
    let arg_1: i32 = -1542;
    let expected: bool = true;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![650, -2147];
    let arg_1: i32 = 242;
    let expected: bool = false;
    assert_eq!(below_threshold(&arg_0, arg_1), expected);
}