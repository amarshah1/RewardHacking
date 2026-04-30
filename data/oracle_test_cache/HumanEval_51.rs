#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['E', 'i', 'A', 'e', 'I', 'A', 'U', 'o', 'I', 'O', 'O', 'I', 'E', 'I', 'e'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec!['I', 'a', 'O'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<char> = vec!['A', 'j', 'T', 'O', 'a'];
    let expected: Vec<char> = vec!['j', 'T'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<char> = vec!['F', '[', '~', '!', 'u', 't', '4', 'E', 'E', 'E', 'i', 'a', 'E', 'u'];
    let expected: Vec<char> = vec!['F', '[', '~', '!', 't', '4'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['e', 'U', 'U', 'a', 'u', 'I', 'i', 'o', 'E', 'O', 'e'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec!['O', 'O', 'i'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec!['U', 'o', 'S', ')', 'e', 'I', 'O'];
    let expected: Vec<char> = vec!['S', ')'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec!['I', 'e', ']', 'e', 'O', 'I', 'Q'];
    let expected: Vec<char> = vec![']', 'Q'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['u', 'i', 'O'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec!['E', 'e', 'O'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec!['u', '`', 'I', 'a'];
    let expected: Vec<char> = vec!['`'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['I', 'e', 'U', 'E'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec!['O', 'i', 'i'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['u', 'I', 'F', 'o', '`'];
    let expected: Vec<char> = vec!['F', '`'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['\'', 'o', 'E', 'e', 'o', 'A'];
    let expected: Vec<char> = vec!['\''];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['a', 'U', 'o', '<', 'I', 'u'];
    let expected: Vec<char> = vec!['<'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec!['O', 'C', 'I', 'a'];
    let expected: Vec<char> = vec!['C'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['u', 'u', 'u'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['E', 'U', 'u', 'u', 'i', 'O'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['o', 'i', 'I', 'u'];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['?', '\u{6310b}', '\u{db3ef}', 'à', '\'', 't', '\u{86351}', '-', '<', 'b', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let expected: Vec<char> = vec!['?', '\u{6310b}', '\u{db3ef}', 'à', '\'', 't', '\u{86351}', '-', '<', 'b', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['Ѩ', '\n', '\u{f455d}'];
    let expected: Vec<char> = vec!['Ѩ', '\n', '\u{f455d}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec![];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['뵐', 'E', '𝁽', ':', '\u{202e}', '\r', '/'];
    let expected: Vec<char> = vec!['뵐', '𝁽', ':', '\u{202e}', '\r', '/'];
    assert_eq!(remove_vowels(&arg_0), expected);
}