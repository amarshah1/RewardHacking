#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![154980374i32, -455553371i32, 912819079i32, -912819079i32, -154980374i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![905163256, -755173140, 755501170, -463961699, 519479451, 1061825037, 472773186, 797615980, -159440032, -258154253, -620582780, -851653228, 950069191, 308445674, 1158119, 272210096, -381844337, -1051314959, -33951155, 405771757, 1073269578];
    let arg_1: i32 = 1491;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![525595244i32, 790566641i32, -783354082i32, 618197859i32, 783354082i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![59529255, -809609527];
    let arg_1: i32 = 3574;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![-863633138i32, -810843963i32, -272217699i32, -809992697i32, -361953552i32, 198254772i32, 108574106i32, -742836365i32, -631185357i32, -811580274i32, 811580274i32, 742836365i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![839076443, 880168765, 626368741, -64384774, 289511210];
    let arg_1: i32 = 3925;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![298680957i32, -858575790i32, 77855044i32, 507030015i32, -274843589i32, -771586090i32, -587113987i32, -507030015i32, 771586090i32, -77855044i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![490340096, 478086778, -139453654, 413998590];
    let arg_1: i32 = 3543;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-958047583i32, 708344398i32, 958047583i32, -714475957i32, 714475957i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-886909237, 517632115, 615546211];
    let arg_1: i32 = -2794;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![-444239827i32, -1062920794i32, -619523059i32, -539792229i32, 444239827i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![676369172, -612548665, 167378647, 1073644861, -220207375, 454553772];
    let arg_1: i32 = -3605;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![-1065630154i32, -38594623i32, 885770418i32, 38594623i32, 1065630154i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-830354333, -165019776, 292844977];
    let arg_1: i32 = 3746;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![71187092i32, 854280180i32, 275717697i32, -267213142i32, 267213142i32, -214383805i32, -309965750i32, 120537331i32, -120537331i32, 409739745i32, -267348217i32, -409739745i32, -71187092i32, 214383805i32, -143783126i32, -854280180i32, -275717697i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-913377385, 951278961, -82952311, 676126360, 654425755, -238926234];
    let arg_1: i32 = -3846;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![268909610i32, -271266299i32, 336645977i32, 271266299i32, 541749110i32, -336645977i32, -1010445859i32, -268909610i32, -573831507i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-360595129, 835560722, -578995239];
    let arg_1: i32 = -2576;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![995327431i32, -462923171i32, 462923171i32, 385049862i32, -995327431i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![285143881, -684192130, -908908182, 22723281, 371396935, 973298179, 682972311, 1050195598, 381414847, -274588749, 843446761, 300583854, 1063461667, -896042857, -524773706, -823856249, 127496800, 158645567, -591510770, 536218316, -879594015, 200781946, -252665855, 541038314, 641719692, 719308526, 683923829, -717343348];
    let arg_1: i32 = 1725;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![-155914661i32, -412055994i32, -967143061i32, -609162891i32, -694894125i32, -80020158i32, -266352850i32, -225732143i32, -814706226i32, 26774380i32, 225732143i32, -26774380i32, 266352850i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![-35218649, 289086006];
    let arg_1: i32 = 4207;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![394340480i32, -895174965i32, 895174965i32, -394340480i32, 828206332i32];
    let arg_1: i32 = 0i32;
    let expected: bool = true;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![-770765430, 963090563];
    let arg_1: i32 = -3754;
    let expected: bool = false;
    assert_eq!(pairs_sum_to_zero(&arg_0, arg_1), expected);
}