#[test]
fn oracle_case_0() {
    let arg_0: Vec<char> = vec!['?', '\u{6310b}', '\u{db3ef}', 'à', '\'', 't', '\u{86351}', '-', '<', 'b', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z'];
    let arg_1: Vec<char> = vec!['Ѩ', '\n', '\u{f455d}', ']'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<char> = vec!['뵐', 'E', '𝁽', ':'];
    let arg_1: Vec<char> = vec!['\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<char> = vec!['b'];
    let arg_1: Vec<char> = vec!['?', '\n', ':', '%', '\u{9227e}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<char> = vec!['{', '?', '\u{db202}', '\u{101970}'];
    let arg_1: Vec<char> = vec![')', '{', 'ª', '\u{4c6d0}', '\u{b}', 'g', '\u{9e}', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$', '\u{56e89}', '\r', '%', 'm', '?', 'E', '\n', '\r', 'r', 'l', ']', '.', 'Ѩ', '\u{e3d66}', '\t', 'I', '\u{52f69}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<char> = vec!['Ⱥ', 'þ', '\\', '{'];
    let arg_1: Vec<char> = vec!['{'];
    let expected: Option<u32> = Some(1);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<char> = vec!['{', '/', '\n', 'h', 'E', '𑩄', '`'];
    let arg_1: Vec<char> = vec!['.', '/'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\r'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['\u{d571e}', '/', '\u{d69bb}', 'A', '¥', ':'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<char> = vec!['/', '\u{79d17}', '\u{b}', '\u{d35be}', 'Ѩ', '9', 'f'];
    let arg_1: Vec<char> = vec!['\u{1e3a8}', '\u{d75d9}', 'F', 'Ѩ', 'd', '🕴', '\u{5f148}', '.'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<char> = vec!['4', ':'];
    let arg_1: Vec<char> = vec!['\u{e8de6}', '𧜫', 'z', '\r', 'g', '\t', '\'', '\u{b8371}', '\u{7fc85}', '\u{a47ab}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<char> = vec![':', '\u{938ec}', '\u{bf008}'];
    let arg_1: Vec<char> = vec!['k', '\u{ede30}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<char> = vec!['<'];
    let arg_1: Vec<char> = vec!['𤊲'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec!['['];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<char> = vec!['\u{c8187}', '/'];
    let arg_1: Vec<char> = vec!['P'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<char> = vec!['Q', 'n', '\u{fd7be}', '=', '?', '\u{202e}'];
    let arg_1: Vec<char> = vec!['$'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<char> = vec!['\u{89ca1}', '\0', '$', '*', '\0'];
    let arg_1: Vec<char> = vec!['\u{f2311}', '-', '焗', '.', ' ', 'Ⱥ', 'C', 'o'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<char> = vec!['%', '\u{7fdb7}'];
    let arg_1: Vec<char> = vec!['\u{d58f6}', '$', '\u{58725}', '[', '\u{10d810}', '`'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<char> = vec!['\u{6f275}', '\u{8cefd}', '¥'];
    let arg_1: Vec<char> = vec![':', 'U', 'L', '\'', '\u{700b7}', '\u{f8317}', '`', '\u{aa99d}', '\u{aa8e5}', '\n', 'Ѩ', 'N', 'p'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<char> = vec![];
    let arg_1: Vec<char> = vec![')', '𖪔', '⣒', '\\', 'j', '¥', '=', '\u{36064}', '\u{10a811}'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<char> = vec!['\r', '\u{16b93}', ':', '\u{67661}', '\u{1a31a}', '<', '#', 'I', '🕴', '\u{418eb}', '?', 'Ⱥ', '\t', '\u{aa1bd}', '\u{8b812}', 'b'];
    let arg_1: Vec<char> = vec!['\n'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<char> = vec!['\u{d3a91}', '\0'];
    let arg_1: Vec<char> = vec!['催', '>', '\n'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<char> = vec!['\u{334aa}'];
    let arg_1: Vec<char> = vec!['*', 'L', '¥'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<char> = vec!['/', '𢨑'];
    let arg_1: Vec<char> = vec!['?', '\r', ':', '\u{8f912}', '2'];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<char> = vec!['/', '\0', '¥', '\u{b}', '$', '\u{1b}', '\u{76e1d}'];
    let arg_1: Vec<char> = vec!['\u{6e906}', '\u{66690}', '/', '\''];
    let expected: Option<u32> = Some(0);
    assert_eq!(how_many_times_impl(arg_0, arg_1), expected);
}