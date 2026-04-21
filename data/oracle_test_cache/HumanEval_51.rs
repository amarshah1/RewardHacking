#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['?', '\u{6310b}', '\u{db3ef}', 'à', '\'', 't', '\u{86351}', '-', '<', 'b', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let expected: Vec<char> = vec!['?', '\u{6310b}', '\u{db3ef}', 'à', '\'', 't', '\u{86351}', '-', '<', 'b', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec!['Ѩ', '\n', '\u{f455d}'];
    let expected: Vec<char> = vec!['Ѩ', '\n', '\u{f455d}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<char> = vec![];
    let expected: Vec<char> = vec![];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<char> = vec!['뵐', 'E', '𝁽', ':', '\u{202e}', '\r', '/'];
    let expected: Vec<char> = vec!['뵐', '𝁽', ':', '\u{202e}', '\r', '/'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['?', '\n', ':', '%', '\u{9227e}', '🕴', '\u{7f}'];
    let expected: Vec<char> = vec!['?', '\n', ':', '%', '\u{9227e}', '🕴', '\u{7f}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec!['\u{d32b2}', '\''];
    let expected: Vec<char> = vec!['\u{d32b2}', '\''];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec![')', '{', 'ª', '\u{4c6d0}', '\u{b}', 'g', '\u{9e}', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$', '\u{56e89}', '\r', '%', 'm', '?', 'E', '\n', '\r', 'r', 'l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}', 'j'];
    let expected: Vec<char> = vec![')', '{', 'ª', '\u{4c6d0}', '\u{b}', 'g', '\u{9e}', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$', '\u{56e89}', '\r', '%', 'm', '?', '\n', '\r', 'r', 'l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', '\u{52f69}', 'j'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec!['þ'];
    let expected: Vec<char> = vec!['þ'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['{', 'Ⱥ', '\u{f0bbb}', '㼛'];
    let expected: Vec<char> = vec!['{', 'Ⱥ', '\u{f0bbb}', '㼛'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec!['h', 'E', '𑩄'];
    let expected: Vec<char> = vec!['h', '𑩄'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec!['.', '/', '�'];
    let expected: Vec<char> = vec!['.', '/', '�'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['*'];
    let expected: Vec<char> = vec!['*'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec!['\u{d571e}', '/'];
    let expected: Vec<char> = vec!['\u{d571e}', '/'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['A', '¥', ':'];
    let expected: Vec<char> = vec!['¥', ':'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['/', '\u{79d17}', '\u{b}', '\u{d35be}', 'Ѩ', '9', 'f'];
    let expected: Vec<char> = vec!['/', '\u{79d17}', '\u{b}', '\u{d35be}', 'Ѩ', '9', 'f'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['\u{1e3a8}', '\u{d75d9}', 'F', 'Ѩ', 'd', '🕴', '\u{5f148}'];
    let expected: Vec<char> = vec!['\u{1e3a8}', '\u{d75d9}', 'F', 'Ѩ', 'd', '🕴', '\u{5f148}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec!['+'];
    let expected: Vec<char> = vec!['+'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['P', '\'', '\u{107d85}'];
    let expected: Vec<char> = vec!['P', '\'', '\u{107d85}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['𧜫', 'z', '\r'];
    let expected: Vec<char> = vec!['𧜫', 'z', '\r'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['\u{7fc85}', '\u{a47ab}', '4', ':', '\u{938ec}', '\u{bf008}', '\u{ea651}', '9', '%', '<', '\u{3d4e0}'];
    let expected: Vec<char> = vec!['\u{7fc85}', '\u{a47ab}', '4', ':', '\u{938ec}', '\u{bf008}', '\u{ea651}', '9', '%', '<', '\u{3d4e0}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['\''];
    let expected: Vec<char> = vec!['\''];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['\u{c8187}', '/', '`', 'Ⱥ', '6'];
    let expected: Vec<char> = vec!['\u{c8187}', '/', '`', 'Ⱥ', '6'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['Q', 'n', '\u{fd7be}'];
    let expected: Vec<char> = vec!['Q', 'n', '\u{fd7be}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['\u{88}', '$'];
    let expected: Vec<char> = vec!['\u{88}', '$'];
    assert_eq!(remove_vowels(&arg_0), expected);
}