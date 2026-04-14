#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['V', '\u{b}', '𰟣'];
    let arg_1: Vec<char> = vec!['.', '?'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['l', 'Ⱥ', '\'', 't', '\u{86351}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<char> = vec!['\u{b}', '<', '\u{7f}', 'Ⱥ', '🕴', '\u{b}'];
    let arg_1: Vec<char> = vec!['\u{98ba3}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<char> = vec!['7'];
    let arg_1: Vec<char> = vec!['`', '\u{85}', '?', '\'', '8', '뵐', 'E', '𝁽', ':', '\u{202e}', '\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['/', '\u{bb313}', '?'];
    let arg_1: Vec<char> = vec![':'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\u{9227e}', '🕴', '\u{7f}', '�'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec!['\'', 'õ', '\r', '\u{eb895}', ')'];
    let arg_1: Vec<char> = vec!['"', '*', '\u{1fbfe}', '\u{da632}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec![':', '\r'];
    let arg_1: Vec<char> = vec!['{'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['^', '<', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$'];
    let arg_1: Vec<char> = vec![':', '\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}', 'j', '\t', '%', '\u{202e}', '\u{202e}', '0', '{', 'Ⱥ', '\u{f0bbb}', '㼛', '/'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['h'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\u{51761}', '-', '/', '�', '\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['M', 'g', '\u{e5d02}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['&'];
    let arg_1: Vec<char> = vec!['\u{1b}', ')'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['\u{103844}', '$', '\u{1070f9}', '"', '\r'];
    let arg_1: Vec<char> = vec!['\t', 'B', '¥', '"'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['\u{da6f5}', 'd', '🕴', '\u{5f148}', '.', '+', '\n', 'P'];
    let arg_1: Vec<char> = vec!['\u{cfc}', '*', '\u{e8de6}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec!['z', '\r', 'g'];
    let arg_1: Vec<char> = vec!['ɸ', '.', '𬷒'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['4', ':'];
    let arg_1: Vec<char> = vec!['J', '\u{67d5c}', '&', '\u{15418}', '('];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['\u{b1c21}'];
    let arg_1: Vec<char> = vec!['Ⱥ'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\u{c8187}', '/', '`'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['6'];
    let arg_1: Vec<char> = vec!['\u{feff}', '\u{57bce}', '*', '\u{a0}', 'j'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['g', '\''];
    let arg_1: Vec<char> = vec!['\u{b4a60}', '\0'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec![':', 'q', '叺'];
    let arg_1: Vec<char> = vec!['Ѩ', '𪨹', '$', '&'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['\\'];
    let arg_1: Vec<char> = vec!['E', '\u{7fdb7}', '/', '\u{474f6}', '\u{d58f6}', '$', '\u{58725}', '['];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}