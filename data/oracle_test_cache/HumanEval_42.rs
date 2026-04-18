#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![2147482095, 2147483142, 2147483535, 2147479738, 2147481384, 2147481221, 2147479771, 2147479862, 2147481639, 2147479120, 2147482583, 2147479660, 2147480340, 2147479138, 2147479894, 2147481460, 2147479391, 2147482383, 2147479870];
    let expected: Vec<i32> = vec![2147482096, 2147483143, 2147483536, 2147479739, 2147481385, 2147481222, 2147479772, 2147479863, 2147481640, 2147479121, 2147482584, 2147479661, 2147480341, 2147479139, 2147479895, 2147481461, 2147479392, 2147482384, 2147479871];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![2147479795, 2147482307, 2147480329, 2147483497, 2147479472, 2147479988, 2147480514, 2147479021, 2147482155, 2147482883, 2147481138, 2147480583, 2147481503, 2147480933];
    let expected: Vec<i32> = vec![2147479796, 2147482308, 2147480330, 2147483498, 2147479473, 2147479989, 2147480515, 2147479022, 2147482156, 2147482884, 2147481139, 2147480584, 2147481504, 2147480934];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![2147479214, 2147483230, 2147482865, 2147482057, 2147481565, 2147483211];
    let expected: Vec<i32> = vec![2147479215, 2147483231, 2147482866, 2147482058, 2147481566, 2147483212];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![2147481979, 2147481911, 2147481888, 2147482151, 2147481493, 2147480735, 2147480769];
    let expected: Vec<i32> = vec![2147481980, 2147481912, 2147481889, 2147482152, 2147481494, 2147480736, 2147480770];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![2147481041];
    let expected: Vec<i32> = vec![2147481042];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![2147482470];
    let expected: Vec<i32> = vec![2147482471];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![2147483230];
    let expected: Vec<i32> = vec![2147483231];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![2147480201, 2147479809, 2147481922, 2147479904, 2147482020, 2147480053, 2147478843, 2147482639];
    let expected: Vec<i32> = vec![2147480202, 2147479810, 2147481923, 2147479905, 2147482021, 2147480054, 2147478844, 2147482640];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![2147481971, 2147482738, 2147480233, 2147480673, 2147481851];
    let expected: Vec<i32> = vec![2147481972, 2147482739, 2147480234, 2147480674, 2147481852];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![2147479900, 2147481948];
    let expected: Vec<i32> = vec![2147479901, 2147481949];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![2147478975, 2147478782, 2147479229, 2147480367];
    let expected: Vec<i32> = vec![2147478976, 2147478783, 2147479230, 2147480368];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![2147480473];
    let expected: Vec<i32> = vec![2147480474];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![2147482996];
    let expected: Vec<i32> = vec![2147482997];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![2147483077, 2147481491, 2147480496, 2147482499, 2147482928];
    let expected: Vec<i32> = vec![2147483078, 2147481492, 2147480497, 2147482500, 2147482929];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![2147480077, 2147479938, 2147479836, 2147479873, 2147482746, 2147483062, 2147479830, 2147482923, 2147479123, 2147480569, 2147480107, 2147480167, 2147481495, 2147480741, 2147480921, 2147483269, 2147481495, 2147479371, 2147481356, 2147483479, 2147483282, 2147482639, 2147482095, 2147482355, 2147479452, 2147481216, 2147482081, 2147482421, 2147479271];
    let expected: Vec<i32> = vec![2147480078, 2147479939, 2147479837, 2147479874, 2147482747, 2147483063, 2147479831, 2147482924, 2147479124, 2147480570, 2147480108, 2147480168, 2147481496, 2147480742, 2147480922, 2147483270, 2147481496, 2147479372, 2147481357, 2147483480, 2147483283, 2147482640, 2147482096, 2147482356, 2147479453, 2147481217, 2147482082, 2147482422, 2147479272];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![2147481412];
    let expected: Vec<i32> = vec![2147481413];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![2147481083, 2147480988, 2147480782];
    let expected: Vec<i32> = vec![2147481084, 2147480989, 2147480783];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![2147479941];
    let expected: Vec<i32> = vec![2147479942];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![2147479219, 2147483130, 2147480741];
    let expected: Vec<i32> = vec![2147479220, 2147483131, 2147480742];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![2147481788, 2147480453];
    let expected: Vec<i32> = vec![2147481789, 2147480454];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![2147483035];
    let expected: Vec<i32> = vec![2147483036];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![2147480080, 2147480816, 2147478810, 2147480638, 2147479892];
    let expected: Vec<i32> = vec![2147480081, 2147480817, 2147478811, 2147480639, 2147479893];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![2147482147, 2147479411, 2147481198, 2147480270];
    let expected: Vec<i32> = vec![2147482148, 2147479412, 2147481199, 2147480271];
    assert_eq!(incr_list(arg_0), expected);
}