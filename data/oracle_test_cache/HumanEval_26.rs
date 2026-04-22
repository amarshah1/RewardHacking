#[test]
fn oracle_case_0() {
    let arg_0: Vec<i64> = vec![3619i64, -1147i64, 1760i64, 441i64, 3451i64, 1760i64, 3642i64];
    let expected: Vec<i64> = vec![3619, -1147, 441, 3451, 3642];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i64> = vec![-348i64, -348i64, 4885i64, 4552i64, 4552i64, 4552i64, 4273i64, 3084i64];
    let expected: Vec<i64> = vec![4885, 4273, 3084];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i64> = vec![4346i64, -3248i64, 4346i64, 2152i64, -3248i64, 2152i64];
    let expected: Vec<i64> = vec![];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i64> = vec![-4553i64, 3130i64, 3130i64];
    let expected: Vec<i64> = vec![-4553];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i64> = vec![3846i64, -2364i64, 769i64, 3846i64, 888i64, -2364i64, -431i64, -3673i64, 769i64, -3673i64, 769i64, -4369i64];
    let expected: Vec<i64> = vec![888, -431, -4369];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i64> = vec![3606i64, 646i64, 1483i64, -498i64, 1483i64, 1483i64, -624i64, 646i64];
    let expected: Vec<i64> = vec![3606, -498, -624];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i64> = vec![2019i64, 2019i64, 2019i64, -4980i64, 924i64, 924i64, -2048i64, -4980i64, -2048i64, -1794i64, -2048i64, -1794i64];
    let expected: Vec<i64> = vec![];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i64> = vec![4191i64, 155i64, 948i64, 3952i64, 3952i64];
    let expected: Vec<i64> = vec![4191, 155, 948];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i64> = vec![1444i64, 4535i64, 4535i64, -535i64, 4535i64, -535i64, -535i64, 1444i64];
    let expected: Vec<i64> = vec![];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i64> = vec![-4854i64, -2756i64, -1699i64, -502i64, 3413i64, -502i64, -4882i64, -502i64, -2355i64, -4854i64, -4882i64, -2756i64, -2355i64, 3413i64, -2241i64, -4882i64];
    let expected: Vec<i64> = vec![-1699, -2241];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i64> = vec![-1203i64, -1203i64, 594i64, -1203i64, 594i64];
    let expected: Vec<i64> = vec![];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i64> = vec![-498i64, -1216i64, -980i64, 4616i64, 4616i64, 2282i64, -924i64, -1657i64, 2282i64, -2163i64, -1657i64, -4732i64, -994i64, 3925i64, -663i64, -663i64, -4052i64, -1657i64, -924i64, -1216i64, -4732i64, -980i64, -4732i64, -980i64, -924i64];
    let expected: Vec<i64> = vec![-498, -2163, -994, 3925, -4052];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i64> = vec![-1176i64, 4450i64, 4450i64, 4450i64, -3092i64, -648i64, -2541i64, 2579i64];
    let expected: Vec<i64> = vec![-1176, -3092, -648, -2541, 2579];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i64> = vec![1272i64, 4301i64, 3467i64, 4939i64, -1411i64, 3467i64, 3105i64, 4939i64, -193i64, 3467i64, 3105i64, 3501i64, 3501i64, 4939i64, -4903i64];
    let expected: Vec<i64> = vec![1272, 4301, -1411, -193, -4903];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i64> = vec![2112i64, -1336i64, 4925i64, 2343i64, 2343i64, -1336i64, -4854i64, -1336i64, 2343i64, 2112i64, -563i64];
    let expected: Vec<i64> = vec![4925, -4854, -563];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i64> = vec![-3030i64, 1600i64, -3963i64, -3014i64, 4010i64, -4832i64, 4934i64, -3963i64, -3285i64, -4832i64, 1600i64, -3963i64, -4193i64, -3030i64, -2630i64, 4979i64, -3285i64, -3014i64, -3285i64, 4979i64, 636i64, -2684i64, -3014i64, -4832i64, 4010i64, -3030i64, -2684i64, 4979i64, 4934i64];
    let expected: Vec<i64> = vec![-4193, -2630, 636];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i64> = vec![-2595i64, -3934i64, -3934i64, -1075i64, -2595i64, -348i64, -2595i64, -3934i64];
    let expected: Vec<i64> = vec![-1075, -348];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i64> = vec![993i64, 3217i64, 3109i64, 3217i64, 3217i64, 3109i64, 3109i64, 4188i64, 850i64, -176i64, 1913i64, 993i64, -712i64, -4800i64, -176i64, -4800i64, -4800i64, 993i64];
    let expected: Vec<i64> = vec![4188, 850, 1913, -712];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i64> = vec![-4421i64, -4421i64, -4421i64];
    let expected: Vec<i64> = vec![];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i64> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    let expected: Vec<i64> = vec![1551, 111, 2262, 3875, -2007, -1063, -3306, 3752, -4255, -3776, 4032, 3718, -623, 3952, 4899, -4881, -4629, -3851, -3317];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i64> = vec![-3063, -2713, 3574, 3640, 1331, 4432];
    let expected: Vec<i64> = vec![-3063, -2713, 3574, 3640, 1331, 4432];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i64> = vec![-3881, -4900, 176, -4043];
    let expected: Vec<i64> = vec![-3881, -4900, 176, -4043];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i64> = vec![];
    let expected: Vec<i64> = vec![];
    assert_eq!(remove_duplicates(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i64> = vec![2605, 2376, -199, 416, 2862, 296, -4084];
    let expected: Vec<i64> = vec![2605, 2376, -199, 416, 2862, 296, -4084];
    assert_eq!(remove_duplicates(&arg_0), expected);
}