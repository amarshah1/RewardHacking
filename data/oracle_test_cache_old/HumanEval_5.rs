#[test]
fn oracle_case_0() {
    let arg_0: Vec<u64> = vec![14729830432180286858, 10530800043416210610, 12331806457460433707];
    let arg_1: u64 = 7241726879045979711;
    let expected: Vec<u64> = vec![14729830432180286858, 7241726879045979711, 10530800043416210610, 7241726879045979711, 12331806457460433707];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u64> = vec![15107015631591094296, 2832275636194402579, 6655104060375675384];
    let arg_1: u64 = 6662289678587984108;
    let expected: Vec<u64> = vec![15107015631591094296, 6662289678587984108, 2832275636194402579, 6662289678587984108, 6655104060375675384];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u64> = vec![14869018854397747355, 14797303565500688909, 15389444961492398658];
    let arg_1: u64 = 4283642506346407788;
    let expected: Vec<u64> = vec![14869018854397747355, 4283642506346407788, 14797303565500688909, 4283642506346407788, 15389444961492398658];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 3502921947551820359;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u64> = vec![10177235294094056586];
    let arg_1: u64 = 8692202124828742491;
    let expected: Vec<u64> = vec![10177235294094056586];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u64> = vec![12195049118401618611, 9319694691601266527, 13689238957088788843, 15577834484271420246, 18444715795135843672];
    let arg_1: u64 = 12113794621367636255;
    let expected: Vec<u64> = vec![12195049118401618611, 12113794621367636255, 9319694691601266527, 12113794621367636255, 13689238957088788843, 12113794621367636255, 15577834484271420246, 12113794621367636255, 18444715795135843672];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 4867362222220893183;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u64> = vec![11387648338208413591, 4611817604117638626, 17438863938259575802];
    let arg_1: u64 = 17615354116363844125;
    let expected: Vec<u64> = vec![11387648338208413591, 17615354116363844125, 4611817604117638626, 17615354116363844125, 17438863938259575802];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u64> = vec![5666844007047427245, 10807635390534543157, 6717680698523022386, 6665053097574989636, 4012737137952782417, 15613168459999197000];
    let arg_1: u64 = 4777489568778179747;
    let expected: Vec<u64> = vec![5666844007047427245, 4777489568778179747, 10807635390534543157, 4777489568778179747, 6717680698523022386, 4777489568778179747, 6665053097574989636, 4777489568778179747, 4012737137952782417, 4777489568778179747, 15613168459999197000];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u64> = vec![280841473516295883, 3770724347503341683];
    let arg_1: u64 = 536024418584919395;
    let expected: Vec<u64> = vec![280841473516295883, 536024418584919395, 3770724347503341683];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u64> = vec![3434335401724260885];
    let arg_1: u64 = 1611154708229543960;
    let expected: Vec<u64> = vec![3434335401724260885];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u64> = vec![11204181575914652023];
    let arg_1: u64 = 14553943872757666877;
    let expected: Vec<u64> = vec![11204181575914652023];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u64> = vec![212194756340475633, 153091941835731116];
    let arg_1: u64 = 12695179283796880746;
    let expected: Vec<u64> = vec![212194756340475633, 12695179283796880746, 153091941835731116];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 10268713354534806241;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 11656810062108295040;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u64> = vec![5198026311149810682];
    let arg_1: u64 = 7597699155819524439;
    let expected: Vec<u64> = vec![5198026311149810682];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u64> = vec![14722695418191214487, 13206559139795524977];
    let arg_1: u64 = 15561918916124884873;
    let expected: Vec<u64> = vec![14722695418191214487, 15561918916124884873, 13206559139795524977];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 16645795271474091805;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u64> = vec![4030693722604462151, 12286313769261737552, 17423764034161739783];
    let arg_1: u64 = 11348292442982096191;
    let expected: Vec<u64> = vec![4030693722604462151, 11348292442982096191, 12286313769261737552, 11348292442982096191, 17423764034161739783];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 14514591311607674361;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u64> = vec![3594539726220062357, 18024940915038837143, 12489628199863241280];
    let arg_1: u64 = 1829469989746740719;
    let expected: Vec<u64> = vec![3594539726220062357, 1829469989746740719, 18024940915038837143, 1829469989746740719, 12489628199863241280];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u64> = vec![];
    let arg_1: u64 = 1965683985323430820;
    let expected: Vec<u64> = vec![];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u64> = vec![2153219818725394894];
    let arg_1: u64 = 5394706966840013486;
    let expected: Vec<u64> = vec![2153219818725394894];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u64> = vec![5836369664964711373, 10896475236251476285, 707955102111490288, 13932653806290110439];
    let arg_1: u64 = 6206823712212047110;
    let expected: Vec<u64> = vec![5836369664964711373, 6206823712212047110, 10896475236251476285, 6206823712212047110, 707955102111490288, 6206823712212047110, 13932653806290110439];
    assert_eq!(intersperse(arg_0, arg_1), expected);
}