#[test]
fn oracle_case_0() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{6310b}'], vec!['l'], vec!['<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z', '%', '\'', 'Ѩ', '\n', '\u{f455d}', ']', '\u{5ebad}', '\u{aed06}', '\u{37999}', '\u{d3106}', '𗰛', '´'], vec!['b'], vec!['?', '\n', ':', '%'], vec!['Ѩ', '\u{1b}'], vec!['{'], vec![], vec!['\u{202e}', '\u{feff}', '$', 'k', '"', '\u{109c7c}'], vec![')', '{', 'ª', '\u{4c6d0}', '\u{b}', 'g', '\u{9e}', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$', '\u{56e89}', '\r'], vec![], vec!['&'], vec!['%', '\n', '\r', 'r'], vec!['\u{b}', '�', 'K', 'ù'], vec!['%'], vec![], vec![], vec!['Ⱥ', 'þ', '\\', '{', '^', '=', '\u{78cc7}'], vec![]];
    let expected: Vec<char> = vec!['\u{6310b}', 'l', '<', '\u{1b}', '\r', '¥', '{', '\u{98ba3}', '\\', '7', 'z', '%', '\'', 'Ѩ', '\n', '\u{f455d}', ']', '\u{5ebad}', '\u{aed06}', '\u{37999}', '\u{d3106}', '𗰛', '´', 'b', '?', '\n', ':', '%', 'Ѩ', '\u{1b}', '{', '\u{202e}', '\u{feff}', '$', 'k', '"', '\u{109c7c}', ')', '{', 'ª', '\u{4c6d0}', '\u{b}', 'g', '\u{9e}', '/', '鷫', '3', '?', '\'', '\u{e65e3}', '*', '\u{feff}', '$', '\u{56e89}', '\r', '&', '%', '\n', '\r', 'r', '\u{b}', '�', 'K', 'ù', '%', 'Ⱥ', 'þ', '\\', '{', '^', '=', '\u{78cc7}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<Vec<char>> = vec![];
    let expected: Vec<char> = vec![];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<Vec<char>> = vec![vec![]];
    let expected: Vec<char> = vec![];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<Vec<char>> = vec![vec!['%', '𑩄', '`', '-', '/'], vec![], vec![]];
    let expected: Vec<char> = vec!['%', '𑩄', '`', '-', '/'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<Vec<char>> = vec![vec!['¥'], vec![], vec!['A', '¥', ':', '<'], vec!['/', '\u{79d17}', '\u{b}', '\u{d35be}', 'Ѩ'], vec![]];
    let expected: Vec<char> = vec!['¥', 'A', '¥', ':', '<', '/', '\u{79d17}', '\u{b}', '\u{d35be}', 'Ѩ'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<Vec<char>> = vec![vec!['9']];
    let expected: Vec<char> = vec!['9'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec![], vec![], vec!['$'], vec!['\r', '\u{b}', '"', '\u{c523d}', '\u{620d4}']];
    let expected: Vec<char> = vec!['$', '\r', '\u{b}', '"', '\u{c523d}', '\u{620d4}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec!['\u{e8de6}', '𧜫', 'z', '\r', 'g', '\t', '\'', '\u{b8371}', '\u{7fc85}'], vec!['{', '"']];
    let expected: Vec<char> = vec!['\u{e8de6}', '𧜫', 'z', '\r', 'g', '\t', '\'', '\u{b8371}', '\u{7fc85}', '{', '"'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{15418}', '(', '𤊲', 'j'], vec!['\u{c8187}', '/', '`', 'Ⱥ'], vec!['Q', 'n', '\u{fd7be}', '=', '?', '\u{202e}', '{'], vec!['\u{89ca1}', '\0', '$', '*'], vec!['\u{f2311}', '-', '焗', '.', ' ', 'Ⱥ', 'C', 'o', '/'], vec![], vec!['&', '/', '\u{474f6}'], vec!['\u{6f275}', '\u{8cefd}', '¥', '.', '\'', '\u{fe3ca}', 'o', '\u{365bb}', '<', '`', ' ', '𩏜', '\u{ae88f}', '𓉫', ':', '\u{92fd1}', '�', '=', '\0', '\u{5afbb}', '¥', '\0', '¥'], vec![')', '𖪔', '⣒', '\\'], vec!['\n', '찲', '\u{b580b}', ':', '&', '\u{abd91}']];
    let expected: Vec<char> = vec!['\u{15418}', '(', '𤊲', 'j', '\u{c8187}', '/', '`', 'Ⱥ', 'Q', 'n', '\u{fd7be}', '=', '?', '\u{202e}', '{', '\u{89ca1}', '\0', '$', '*', '\u{f2311}', '-', '焗', '.', ' ', 'Ⱥ', 'C', 'o', '/', '&', '/', '\u{474f6}', '\u{6f275}', '\u{8cefd}', '¥', '.', '\'', '\u{fe3ca}', 'o', '\u{365bb}', '<', '`', ' ', '𩏜', '\u{ae88f}', '𓉫', ':', '\u{92fd1}', '�', '=', '\0', '\u{5afbb}', '¥', '\0', '¥', ')', '𖪔', '⣒', '\\', '\n', '찲', '\u{b580b}', ':', '&', '\u{abd91}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec!['$', ':'], vec!['\u{c717e}', '\u{b9e6e}'], vec!['슈', '\u{90}', '\u{64a2c}'], vec!['\t', '\u{aa1bd}', '\u{8b812}', 'b'], vec![], vec![], vec!['\u{d3a91}', '\0', '\t'], vec![]];
    let expected: Vec<char> = vec!['$', ':', '\u{c717e}', '\u{b9e6e}', '슈', '\u{90}', '\u{64a2c}', '\t', '\u{aa1bd}', '\u{8b812}', 'b', '\u{d3a91}', '\0', '\t'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{334aa}', '©', '\u{64b34}', '>', '!'], vec!['?', '\r', ':', '\u{8f912}', '2', '?', '&', 'n'], vec![], vec![], vec!['\u{b}', ',', '\u{59b4f}', 'Ⱥ']];
    let expected: Vec<char> = vec!['\u{334aa}', '©', '\u{64b34}', '>', '!', '?', '\r', ':', '\u{8f912}', '2', '?', '&', 'n', '\u{b}', ',', '\u{59b4f}', 'Ⱥ'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\'', '7', ' ', '=', '\u{666ae}', '🕴', '\u{3fac4}', 'k', '{', '\u{539b9}', '\0', 'x'], vec!['*', '?', 'q', '=', '\u{dcc73}', '\u{d3f1b}'], vec!['�']];
    let expected: Vec<char> = vec!['\'', '7', ' ', '=', '\u{666ae}', '🕴', '\u{3fac4}', 'k', '{', '\u{539b9}', '\0', 'x', '*', '?', 'q', '=', '\u{dcc73}', '\u{d3f1b}', '�'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\n', '\u{ab334}', '\u{1b}'], vec!['?', '%', 'i', '\r', '\\'], vec!['\r', '\u{202e}', '"', '\u{cb75e}', '\0'], vec!['\t', '\u{97}'], vec!['=']];
    let expected: Vec<char> = vec!['\n', '\u{ab334}', '\u{1b}', '?', '%', 'i', '\r', '\\', '\r', '\u{202e}', '"', '\u{cb75e}', '\0', '\t', '\u{97}', '='];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\t', '\u{1e715}'], vec![], vec!['\u{7c48d}', '2', '\u{feff}', 'Ѩ', '�'], vec!['\u{100d00}']];
    let expected: Vec<char> = vec!['\t', '\u{1e715}', '\u{7c48d}', '2', '\u{feff}', 'Ѩ', '�', '\u{100d00}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\0']];
    let expected: Vec<char> = vec!['\0'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<Vec<char>> = vec![vec!['$', '🕴', 'l', '.', '\u{8da16}', '\u{e7ee0}', '1', 'i', '\u{202e}', '\u{202e}', ',', '\u{eb65}'], vec![], vec!['𦊧', '\u{a74fe}', '\u{7bf68}'], vec!['"', '*', 'Ѩ'], vec![], vec!['#', '�', 'n', '\u{3ee31}']];
    let expected: Vec<char> = vec!['$', '🕴', 'l', '.', '\u{8da16}', '\u{e7ee0}', '1', 'i', '\u{202e}', '\u{202e}', ',', '\u{eb65}', '𦊧', '\u{a74fe}', '\u{7bf68}', '"', '*', 'Ѩ', '#', '�', 'n', '\u{3ee31}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec!['\u{1467e}'], vec!['{'], vec!['\u{ec9cb}', '\u{375e5}', '\u{9a97a}', '\u{bb237}', 'R', '\u{69971}', ' ', 'k', '\u{9a27f}', 'e', '\t', '3', '"', 'Á', '.', '\u{48362}', 'B', '¥', '.', '\u{1b}', 'e'], vec!['%']];
    let expected: Vec<char> = vec!['\u{1467e}', '{', '\u{ec9cb}', '\u{375e5}', '\u{9a97a}', '\u{bb237}', 'R', '\u{69971}', ' ', 'k', '\u{9a27f}', 'e', '\t', '3', '"', 'Á', '.', '\u{48362}', 'B', '¥', '.', '\u{1b}', 'e', '%'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{e71fa}', '¥'], vec!['\u{b}', '\u{52b37}', '"', '?'], vec!['🕴'], vec![], vec!['\u{7d3bd}', '\u{10985e}', 'N'], vec!['á', 'o', '\u{6289e}', '\u{59055}', 'n', '🕴', '\u{33a55}', 'Ѩ'], vec![], vec!['\u{82cc5}', 'u', '\u{ad}', '🕴', '\u{d1875}', '/', '*'], vec!['Ѩ', '\u{72183}'], vec!['<'], vec!['\u{62e82}', '\u{895cd}']];
    let expected: Vec<char> = vec!['\u{e71fa}', '¥', '\u{b}', '\u{52b37}', '"', '?', '🕴', '\u{7d3bd}', '\u{10985e}', 'N', 'á', 'o', '\u{6289e}', '\u{59055}', 'n', '🕴', '\u{33a55}', 'Ѩ', '\u{82cc5}', 'u', '\u{ad}', '🕴', '\u{d1875}', '/', '*', 'Ѩ', '\u{72183}', '<', '\u{62e82}', '\u{895cd}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{ad86f}', '\u{d4824}'], vec!['¥', '_'], vec!['🕴', 'T', ']', '|', 'e'], vec!['\u{feff}', '\n', '\u{cf1ac}', '\r', '#'], vec!['%', 'a', '\u{925de}', '=', '\u{51fd3}', '\u{b}', '\u{157d6}', '�', '\u{abdd7}'], vec!['|', '¥', 'Q'], vec!['�', '¥', '£', '�', 'Ѩ', '\u{38816}', ' ', '\u{155b2}'], vec!['Ⱥ', '{', 'W', 'H', ':', '\u{5a173}', '6', 'C', '\t', '\t', '&', '%', 'F', '\u{1b}', '\u{3bd77}', 'g', ':', '"', 'Ѩ', '\u{b}', '\\', 'Ѩ', '\u{1b}', '\n', '¥', '\u{7f}', '&'], vec!['='], vec!['\t', '\u{6f029}', 't'], vec!['\u{1b}', '=', '"'], vec!['5', '`'], vec!['\u{b}', '\u{cdf3c}', '"', '𘒫', '.', '\u{1bbc7}'], vec![], vec!['\u{8b92c}', '\r', '\u{e3673}', 'õ', ':', '{', '\u{6da2c}', '\u{544da}'], vec!['🕴', '^', '🕴', '=', '\u{feff}', '\u{731b8}', '(']];
    let expected: Vec<char> = vec!['\u{ad86f}', '\u{d4824}', '¥', '_', '🕴', 'T', ']', '|', 'e', '\u{feff}', '\n', '\u{cf1ac}', '\r', '#', '%', 'a', '\u{925de}', '=', '\u{51fd3}', '\u{b}', '\u{157d6}', '�', '\u{abdd7}', '|', '¥', 'Q', '�', '¥', '£', '�', 'Ѩ', '\u{38816}', ' ', '\u{155b2}', 'Ⱥ', '{', 'W', 'H', ':', '\u{5a173}', '6', 'C', '\t', '\t', '&', '%', 'F', '\u{1b}', '\u{3bd77}', 'g', ':', '"', 'Ѩ', '\u{b}', '\\', 'Ѩ', '\u{1b}', '\n', '¥', '\u{7f}', '&', '=', '\t', '\u{6f029}', 't', '\u{1b}', '=', '"', '5', '`', '\u{b}', '\u{cdf3c}', '"', '𘒫', '.', '\u{1bbc7}', '\u{8b92c}', '\r', '\u{e3673}', 'õ', ':', '{', '\u{6da2c}', '\u{544da}', '🕴', '^', '🕴', '=', '\u{feff}', '\u{731b8}', '('];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{70b39}', 'Q', '\u{103893}', '\0', '^', '5', 'À', '𘘗', '\u{cea91}', '\u{1b}', '^', '\0', '\u{feff}', '=', '!', 'a', '-', 'M', '\t', '>', '*', 'e', '-', ')'], vec!['\u{41d76}', '\u{8d05c}', 'C'], vec!['k', '<', 'i', '\u{b}', 'E', '\u{48f7f}', '{', '<', 'x', '\u{a9ec7}', '\u{10778f}', '\u{c71ed}', '\u{7f}', '\u{b}', '/', '\u{b620a}', 'p', '\u{b}', '\u{69c82}', 'O', '\u{1b}']];
    let expected: Vec<char> = vec!['\u{70b39}', 'Q', '\u{103893}', '\0', '^', '5', 'À', '𘘗', '\u{cea91}', '\u{1b}', '^', '\0', '\u{feff}', '=', '!', 'a', '-', 'M', '\t', '>', '*', 'e', '-', ')', '\u{41d76}', '\u{8d05c}', 'C', 'k', '<', 'i', '\u{b}', 'E', '\u{48f7f}', '{', '<', 'x', '\u{a9ec7}', '\u{10778f}', '\u{c71ed}', '\u{7f}', '\u{b}', '/', '\u{b620a}', 'p', '\u{b}', '\u{69c82}', 'O', '\u{1b}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<Vec<char>> = vec![vec!['>', 'û', 'Ã', '\u{9deca}', '�'], vec![], vec!['\u{202e}'], vec!['Z', '\u{f7949}', '\u{94aa7}', '\u{202e}'], vec!['Ⱥ', '9', '\u{c1933}', '\n', 'Ѩ', '\u{f1b9d}', '\u{feff}', 'Ѩ', '|', '\\', '\u{feff}', '\u{feff}', '`', 'f'], vec!['\u{b}', '\u{6d553}', '\u{feff}', '\\', '?', '\\'], vec!['\u{915fb}', 'N', '�', '%'], vec!['?', '\u{9ebe3}', '>', '?', 'Ѩ', '\u{1b}'], vec!['$']];
    let expected: Vec<char> = vec!['>', 'û', 'Ã', '\u{9deca}', '�', '\u{202e}', 'Z', '\u{f7949}', '\u{94aa7}', '\u{202e}', 'Ⱥ', '9', '\u{c1933}', '\n', 'Ѩ', '\u{f1b9d}', '\u{feff}', 'Ѩ', '|', '\\', '\u{feff}', '\u{feff}', '`', 'f', '\u{b}', '\u{6d553}', '\u{feff}', '\\', '?', '\\', '\u{915fb}', 'N', '�', '%', '?', '\u{9ebe3}', '>', '?', 'Ѩ', '\u{1b}', '$'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<Vec<char>> = vec![vec!['🕴', '¥', '/', 'Ⱥ', '`', '\u{b}']];
    let expected: Vec<char> = vec!['🕴', '¥', '/', 'Ⱥ', '`', '\u{b}'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<Vec<char>> = vec![vec![], vec!['\\', '<', '5', '='], vec!['`', '¢'], vec!['h'], vec!['\u{feff}', '¥', 'J', '<', '¥', '\'', '/', '0', 'Ѩ', '\u{1b}', '\u{6f060}', '{']];
    let expected: Vec<char> = vec!['\\', '<', '5', '=', '`', '¢', 'h', '\u{feff}', '¥', 'J', '<', '¥', '\'', '/', '0', 'Ѩ', '\u{1b}', '\u{6f060}', '{'];
    assert_eq!(concatenate_impl(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<Vec<char>> = vec![vec!['\u{b}', '\u{feff}', '~'], vec!['\0', 'G', '𬜦', '¥', '\u{dd59d}', '7', '='], vec!['%']];
    let expected: Vec<char> = vec!['\u{b}', '\u{feff}', '~', '\0', 'G', '𬜦', '¥', '\u{dd59d}', '7', '=', '%'];
    assert_eq!(concatenate_impl(arg_0), expected);
}