#[test]
fn oracle_case_0() {
    let arg_0: &str = "E";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: &str = "";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: &str = ".o H";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: &str = "%";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: &str = "nK;J y";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: &str = "B5Fl >";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: &str = "H t";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: &str = "2!1 1";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: &str = "iYCFrmwmT O";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: &str = "pK";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: &str = "YxyVk: o";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: &str = "FR";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: &str = "v X";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: &str = "|A";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: &str = "n N";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: &str = "^E`N";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: &str = "q";
    let expected: bool = true;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: &str = "ᦻ`�=[𝍩'ⷀ𜾉Ⱥ໗-<-<b";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: &str = "²¥{𞹙\\7z%'Ѩⵐ??'8𑎋";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: &str = "";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: &str = "࠷.:jbΠ.𑴄:%𖽭🕴d?\u{16ff1}𑙙K𝒾\u{a4d}ଊ.ᾆ𑽕𑎋){ª𐎩";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: &str = "{=:🕴,:¥sຓ𞗿$𞹍2&ᙤ%F}Ѩ].Ѩ𑤱\u{11c9a}ຎ𧓅🂮";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: &str = "\\{^=𞤡B:Cஷ%⿵`-/�𑒛𛅥\u{11d3d}/𝞎A¥:<ꟓ𞺋💸\u{59e}🕴\"\u{820}9";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: &str = "?¥\"\u{b55}$*8}\u{a806}.+𖹜`\u{afb}2k\u{c46}*Xx\u{fe20}.𐀫1";
    let expected: bool = false;
    assert_eq!(check_if_last_char_is_a_letter(arg_0), expected);
}