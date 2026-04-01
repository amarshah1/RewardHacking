#[test]
fn test_single_group() {
    let input = vec!['(', ')'];
    let expected = vec![vec!['(', ')']];
    assert_eq!(separate_paren_groups(&input), expected);
}

#[test]
fn test_multiple_simple_groups() {
    let input = vec!['(', ')', '(', ')'];
    let expected = vec![vec!['(', ')'], vec!['(', ')']];
    assert_eq!(separate_paren_groups(&input), expected);
}

#[test]
fn test_nested_parentheses() {
    let input = vec!['(', '(', ')', ')'];
    let expected = vec![vec!['(', '(', ')', ')']];
    assert_eq!(separate_paren_groups(&input), expected);
}

#[test]
fn test_groups_with_spaces() {
    let input = vec!['(', ' ', ')', ' ', '(', '(', ' ', ')', ')', ' ', '(', '(', ' ', ')', '(', ' ', ')', ')'];
    let expected = vec![
        vec!['(', ')'],
        vec!['(', '(', ')', ')'],
        vec!['(', '(', ')', '(', ')', ')']
    ];
    assert_eq!(separate_paren_groups(&input), expected);
}

#[test]
fn test_empty_input() {
    let input = Vec::<char>::new();
    let expected = Vec::<Vec<char>>::new();
    assert_eq!(separate_paren_groups(&input), expected);
}

#[test]
fn test_complex_nested_groups() {
    let input = vec!['(', '(', '(', ')', ')', ')', '(', ')', '(', '(', ')', '(', '(', ')', ')', ')'];
    let expected = vec![
        vec!['(', '(', '(', ')', ')', ')'],
        vec!['(', ')'],
        vec!['(', '(', ')', '(', '(', ')', ')', ')']
    ];
    assert_eq!(separate_paren_groups(&input), expected);
}

#[test]
fn test_only_spaces() {
    let input = vec![' ', ' ', ' '];
    let expected = Vec::<Vec<char>>::new();
    assert_eq!(separate_paren_groups(&input), expected);
}

#[test]
fn test_deeply_nested_single_group() {
    let input = vec!['(', '(', '(', '(', ')', ')', ')', ')'];
    let expected = vec![vec!['(', '(', '(', '(', ')', ')', ')', ')']];
    assert_eq!(separate_paren_groups(&input), expected);
}