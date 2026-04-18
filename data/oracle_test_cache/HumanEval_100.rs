#[test]
fn oracle_case_0() {
    let arg_0: usize = 3;
    let expected: Vec<usize> = vec![3, 5, 7];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: usize = 1;
    let expected: Vec<usize> = vec![1];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: usize = 7;
    let expected: Vec<usize> = vec![7, 9, 11, 13, 15, 17, 19];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: usize = 0;
    let expected: Vec<usize> = vec![];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: usize = 4;
    let expected: Vec<usize> = vec![4, 6, 8, 10];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: usize = 6;
    let expected: Vec<usize> = vec![6, 8, 10, 12, 14, 16];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: usize = 8;
    let expected: Vec<usize> = vec![8, 10, 12, 14, 16, 18, 20, 22];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: usize = 5;
    let expected: Vec<usize> = vec![5, 7, 9, 11, 13];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: usize = 12;
    let expected: Vec<usize> = vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: usize = 2;
    let expected: Vec<usize> = vec![2, 4];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: usize = 15;
    let expected: Vec<usize> = vec![15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: usize = 18;
    let expected: Vec<usize> = vec![18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: usize = 11;
    let expected: Vec<usize> = vec![11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: usize = 9;
    let expected: Vec<usize> = vec![9, 11, 13, 15, 17, 19, 21, 23, 25];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: usize = 19;
    let expected: Vec<usize> = vec![19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55];
    assert_eq!(make_a_pile(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: usize = 16;
    let expected: Vec<usize> = vec![16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46];
    assert_eq!(make_a_pile(arg_0), expected);
}