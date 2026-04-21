#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['G', 'W', 'O', 'Y', 'E', 'W', 'U', ',', '_', 'Q'];
    let expected: u128 = 648;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec!['A', '.', 'T', '1', 'O', 'C', 'I', 'z', ':', 'M'];
    let expected: u128 = 445;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<char> = vec!['B', '\\', 'U', 'Q', 'K', 's', 'X', 'F', 'z', 'T'];
    let expected: u128 = 549;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<char> = vec!['K', 'J', 'N', '.', '"', 'R', '#', '5', 'W', 'M'];
    let expected: u128 = 473;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['~', '\'', '_', 'N', '+', '6', 'I', ']', 'O', ';'];
    let expected: u128 = 230;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec!['L', 'Y', 'S', '5', 'v', ';', 'D', '2', 'h', '?'];
    let expected: u128 = 316;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec!['H', 'K', 'G', 'V', 'F', 'C', '"', 'z', '*', 'S', '|', '^', 'Y'];
    let expected: u128 = 613;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec!['W', '/', 'F', 'z', 'E', 'Q', '<', 'V', 'g', '4', 'M', 'I', 'R', 'W'];
    let expected: u128 = 712;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['Q', 'T', 'T', 'Y', 'G', 'K', 'P', 'Y', 'W', 'r'];
    let expected: u128 = 740;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec!['G', '!', 'i', 'A', 'I', 'E', 'V', 's', 'C', 'd'];
    let expected: u128 = 431;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec!['V', '\u{b}', '𰟣'];
    let expected: u128 = 86;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['.'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec![];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['𦧠'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['\u{33a6a}', '.', 'L', 'Ѩ', '=', '\u{b}'];
    let expected: u128 = 76;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec!['`', '\u{85}', '?', '\'', '8', '뵐'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['\u{d3106}', '𗰛'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['\r'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['/', '\u{bb313}', '?', '\n'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['`'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['\u{9227e}'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['Ⱥ', '{'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['\'', 'õ', '\r', '\u{eb895}'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}