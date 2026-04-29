#[test]
fn oracle_case_0() {
    let arg_0: &str = "(())()()()(((()))) ";
    let expected: Option<Vec<usize>> = Some(vec![4]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: &str = "())(((  ) ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: &str = "(((((((()))))))) ";
    let expected: Option<Vec<usize>> = Some(vec![8]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: &str = ")())( ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: &str = "(()(())) ";
    let expected: Option<Vec<usize>> = Some(vec![3]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: &str = "   ((  ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: &str = "";
    let expected: Option<Vec<usize>> = Some(vec![]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: &str = " )   ())()))";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: &str = "()(()) ";
    let expected: Option<Vec<usize>> = Some(vec![2]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: &str = " ((  )((";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: &str = "()";
    let expected: Option<Vec<usize>> = Some(vec![1]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: &str = "( ))(     ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: &str = "(((()))) ";
    let expected: Option<Vec<usize>> = Some(vec![4]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: &str = " ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: &str = "() (()) ";
    let expected: Option<Vec<usize>> = Some(vec![1, 2]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: &str = "( ))) ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: &str = "(()) ";
    let expected: Option<Vec<usize>> = Some(vec![2]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: &str = "( ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: &str = "((()))";
    let expected: Option<Vec<usize>> = Some(vec![3]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: &str = " ((  )) ())";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: &str = "(())";
    let expected: Option<Vec<usize>> = Some(vec![2]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: &str = "( ( ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: &str = "(()())";
    let expected: Option<Vec<usize>> = Some(vec![2]);
    assert_eq!(parse_nested_parens(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: &str = "(((( )   ";
    let expected: Option<Vec<usize>> = None;
    assert_eq!(parse_nested_parens(arg_0), expected);
}