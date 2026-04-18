#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['V', '\u{b}', '𰟣'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec!['.'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<char> = vec![];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<char> = vec!['𦧠'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['\u{33a6a}', '.', 'L', 'Ѩ', '=', '\u{b}'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec!['¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec!['`', '\u{85}', '?', '\'', '8', '뵐'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec!['\u{d3106}', '𗰛'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['\r'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec!['/', '\u{bb313}', '?', '\n'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec!['`'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['\u{9227e}'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec!['Ⱥ', '{'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['\'', 'õ', '\r', '\u{eb895}'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['\u{109c7c}', '\u{b}'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['.', '\u{1fbfe}'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec![':', '\r', '\u{cd706}', ':', '🕴', '%'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['^', '<', '/', '鷫', '3', '?'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['*', '\u{feff}', '$', '\u{56e89}'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}', 'j', '\t', '%', '\u{202e}', '\u{202e}', '0', '{', 'Ⱥ', '\u{f0bbb}', '㼛'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['?', '\u{feff}', '\u{90658}', '\u{65c67}'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['-', '/'];
    let expected: bool = false;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['?', '\u{5bb32}', '\u{d571e}', '/'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['&', 'c', ')', '\u{586b9}'];
    let expected: bool = true;
    assert_eq!(is_happy(&arg_0), expected);
}