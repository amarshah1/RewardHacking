#[test]
fn oracle_case_0() {
    let arg_0: Vec<u8> = vec![138, 188, 178];
    let expected: Vec<Vec<u8>> = vec![vec![138], vec![138, 188], vec![138, 188, 178]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<u8> = vec![];
    let expected: Vec<Vec<u8>> = vec![];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<u8> = vec![24, 173, 19, 175, 248, 225, 236];
    let expected: Vec<Vec<u8>> = vec![vec![24], vec![24, 173], vec![24, 173, 19], vec![24, 173, 19, 175], vec![24, 173, 19, 175, 248], vec![24, 173, 19, 175, 248, 225], vec![24, 173, 19, 175, 248, 225, 236]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<u8> = vec![155, 222, 13, 49];
    let expected: Vec<Vec<u8>> = vec![vec![155], vec![155, 222], vec![155, 222, 13], vec![155, 222, 13, 49]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<u8> = vec![41, 96];
    let expected: Vec<Vec<u8>> = vec![vec![41], vec![41, 96]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<u8> = vec![138, 148, 91];
    let expected: Vec<Vec<u8>> = vec![vec![138], vec![138, 148], vec![138, 148, 91]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<u8> = vec![179, 143, 95, 241, 107, 77];
    let expected: Vec<Vec<u8>> = vec![vec![179], vec![179, 143], vec![179, 143, 95], vec![179, 143, 95, 241], vec![179, 143, 95, 241, 107], vec![179, 143, 95, 241, 107, 77]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<u8> = vec![74, 31];
    let expected: Vec<Vec<u8>> = vec![vec![74], vec![74, 31]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<u8> = vec![255];
    let expected: Vec<Vec<u8>> = vec![vec![255]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<u8> = vec![151, 212, 226, 173];
    let expected: Vec<Vec<u8>> = vec![vec![151], vec![151, 212], vec![151, 212, 226], vec![151, 212, 226, 173]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<u8> = vec![173, 153, 53, 32, 50, 0, 68, 122, 81, 42];
    let expected: Vec<Vec<u8>> = vec![vec![173], vec![173, 153], vec![173, 153, 53], vec![173, 153, 53, 32], vec![173, 153, 53, 32, 50], vec![173, 153, 53, 32, 50, 0], vec![173, 153, 53, 32, 50, 0, 68], vec![173, 153, 53, 32, 50, 0, 68, 122], vec![173, 153, 53, 32, 50, 0, 68, 122, 81], vec![173, 153, 53, 32, 50, 0, 68, 122, 81, 42]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<u8> = vec![203, 213, 115, 208, 99];
    let expected: Vec<Vec<u8>> = vec![vec![203], vec![203, 213], vec![203, 213, 115], vec![203, 213, 115, 208], vec![203, 213, 115, 208, 99]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<u8> = vec![21, 218];
    let expected: Vec<Vec<u8>> = vec![vec![21], vec![21, 218]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<u8> = vec![119, 199];
    let expected: Vec<Vec<u8>> = vec![vec![119], vec![119, 199]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<u8> = vec![241, 1, 172, 79];
    let expected: Vec<Vec<u8>> = vec![vec![241], vec![241, 1], vec![241, 1, 172], vec![241, 1, 172, 79]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<u8> = vec![225, 99];
    let expected: Vec<Vec<u8>> = vec![vec![225], vec![225, 99]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<u8> = vec![250, 99];
    let expected: Vec<Vec<u8>> = vec![vec![250], vec![250, 99]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<u8> = vec![242];
    let expected: Vec<Vec<u8>> = vec![vec![242]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<u8> = vec![151];
    let expected: Vec<Vec<u8>> = vec![vec![151]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<u8> = vec![137, 205];
    let expected: Vec<Vec<u8>> = vec![vec![137], vec![137, 205]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<u8> = vec![71, 149, 80, 71, 7];
    let expected: Vec<Vec<u8>> = vec![vec![71], vec![71, 149], vec![71, 149, 80], vec![71, 149, 80, 71], vec![71, 149, 80, 71, 7]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<u8> = vec![249, 229, 66];
    let expected: Vec<Vec<u8>> = vec![vec![249], vec![249, 229], vec![249, 229, 66]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<u8> = vec![149, 214];
    let expected: Vec<Vec<u8>> = vec![vec![149], vec![149, 214]];
    assert_eq!(all_prefixes(&arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<u8> = vec![172, 239];
    let expected: Vec<Vec<u8>> = vec![vec![172], vec![172, 239]];
    assert_eq!(all_prefixes(&arg_0), expected);
}