#[test]
fn oracle_case_0() {
    let arg_0: Vec<Vec<i32>> = vec![vec![0], vec![3, 0, 3, 5, 0, -2, 0], vec![-7, -2, 4, 0, 11, -2, 0]];
    let arg_1: i32 = -3;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2, 1, 1], vec![-1, 2, -1, -2, -2, 1], vec![-1, 4, -1], vec![4, 3, -2, -2, 0], vec![-3, 14, 4, 0, 0, -1]];
    let arg_1: i32 = -3;
    let expected: Vec<(usize, usize)> = vec![(4, 0)];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = 3;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![0, 0], vec![]];
    let arg_1: i32 = 17;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![-1]];
    let arg_1: i32 = 2;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2], vec![-2, 3, 0, -5, -5, -4, -3, 2, 1, 1, -3]];
    let arg_1: i32 = -1;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<Vec<i32>> = vec![vec![]];
    let arg_1: i32 = 0;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<Vec<i32>> = vec![vec![0, 8, 3], vec![4, -4], vec![], vec![], vec![0, -10], vec![], vec![], vec![0, 1, 2, 0, 5, 2, -3, -3]];
    let arg_1: i32 = 5;
    let expected: Vec<(usize, usize)> = vec![(7, 4)];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = -3;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![0], vec![0, 0], vec![]];
    let arg_1: i32 = 4;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = 0;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![4], vec![]];
    let arg_1: i32 = 1;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-2, 7, -3, 2, -1], vec![]];
    let arg_1: i32 = -1;
    let expected: Vec<(usize, usize)> = vec![(0, 4)];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![2, 6, 6]];
    let arg_1: i32 = 0;
    let expected: Vec<(usize, usize)> = vec![(0, 2), (0, 1), (0, 0)];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<Vec<i32>> = vec![vec![0, -8, 2, 1], vec![-1, 2, 3, 0, -7, 0, 18, -2], vec![4, 3, 0, 15], vec![-5, 0, 0]];
    let arg_1: i32 = -7;
    let expected: Vec<(usize, usize)> = vec![(1, 4)];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-2, 0, 3, -6, -2]];
    let arg_1: i32 = -1;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<Vec<i32>> = vec![];
    let arg_1: i32 = 2;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<Vec<i32>> = vec![vec![2, 5, 3, 2, 2, -4], vec![2]];
    let arg_1: i32 = 10;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<Vec<i32>> = vec![vec![], vec![], vec![-3, 3]];
    let arg_1: i32 = 0;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<Vec<i32>> = vec![vec![-1, 1, 7], vec![3, -2]];
    let arg_1: i32 = 4;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<Vec<i32>> = vec![vec![]];
    let arg_1: i32 = -7;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<Vec<i32>> = vec![vec![0], vec![], vec![0, -1], vec![7, 0, 1, -1, 3, -1, 10, -6, -1, 1, 4, -2, 0], vec![-11, 5], vec![0, 0, 0, -3, 1, -8, -3], vec![-1], vec![], vec![0, 0, -10, -4, 2, 1], vec![], vec![], vec![0, 2, -1, -5, -11, -5, 2], vec![], vec![-3, -4, 1], vec![]];
    let arg_1: i32 = 7;
    let expected: Vec<(usize, usize)> = vec![(3, 0)];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<Vec<i32>> = vec![vec![8, 1, -1, 9, 4, 4, 1, 3], vec![]];
    let arg_1: i32 = 17;
    let expected: Vec<(usize, usize)> = vec![];
    assert_eq!(get_row(arg_0, arg_1), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<Vec<i32>> = vec![vec![3, 2], vec![0, 12], vec![-7], vec![], vec![]];
    let arg_1: i32 = 2;
    let expected: Vec<(usize, usize)> = vec![(0, 1)];
    assert_eq!(get_row(arg_0, arg_1), expected);
}