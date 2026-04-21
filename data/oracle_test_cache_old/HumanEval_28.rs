#[test]
fn oracle_case_0() {
    let arg_0: Vec<Vec<char>> = vec![vec!['À'], vec!['.', '?', '\u{6310b}', '\u{db3ef}', 'à'], vec![]];
    let expected: Vec<char> = vec!['À', '.', '?', '\u{6310b}', '\u{db3ef}', 'à'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<Vec<char>> = vec![vec!['L', 'Ѩ', '=']];
    let expected: Vec<char> = vec!['L', 'Ѩ', '='];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{98ba3}', '\\', '7'], vec!['`', '\u{85}', '?', '\'', '8', '뵐', 'E', '𝁽', ':', '\u{202e}'], vec!['/', '\u{bb313}', '?', '\n', ':'], vec!['\u{9227e}'], vec!['Ⱥ', '{'], vec!['\'', 'õ', '\r', '\u{eb895}'], vec!['\u{109c7c}', '\u{b}'], vec!['.', '\u{1fbfe}']];
    let expected: Vec<char> = vec!['\u{98ba3}', '\\', '7', '`', '\u{85}', '?', '\'', '8', '뵐', 'E', '𝁽', ':', '\u{202e}', '/', '\u{bb313}', '?', '\n', ':', '\u{9227e}', 'Ⱥ', '{', '\'', 'õ', '\r', '\u{eb895}', '\u{109c7c}', '\u{b}', '.', '\u{1fbfe}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<Vec<char>> = vec![vec!['{', 'ª', '\u{4c6d0}', '\u{b}'], vec!['^'], vec!['/'], vec!['\\'], vec![], vec!['\'', '\u{e65e3}', '*', '\u{feff}']];
    let expected: Vec<char> = vec!['{', 'ª', '\u{4c6d0}', '\u{b}', '^', '/', '\\', '\'', '\u{e65e3}', '*', '\u{feff}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<Vec<char>> = vec![];
    let expected: Vec<char> = vec![];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\r'], vec![]];
    let expected: Vec<char> = vec!['\r'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\\', '\u{b}'], vec!['K'], vec!['\t', 'I'], vec!['\u{202e}', '\u{202e}', '0', '{', 'Ⱥ', '\u{f0bbb}', '㼛', '/', '\n', 'h', 'E'], vec!['\u{51761}'], vec!['b', '\u{1b}'], vec!['?'], vec![], vec!['\u{c35d6}'], vec!['&', 'c', ')', '\u{586b9}', '$', '_', '\\'], vec!['\r', '\u{7a54b}', '\t', 'B', '¥'], vec!['\u{da6f5}', 'd', '🕴', '\u{5f148}', '.', '+', '\n', 'P', '4', '\u{107d85}'], vec!['\'', '\u{43e67}', '*'], vec!['\n', '\u{b}', '\u{202e}'], vec!['\u{7fc85}', '\u{a47ab}', '4', ':'], vec!['J', '\u{67d5c}', '&', '\u{15418}'], vec!['$'], vec!['\u{b1c21}']];
    let expected: Vec<char> = vec!['\\', '\u{b}', 'K', '\t', 'I', '\u{202e}', '\u{202e}', '0', '{', 'Ⱥ', '\u{f0bbb}', '㼛', '/', '\n', 'h', 'E', '\u{51761}', 'b', '\u{1b}', '?', '\u{c35d6}', '&', 'c', ')', '\u{586b9}', '$', '_', '\\', '\r', '\u{7a54b}', '\t', 'B', '¥', '\u{da6f5}', 'd', '🕴', '\u{5f148}', '.', '+', '\n', 'P', '4', '\u{107d85}', '\'', '\u{43e67}', '*', '\n', '\u{b}', '\u{202e}', '\u{7fc85}', '\u{a47ab}', '4', ':', 'J', '\u{67d5c}', '&', '\u{15418}', '$', '\u{b1c21}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{c8187}', '/']];
    let expected: Vec<char> = vec!['\u{c8187}', '/'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{feff}', '\u{57bce}', '*', '\u{a0}', 'j', '?', '$', '?'], vec![]];
    let expected: Vec<char> = vec!['\u{feff}', '\u{57bce}', '*', '\u{a0}', 'j', '?', '$', '?'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<Vec<char>> = vec![vec![]];
    let expected: Vec<char> = vec![];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<Vec<char>> = vec![vec![':', 'q', '叺'], vec!['Ѩ', '𪨹', '$'], vec!['K']];
    let expected: Vec<char> = vec![':', 'q', '叺', 'Ѩ', '𪨹', '$', 'K'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{7fdb7}', '/', '\u{474f6}'], vec!['¥'], vec![], vec!['\u{69857}', '"', 'å'], vec!['꼦', '\u{10b0ed}', '\u{1b}', '\u{70c4e}', '\u{67423}'], vec![':', 'U', 'L'], vec!['\u{75362}', '\n', '𓉫', ':']];
    let expected: Vec<char> = vec!['\u{7fdb7}', '/', '\u{474f6}', '¥', '\u{69857}', '"', 'å', '꼦', '\u{10b0ed}', '\u{1b}', '\u{70c4e}', '\u{67423}', ':', 'U', 'L', '\u{75362}', '\n', '𓉫', ':'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\0', '¥', '𣄕'], vec![], vec!['.'], vec!['"', '\u{7f}', '\u{202e}', '/', '\n', '찲'], vec!['🕴', 'H'], vec!['\\'], vec!['\\'], vec![], vec![], vec!['\'', 's', '\u{108441}'], vec!['\u{b9e6e}', '𫰼', '슈']];
    let expected: Vec<char> = vec!['\0', '¥', '𣄕', '.', '"', '\u{7f}', '\u{202e}', '/', '\n', '찲', '🕴', 'H', '\\', '\\', '\'', 's', '\u{108441}', '\u{b9e6e}', '𫰼', '슈'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec!['\u{7f}', '\\'], vec!['F']];
    let expected: Vec<char> = vec!['\u{7f}', '\\', 'F'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<Vec<char>> = vec![vec!['<', '\0', '🕴'], vec!['>', '\n', '@'], vec!['\0', '\u{64b34}', '>'], vec!['\u{5cdd0}', 'Ⱥ', '\u{85}', '\r'], vec!['\0', '2', '?', '&', 'n'], vec!['\u{76e1d}', '\u{84}', '\u{6e906}', '\u{66690}', '/', '\'', '`', '\0', '"', ':', '\u{9a0e1}'], vec!['𰩺'], vec![], vec!['𩹐', '\u{feff}'], vec!['\u{c68b0}'], vec!['\t', '\u{1b}', '\u{ab330}', '.', '?', 'q', '=']];
    let expected: Vec<char> = vec!['<', '\0', '🕴', '>', '\n', '@', '\0', '\u{64b34}', '>', '\u{5cdd0}', 'Ⱥ', '\u{85}', '\r', '\0', '2', '?', '&', 'n', '\u{76e1d}', '\u{84}', '\u{6e906}', '\u{66690}', '/', '\'', '`', '\0', '"', ':', '\u{9a0e1}', '𰩺', '𩹐', '\u{feff}', '\u{c68b0}', '\t', '\u{1b}', '\u{ab330}', '.', '?', 'q', '='];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<Vec<char>> = vec![vec!['&'], vec![]];
    let expected: Vec<char> = vec!['&'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<Vec<char>> = vec![vec!['k', 'T', '\u{dc566}', '\t', 'Ѩ', 'F'], vec!['\u{1b}'], vec!['\r', '`', 'T', '\u{97}', '\u{92}', '*', '.'], vec!['\u{7f}'], vec!['\0', '\u{5ab22}', '𦎈', '¦', '\t', '$', '\'', '\u{75f49}', '\u{907c8}'], vec![], vec!['¥', '¥'], vec!['\r', '\u{a7939}'], vec!['?', '\u{feff}', '5', '\u{5e348}', '\u{79a4a}', '*'], vec!['¥', '\u{202e}'], vec![',']];
    let expected: Vec<char> = vec!['k', 'T', '\u{dc566}', '\t', 'Ѩ', 'F', '\u{1b}', '\r', '`', 'T', '\u{97}', '\u{92}', '*', '.', '\u{7f}', '\0', '\u{5ab22}', '𦎈', '¦', '\t', '$', '\'', '\u{75f49}', '\u{907c8}', '¥', '¥', '\r', '\u{a7939}', '?', '\u{feff}', '5', '\u{5e348}', '\u{79a4a}', '*', '¥', '\u{202e}', ','];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{95098}', '"'], vec!['\u{1ad87}', '"'], vec!['g', '\\', '\u{f40a9}'], vec!['\u{e5c1b}', '?', '\u{47baf}', '\u{3aed8}'], vec![], vec!['$', '"', '结', '\u{1b}'], vec!['/', '\u{ec9cb}', '\u{375e5}', '\u{9a97a}', '\u{bb237}', 'R', '\u{69971}', ' ', 'k'], vec!['`', '.']];
    let expected: Vec<char> = vec!['\u{95098}', '"', '\u{1ad87}', '"', 'g', '\\', '\u{f40a9}', '\u{e5c1b}', '?', '\u{47baf}', '\u{3aed8}', '$', '"', '结', '\u{1b}', '/', '\u{ec9cb}', '\u{375e5}', '\u{9a97a}', '\u{bb237}', 'R', '\u{69971}', ' ', 'k', '`', '.'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec![], vec!['.', '\u{48362}', 'B'], vec!['.'], vec!['¤', '%', 'z'], vec!['T', '\r']];
    let expected: Vec<char> = vec!['.', '\u{48362}', 'B', '.', '¤', '%', 'z', 'T', '\r'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{b2c46}'], vec![']', '%', 'k', '\u{7d3bd}', '\u{10985e}', 'N'], vec!['\u{7d7ea}', 'á', 'o', '\u{6289e}'], vec!['\u{7156a}', '\u{1b}'], vec!['\u{a1c3b}'], vec![], vec!['Ñ'], vec!['$', '\u{7f}', '?']];
    let expected: Vec<char> = vec!['\u{b2c46}', ']', '%', 'k', '\u{7d3bd}', '\u{10985e}', 'N', '\u{7d7ea}', 'á', 'o', '\u{6289e}', '\u{7156a}', '\u{1b}', '\u{a1c3b}', 'Ñ', '$', '\u{7f}', '?'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec!['/'], vec!['`', '.', 'j', '%', '\u{62e82}'], vec!['\u{895cd}']];
    let expected: Vec<char> = vec!['/', '`', '.', 'j', '%', '\u{62e82}', '\u{895cd}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<Vec<char>> = vec![vec!['�', '"', '?', '~', '.'], vec!['h', '_', '<', '\u{1b}', '`', '🕴', 'T', ']', '|']];
    let expected: Vec<char> = vec!['�', '"', '?', '~', '.', 'h', '_', '<', '\u{1b}', '`', '🕴', 'T', ']', '|'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<Vec<char>> = vec![vec!['`', '¿'], vec!['\u{9de9d}', '\u{a8a98}', ':', '\u{a8f00}', '\u{4e791}', 'D', '\u{a98d9}', '=', 'K', '\t', '\u{ae9b0}', '�', '&', '*', '\n', '=', '\u{89fe0}'], vec!['&', '🕴', '\u{feff}', '�']];
    let expected: Vec<char> = vec!['`', '¿', '\u{9de9d}', '\u{a8a98}', ':', '\u{a8f00}', '\u{4e791}', 'D', '\u{a98d9}', '=', 'K', '\t', '\u{ae9b0}', '�', '&', '*', '\n', '=', '\u{89fe0}', '&', '🕴', '\u{feff}', '�'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<Vec<char>> = vec![vec!['9', '\u{98845}', '*', '\u{6382c}', '\u{90db3}', '\'', '�', '\u{94ad1}', '=', '*', '?', ';', '\t', '\u{5bef7}', 'z', '\n', '8', '\0', '/', '&', '.']];
    let expected: Vec<char> = vec!['9', '\u{98845}', '*', '\u{6382c}', '\u{90db3}', '\'', '�', '\u{94ad1}', '=', '*', '?', ';', '\t', '\u{5bef7}', 'z', '\n', '8', '\0', '/', '&', '.'];
    assert_eq!(concatenate_impl(arg_0), expected);
}