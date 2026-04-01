#[test]
fn test_example_case() {
    let result = parse_nested_parens("(()()) ((())) () ((())()())");
    assert_eq!(result, Some(vec![2, 3, 1, 3]));
}

#[test]
fn test_empty_string() {
    let result = parse_nested_parens("");
    assert_eq!(result, Some(Vec::<usize>::new()));
}

#[test]
fn test_single_group_various_depths() {
    assert_eq!(parse_nested_parens("()"), Some(vec![1]));
    assert_eq!(parse_nested_parens("(())"), Some(vec![2]));
    assert_eq!(parse_nested_parens("((()))"), Some(vec![3]));
    assert_eq!(parse_nested_parens("(((())))"), Some(vec![4]));
}

#[test]
fn test_multiple_groups_same_line() {
    let result = parse_nested_parens("() () ()");
    assert_eq!(result, Some(vec![1, 1, 1]));
    
    let result = parse_nested_parens("(()) (()()) ((()))");
    assert_eq!(result, Some(vec![2, 2, 3]));
}

#[test]
fn test_unbalanced_parentheses() {
    assert_eq!(parse_nested_parens("(()"), None);
    assert_eq!(parse_nested_parens("())"), None);
    assert_eq!(parse_nested_parens("() (()"), None);
    assert_eq!(parse_nested_parens("() ()) ()"), None);
}

#[test]
fn test_complex_nested_patterns() {
    let result = parse_nested_parens("(()()()) (()(()))");
    assert_eq!(result, Some(vec![2, 3]));
    
    let result = parse_nested_parens("(((()))) (()((())()))");
    assert_eq!(result, Some(vec![4, 4]));
}

#[test]
fn test_extra_spaces() {
    let result = parse_nested_parens("()  ()   ()");
    assert_eq!(result, Some(vec![1, 1, 1]));
    
    let result = parse_nested_parens("  (())  ((()))  ");
    assert_eq!(result, Some(vec![2, 3]));
}

#[test]
fn test_wrong_bracket_types() {
    assert_eq!(parse_nested_parens("[]"), None);
    assert_eq!(parse_nested_parens("() []"), None);
    assert_eq!(parse_nested_parens("(]"), None);
}