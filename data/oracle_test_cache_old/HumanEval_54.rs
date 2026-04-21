#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![174u8, 150u8, 212u8, 139u8, 72u8, 49u8, 133u8, 142u8, 224u8, 18u8, 192u8, 6u8, 185u8, 136u8, 0u8, 117u8, 10u8, 87u8];
    let arg_1: Vec<u8> = vec![212u8, 72u8, 133u8, 185u8, 224u8, 139u8, 6u8, 174u8, 10u8, 136u8, 87u8, 192u8, 18u8, 142u8, 0u8, 117u8, 150u8, 49u8];
    let expected: bool = true;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![151u8, 151u8, 37u8, 84u8, 239u8, 134u8, 16u8, 209u8, 198u8, 31u8, 228u8];
    let arg_1: Vec<u8> = vec![198u8, 84u8, 151u8, 37u8, 180u8, 16u8, 230u8, 209u8, 228u8, 31u8, 134u8, 239u8];
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
    let arg_0: Vec<u8> = vec![250u8, 50u8, 207u8, 189u8, 74u8, 100u8, 113u8, 188u8, 113u8, 122u8];
    let arg_1: Vec<u8> = vec![5u8, 207u8, 113u8, 122u8, 68u8, 50u8, 74u8, 189u8, 188u8, 250u8, 100u8];
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
    let arg_0: Vec<u8> = vec![190u8, 205u8, 47u8, 146u8, 11u8, 205u8, 152u8, 237u8, 13u8, 85u8];
    let arg_1: Vec<u8> = vec![146u8, 68u8, 190u8, 11u8, 47u8, 13u8, 85u8, 237u8, 152u8, 205u8, 232u8];
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
    let arg_0: Vec<u8> = vec![123u8, 235u8, 32u8, 151u8, 150u8, 149u8, 56u8, 83u8, 234u8, 131u8];
    let arg_1: Vec<u8> = vec![235u8, 54u8, 83u8, 131u8, 234u8, 149u8, 32u8, 150u8, 123u8, 151u8, 56u8];
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
    let arg_0: Vec<u8> = vec![61u8, 165u8, 154u8, 116u8, 8u8, 86u8, 6u8, 61u8, 125u8, 165u8, 48u8, 98u8, 217u8];
    let arg_1: Vec<u8> = vec![165u8, 6u8, 116u8, 98u8, 19u8, 86u8, 114u8, 217u8, 48u8, 125u8, 154u8, 217u8, 125u8, 61u8, 8u8, 6u8];
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
    let arg_0: Vec<u8> = vec![44u8, 139u8, 101u8, 8u8, 82u8, 82u8, 180u8, 120u8, 216u8, 159u8];
    let arg_1: Vec<u8> = vec![216u8, 120u8, 192u8, 64u8, 44u8, 139u8, 101u8, 82u8, 159u8, 180u8, 8u8];
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
    let arg_0: Vec<u8> = vec![245u8, 165u8, 183u8, 188u8, 183u8, 245u8, 168u8, 91u8, 11u8, 198u8];
    let arg_1: Vec<u8> = vec![183u8, 165u8, 245u8, 44u8, 228u8, 188u8, 11u8, 168u8, 91u8, 198u8];
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
    let arg_0: Vec<u8> = vec![15u8, 166u8, 252u8, 35u8, 200u8, 128u8, 244u8, 244u8, 200u8, 244u8];
    let arg_1: Vec<u8> = vec![1u8, 15u8, 166u8, 244u8, 35u8, 128u8, 166u8, 109u8, 200u8, 252u8];
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
    let arg_0: Vec<u8> = vec![1, 1, 7];
    let arg_1: Vec<u8> = vec![];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![1, 4, 6];
    let arg_1: Vec<u8> = vec![3];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![7];
    let arg_1: Vec<u8> = vec![3, 5, 1, 12, 3, 1, 0, 3];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![3, 3, 2, 2, 6];
    let arg_1: Vec<u8> = vec![3, 2];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![];
    let arg_1: Vec<u8> = vec![3, 2];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![2, 5, 2];
    let arg_1: Vec<u8> = vec![5, 4, 0, 2, 0];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![1, 6];
    let arg_1: Vec<u8> = vec![15, 5, 1, 1];
    let expected: bool = false;
    assert_eq!(same_chars(&arg_0, &arg_1), expected);
}