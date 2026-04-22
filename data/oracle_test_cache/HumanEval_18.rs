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
    let arg_0: Vec<char> = vec!['v', '~', 'T', '+', '%', 'F', 'T', '+', '%', 'T', '+', '%', 'T', '+', '%', 'T', '+', '%', 'T', '+', '%', '(', '=', 'c', 'n', 'T', '+', '%', '#', 'w', '&'];
    let arg_1: Vec<char> = vec!['T', '+', '%'];
    let expected: Option<u32> = Some(7);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec!['P', '"', 'a', '"', 'a', 'H', '%', 'e', '"', 'a', '"', 'a', '3', 'b', 'L', 'N', 'C', 'y', '`', '6', '"', 'a', '`'];
    let arg_1: Vec<char> = vec!['"', 'a'];
    let expected: Option<u32> = Some(5);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec!['G', 'z', ' ', 'G', 'z', ' ', 'v', 'P', 'G', 'z', ' ', 'G', 'z', ' ', 'J', '7', 'G', 'z', ' ', 'G', 'z', ' ', 'u', 'G', 'z', ' ', 'W', 'h', 'C', 'R', '0', 'G', 'z', ' ', 'G', 'z', ' '];
    let arg_1: Vec<char> = vec!['G', 'z', ' '];
    let expected: Option<u32> = Some(9);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['6', 'g', 'n', '7', '{', 'O', 'A', 'y', 'g', '%', '\\', '9', ')', '@', '#', 'J', '>', '9', ')', '@', '9', ')', '@', '6', '3', '9', ')', '@', '9', ')', '@', '9', ')', '@', 'P', '9', ')', '@', '5', '<'];
    let arg_1: Vec<char> = vec!['9', ')', '@'];
    let expected: Option<u32> = Some(7);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec!['v', '$', '/', '%', 'f', 'u', 'P', ';', 'c', '-', '}', 'p', ';', 'c', ' ', ';', 'c', ';', 'c', ';', 'c'];
    let arg_1: Vec<char> = vec![';', 'c'];
    let expected: Option<u32> = Some(5);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['7', '[', '3', 'b', 'd', 'm', 'M', '8', 'Z', 's', 'T', 'k', '|'];
    let arg_1: Vec<char> = vec!['7', '['];
    let expected: Option<u32> = Some(1);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['R', 'b', 'K', 'D', '!'];
    let arg_1: Vec<char> = vec!['b', 'K', 'D'];
    let expected: Option<u32> = Some(1);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['[', 's', 'z', 'a', 'z', 'a', '~', ' ', 'H', 'z', 'a', 'o', 'w', 'd', '[', 'z', 'a', ';', 'W', 'z', 'a', 'G', 'M', '^', '-', 'i', 'Z', '6', 'C', 'X', 'z', 'a', 'g', '^', 'z', 'a', 'z', 'a', 's', 'y', 'z', 'a', 'q', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a', 'z', 'a'];
    let arg_1: Vec<char> = vec!['z', 'a'];
    let expected: Option<u32> = Some(38);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec!['N', '#', 'g', 'a', 'z', 'i', '#', 'B', 'W', '}', 't', 'N', 'P', 'k', '0', 'E', '>', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z', 'g', 'a', 'z'];
    let arg_1: Vec<char> = vec!['g', 'a', 'z'];
    let expected: Option<u32> = Some(29);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['?', '\u{6310b}', '\u{db3ef}', 'à', '\'', 't', '\u{86351}', '-', '<', 'b', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let arg_1: Vec<char> = vec!['Ѩ', '\n', '\u{f455d}', ']'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec!['뵐', 'E', '𝁽', ':'];
    let arg_1: Vec<char> = vec!['\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['b'];
    let arg_1: Vec<char> = vec!['?', '\n', ':', '%', '\u{9227e}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['{', '?', '\u{db202}', '\u{101970}'];
    let arg_1: Vec<char> = vec![')', '{', 'ª', '\u{4c6d0}', '\u{b}', 'g', '\u{9e}', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$', '\u{56e89}', '\r', '%', 'm', '?', 'E', '\n', '\r', 'r', 'l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['Ⱥ', 'þ', '\\', '{'];
    let arg_1: Vec<char> = vec!['{'];
    let expected: Option<u32> = Some(1);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['{', '/', '\n', 'h', 'E', '𑩄', '`'];
    let arg_1: Vec<char> = vec!['.', '/'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}