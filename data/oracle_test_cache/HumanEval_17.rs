#[test]
fn oracle_case_0() {
    let arg_0: &str = "o.|.|o|o|o";
    let expected: Option<Vec<u8>> = Some(vec![4, 1, 1, 2, 2, 4]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: &str = "ᦻ`�=[𝍩'ⷀ𜾉Ⱥ໗-<-<b";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: &str = "o|";
    let expected: Option<Vec<u8>> = Some(vec![2]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: &str = "²¥{𞹙\\7z%'Ѩⵐ??'8𑎋";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: &str = "o|o|.|o.|oo|o|o.|.|o|";
    let expected: Option<Vec<u8>> = Some(vec![2, 2, 1, 4, 1, 4, 2, 2, 4, 1, 1, 2]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: &str = "࠷.:jbΠ.𑴄:%𖽭🕴d?\u{16ff1}𑙙K𝒾\u{a4d}ଊ.ᾆ𑽕𑎋){ª𐎩";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: &str = ".|";
    let expected: Option<Vec<u8>> = Some(vec![1]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: &str = "{=:🕴,:¥sຓ𞗿$𞹍2&ᙤ%F}Ѩ].Ѩ𑤱\u{11c9a}ຎ𧓅🂮";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: &str = ".|.|oo|o|.|.|o|oo";
    let expected: Option<Vec<u8>> = Some(vec![1, 1, 4, 2, 2, 1, 1, 2, 4, 4]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: &str = "\\{^=𞤡B:Cஷ%⿵`-/�𑒛𛅥\u{11d3d}/𝞎A¥:<ꟓ𞺋💸\u{59e}🕴\"\u{820}9";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: &str = "o.|ooo";
    let expected: Option<Vec<u8>> = Some(vec![4, 1, 4, 4, 4]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: &str = "?¥\"\u{b55}$*8}\u{a806}.+𖹜`\u{afb}2k\u{c46}*Xx\u{fe20}.𐀫1";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: &str = "o|o|o|ooo|o.|o|";
    let expected: Option<Vec<u8>> = Some(vec![2, 2, 2, 4, 4, 2, 4, 1, 2]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: &str = "𑙓🁦𐳣ࡩ9%<𐇚𑏗Ⱥ𒓊𝐾/`Ⱥ6ꕴQn𝍑=?𑊈?𑌲";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: &str = ".|o|o|o.|";
    let expected: Option<Vec<u8>> = Some(vec![1, 2, 2, 4, 1]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: &str = ")*᠈\"𝕊𞴚Ѩ૧$&K/\u{1e08f}𝒬";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: &str = ".|.|";
    let expected: Option<Vec<u8>> = Some(vec![1, 1]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: &str = "🫕/Ћ𑌃$ඵ[𖫭`1ꨂ𐜱";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: &str = "o.|o|.|";
    let expected: Option<Vec<u8>> = Some(vec![4, 1, 2, 1]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: &str = ".'ㆈo𗓊<` ꩑𞹔′:𑌭�=꧔O).\u{113be}ଢ଼￼\"Ò=";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: &str = "o|o.|";
    let expected: Option<Vec<u8>> = Some(vec![2, 4, 1]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: &str = "ข𛰲ቜἛￒ𝋌𞹑:🣀\u{11f01}<";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: &str = "o|o|o|o|o.|.|.|.|";
    let expected: Option<Vec<u8>> = Some(vec![2, 2, 2, 2, 4, 1, 1, 1, 1]);
    assert_eq!(parse_music(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: &str = "*\u{1cf02}᪣𞋿\\/F>";
    let expected: Option<Vec<u8>> = None;
    assert_eq!(parse_music(arg_0), expected);
}