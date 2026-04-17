#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['K', 'S', '<', '}', 'S', 'd', 'L', 'k', 'i', 'c', 'B', 'I', 'b', 'z', '9', 'k', 'A', 'd', 'K', 't', 'e', '\\', 'I', 'b', '?', 'r', '+', 'e', 'M', ':', 'I', 'b', 'I', 'b', ']', 'S', 'I', 'b', 'I', 'b', 'i', 'V', '2', '4', 'n', '~', 's', 'V', ' ', 'a', 'g', 'g', 'Z', 'I', 'b', 'm', 'I', 'b', 'I', 'b', 'I', 'b', 'I', 'b', 'I', 'b', 'I', 'b', 'I', 'b', 'D', ']'];
    let arg_1: Vec<char> = vec!['I', 'b'];
    let expected: Option<u32> = Some(14);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec!['4', 'i', 'y', '=', 'M', 'e', '~', 'k', 'O'];
    let arg_1: Vec<char> = vec!['M', 'e', '~'];
    let expected: Option<u32> = Some(1);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<char> = vec!['B', 'W', '%', 'h', '@', 'S', 'm', 'm', '-', 'I', 'w', 'A', 'm', ';'];
    let arg_1: Vec<char> = vec!['m'];
    let expected: Option<u32> = Some(3);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<char> = vec!['<', '[', 'H', 'i', '!', 'C', '[', 'H', 'a', 'r', '8', 't', ':', '"'];
    let arg_1: Vec<char> = vec!['[', 'H'];
    let expected: Option<u32> = Some(2);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['c', 'm', 'z', 'L', '`', '\'', 'n', 'Z', 'c', 'V', 'v', '5', 'c', 'V', 'v', 'c', 'V', 'v', 'f', '6', '#', 'c', 'V', 'v', 'c', 'V', 'v', '8', 't'];
    let arg_1: Vec<char> = vec!['c', 'V', 'v'];
    let expected: Option<u32> = Some(5);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec!['+', '_', 't', 's', '(', 't', 's', '(', 'D', ':', '>', '7', 't', 's', '(', 'k', '$', '0', 't', 's', '(', 't', 's', '(', '>', 't', 's', '(', 'C', '[', '3', 'v', ':', '^', 'N', '=', '[', 'D', '8', ']', '2', 'A', 'G', 't', 's', '(', 't', 's', '(', 't', 's', '(', 't', 's', '(', 't', 's', '(', 't', 's', '(', 'y', 'Q', 't', 's', '(', 'b', 'l', 't', 's', '(', 'R', 'V', 't', 's', '(', '<', '(', 'j', 'a', 'V', 'K', 'A', 'V', 'M', 'v', 't', 's', '(', 't', 's', '(', ')', 'N', '3', 'K'];
    let arg_1: Vec<char> = vec!['t', 's', '('];
    let expected: Option<u32> = Some(17);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec!['\\', 'F', 'I', '{', '8', 'Q', '/', 'U', 'H', 'H', 'K', 'o', '{', '8', 'Q', 'F'];
    let arg_1: Vec<char> = vec!['{', '8', 'Q'];
    let expected: Option<u32> = Some(2);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec![' ', 'f', 'o', 'v', 'h', '1'];
    let arg_1: Vec<char> = vec!['o', 'v'];
    let expected: Option<u32> = Some(1);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['V', '\u{b}', '𰟣'];
    let arg_1: Vec<char> = vec!['.', '?'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['l', 'Ⱥ', '\'', 't', '\u{86351}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec!['\u{b}', '<', '\u{7f}', 'Ⱥ', '🕴', '\u{b}'];
    let arg_1: Vec<char> = vec!['\u{98ba3}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['7'];
    let arg_1: Vec<char> = vec!['`', '\u{85}', '?', '\'', '8', '뵐', 'E', '𝁽', ':', '\u{202e}', '\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec!['/', '\u{bb313}', '?'];
    let arg_1: Vec<char> = vec![':'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\u{9227e}', '🕴', '\u{7f}', '�'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['\'', 'õ', '\r', '\u{eb895}', ')'];
    let arg_1: Vec<char> = vec!['"', '*', '\u{1fbfe}', '\u{da632}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec![':', '\r'];
    let arg_1: Vec<char> = vec!['{'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec!['^', '<', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$'];
    let arg_1: Vec<char> = vec![':', '\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}', 'j', '\t', '%', '\u{202e}', '\u{202e}', '0', '{', 'Ⱥ', '\u{f0bbb}', '㼛', '/'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['h'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\u{51761}', '-', '/', '�', '\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['M', 'g', '\u{e5d02}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['&'];
    let arg_1: Vec<char> = vec!['\u{1b}', ')'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['\u{103844}', '$', '\u{1070f9}', '"', '\r'];
    let arg_1: Vec<char> = vec!['\t', 'B', '¥', '"'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['\u{da6f5}', 'd', '🕴', '\u{5f148}', '.', '+', '\n', 'P'];
    let arg_1: Vec<char> = vec!['\u{cfc}', '*', '\u{e8de6}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}