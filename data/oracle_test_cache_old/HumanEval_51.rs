#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['V', '\u{b}', '𰟣'];
    let expected: Vec<char> = vec!['V', '\u{b}', '𰟣'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec!['.'];
    let expected: Vec<char> = vec!['.'];
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
    let arg_0: Vec<char> = vec!['𦧠'];
    let expected: Vec<char> = vec!['𦧠'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['\u{33a6a}', '.', 'L', 'Ѩ', '=', '\u{b}'];
    let expected: Vec<char> = vec!['\u{33a6a}', '.', 'L', 'Ѩ', '=', '\u{b}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec!['¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let expected: Vec<char> = vec!['¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec!['`', '\u{85}', '?', '\'', '8', '뵐'];
    let expected: Vec<char> = vec!['`', '\u{85}', '?', '\'', '8', '뵐'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec!['\u{d3106}', '𗰛'];
    let expected: Vec<char> = vec!['\u{d3106}', '𗰛'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['\r'];
    let expected: Vec<char> = vec!['\r'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec!['/', '\u{bb313}', '?', '\n'];
    let expected: Vec<char> = vec!['/', '\u{bb313}', '?', '\n'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec!['`'];
    let expected: Vec<char> = vec!['`'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['\u{9227e}'];
    let expected: Vec<char> = vec!['\u{9227e}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec!['Ⱥ', '{'];
    let expected: Vec<char> = vec!['Ⱥ', '{'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['\'', 'õ', '\r', '\u{eb895}'];
    let expected: Vec<char> = vec!['\'', 'õ', '\r', '\u{eb895}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['\u{109c7c}', '\u{b}'];
    let expected: Vec<char> = vec!['\u{109c7c}', '\u{b}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['.', '\u{1fbfe}'];
    let expected: Vec<char> = vec!['.', '\u{1fbfe}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec![':', '\r', '\u{cd706}', ':', '🕴', '%'];
    let expected: Vec<char> = vec![':', '\r', '\u{cd706}', ':', '🕴', '%'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['^', '<', '/', '鷫', '3', '?'];
    let expected: Vec<char> = vec!['^', '<', '/', '鷫', '3', '?'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['*', '\u{feff}', '$', '\u{56e89}'];
    let expected: Vec<char> = vec!['*', '\u{feff}', '$', '\u{56e89}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}', 'j', '\t', '%', '\u{202e}', '\u{202e}', '0', '{', 'Ⱥ', '\u{f0bbb}', '㼛'];
    let expected: Vec<char> = vec!['l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', '\u{52f69}', 'j', '\t', '%', '\u{202e}', '\u{202e}', '0', '{', 'Ⱥ', '\u{f0bbb}', '㼛'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['?', '\u{feff}', '\u{90658}', '\u{65c67}'];
    let expected: Vec<char> = vec!['?', '\u{feff}', '\u{90658}', '\u{65c67}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['-', '/'];
    let expected: Vec<char> = vec!['-', '/'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['?', '\u{5bb32}', '\u{d571e}', '/'];
    let expected: Vec<char> = vec!['?', '\u{5bb32}', '\u{d571e}', '/'];
    assert_eq!(remove_vowels(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['&', 'c', ')', '\u{586b9}'];
    let expected: Vec<char> = vec!['&', 'c', ')', '\u{586b9}'];
    assert_eq!(remove_vowels(&arg_0), expected);
}