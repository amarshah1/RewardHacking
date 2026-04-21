#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![174u8, 150u8, 212u8, 139u8, 72u8, 49u8, 133u8, 142u8, 224u8, 18u8, 192u8, 6u8, 185u8, 136u8, 0u8, 117u8, 10u8, 87u8];
    let arg_1: Vec<u8> = vec![212u8, 72u8, 133u8, 185u8, 224u8, 139u8, 6u8, 174u8, 10u8, 136u8, 87u8, 192u8, 18u8, 142u8, 0u8, 117u8, 150u8, 49u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![248, 225, 236, 42, 114, 209, 157, 100, 155, 222, 13, 49, 66, 167, 108, 41, 96, 71, 243];
    let arg_1: Vec<u8> = vec![241, 107, 77, 86, 237, 88, 74, 31, 6, 228, 255, 39, 143, 201];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![29u8, 229u8, 117u8, 237u8, 81u8, 254u8, 177u8, 156u8, 163u8, 140u8, 22u8, 49u8, 163u8];
    let arg_1: Vec<u8> = vec![177u8, 29u8, 117u8, 22u8, 163u8, 81u8, 229u8, 140u8, 237u8, 49u8, 254u8, 156u8, 156u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![91, 29, 61, 249, 229, 11];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![103u8, 62u8, 78u8, 91u8, 125u8, 140u8, 65u8, 97u8, 103u8, 170u8, 162u8, 163u8, 110u8, 91u8, 218u8];
    let arg_1: Vec<u8> = vec![110u8, 91u8, 218u8, 78u8, 163u8, 103u8, 125u8, 170u8, 62u8, 140u8, 65u8, 162u8, 97u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![0, 68, 122, 81, 42, 72, 254];
    let arg_1: Vec<u8> = vec![130];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![196u8, 253u8, 81u8, 109u8, 58u8, 209u8, 228u8, 28u8, 199u8, 121u8];
    let arg_1: Vec<u8> = vec![81u8, 199u8, 209u8, 109u8, 58u8, 228u8, 253u8, 28u8, 196u8, 121u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![203];
    let arg_1: Vec<u8> = vec![208];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![87u8, 116u8, 198u8, 186u8, 6u8, 58u8, 85u8, 141u8, 58u8, 250u8];
    let arg_1: Vec<u8> = vec![85u8, 116u8, 87u8, 186u8, 116u8, 198u8, 58u8, 141u8, 6u8, 250u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![20, 119, 199, 61, 215, 23, 61, 108];
    let arg_1: Vec<u8> = vec![76, 225, 99, 208, 128];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![201u8, 16u8, 245u8, 216u8, 158u8, 74u8, 96u8, 176u8, 199u8, 23u8];
    let arg_1: Vec<u8> = vec![16u8, 74u8, 158u8, 201u8, 96u8, 23u8, 176u8, 245u8, 199u8, 176u8, 158u8, 199u8, 23u8, 176u8, 158u8, 216u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![250, 99];
    let arg_1: Vec<u8> = vec![151, 122, 113, 144];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![152u8, 47u8, 249u8, 54u8, 80u8, 39u8, 3u8, 18u8, 181u8, 18u8];
    let arg_1: Vec<u8> = vec![18u8, 181u8, 3u8, 80u8, 3u8, 249u8, 54u8, 152u8, 39u8, 47u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![152];
    let arg_1: Vec<u8> = vec![41];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![19u8, 111u8, 96u8, 240u8, 61u8, 249u8, 199u8, 78u8, 216u8, 248u8];
    let arg_1: Vec<u8> = vec![240u8, 78u8, 248u8, 199u8, 96u8, 216u8, 61u8, 111u8, 19u8, 249u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![71, 7, 18, 63, 217];
    let arg_1: Vec<u8> = vec![73, 61, 126, 240, 106, 231, 209, 6, 71, 53, 3, 165, 151, 177, 142, 95, 191, 82, 179, 36, 233, 71, 174, 22, 35, 100, 151, 252, 182];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![123];
    let arg_1: Vec<u8> = vec![14, 220, 204];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![243];
    let arg_1: Vec<u8> = vec![234, 200, 140];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![117, 172];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![39];
    let arg_1: Vec<u8> = vec![138, 62, 131, 206, 255];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![11, 79, 192, 96];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![221, 168, 40, 35, 111, 200, 45, 89, 181, 161];
    let arg_1: Vec<u8> = vec![182, 135, 156, 82];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![76, 160, 248, 233, 118, 89];
    let arg_1: Vec<u8> = vec![45, 53, 210];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}