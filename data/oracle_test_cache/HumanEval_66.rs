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
    let arg_0: Vec<char> = vec!['?', '\u{6310b}', '\u{db3ef}', 'à', '\'', 't', '\u{86351}', '-', '<', 'b', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['Ѩ', '\n', '\u{f455d}'];
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
    let arg_0: Vec<char> = vec!['뵐', 'E', '𝁽', ':', '\u{202e}', '\r', '/'];
    let expected: u128 = 69;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['?', '\n', ':', '%', '\u{9227e}', '🕴', '\u{7f}'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['\u{d32b2}', '\''];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec![')', '{', 'ª', '\u{4c6d0}', '\u{b}', 'g', '\u{9e}', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$', '\u{56e89}', '\r', '%', 'm', '?', 'E', '\n', '\r', 'r', 'l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}', 'j'];
    let expected: u128 = 142;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['þ'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['{', 'Ⱥ', '\u{f0bbb}', '㼛'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['h', 'E', '𑩄'];
    let expected: u128 = 69;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['.', '/', '�'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['*'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['\u{d571e}', '/'];
    let expected: u128 = 0;
    assert_eq!(digit_sum(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['A', '¥', ':'];
    let expected: u128 = 65;
    assert_eq!(digit_sum(&arg_0), expected);
}