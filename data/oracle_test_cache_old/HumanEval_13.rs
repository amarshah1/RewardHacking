#[test]
fn oracle_case_0() {
    let arg_0: u64 = 7645359380336737593;
    let arg_1: u64 = 5281276197874154893;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: u64 = 14729830432180286858;
    let arg_1: u64 = 10530800043416210610;
    let expected: u64 = 2;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: u64 = 12331806457460433707;
    let arg_1: u64 = 7241726879045979711;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: u64 = 3288744496421241381;
    let arg_1: u64 = 883087369427888066;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: u64 = 15107015631591094296;
    let arg_1: u64 = 2832275636194402579;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: u64 = 6655104060375675384;
    let arg_1: u64 = 6662289678587984108;
    let expected: u64 = 4;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: u64 = 6718507327374755954;
    let arg_1: u64 = 4323471465544974749;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: u64 = 14869018854397747355;
    let arg_1: u64 = 14797303565500688909;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: u64 = 15389444961492398658;
    let arg_1: u64 = 4283642506346407788;
    let expected: u64 = 6;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: u64 = 13240277546093716320;
    let arg_1: u64 = 16349052695352238323;
    let expected: u64 = 9;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: u64 = 2597271358337624196;
    let arg_1: u64 = 9101796130848296340;
    let expected: u64 = 36;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: u64 = 16331165287349020615;
    let arg_1: u64 = 2566424386603221482;
    let expected: u64 = 13;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: u64 = 9095930907181820903;
    let arg_1: u64 = 11584633401276733616;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: u64 = 12969442648409343119;
    let arg_1: u64 = 9419086667841942769;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: u64 = 13030060885165077069;
    let arg_1: u64 = 10960579899604047341;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: u64 = 14458755064064953162;
    let arg_1: u64 = 3187333458390996998;
    let expected: u64 = 2;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: u64 = 4867362222220893183;
    let arg_1: u64 = 10357811618216432783;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: u64 = 792014529457528981;
    let arg_1: u64 = 11387648338208413591;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: u64 = 4611817604117638626;
    let arg_1: u64 = 17438863938259575802;
    let expected: u64 = 2;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: u64 = 17615354116363844125;
    let arg_1: u64 = 7301919278693916409;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: u64 = 4335155522522048267;
    let arg_1: u64 = 15078499236703217645;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: u64 = 11933682139714707284;
    let arg_1: u64 = 6539340137592302745;
    let expected: u64 = 1;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: u64 = 16570559639569854240;
    let arg_1: u64 = 12831598957418905344;
    let expected: u64 = 96;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: u64 = 11867398783937480314;
    let arg_1: u64 = 330371668481415978;
    let expected: u64 = 2;
    assert_eq!(compute_gcd(arg_0, arg_1), expected);
}