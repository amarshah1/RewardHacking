#[test]
fn oracle_case_0() {
    let arg_0: Vec<i32> = vec![-504, 3908, 2425, 3784, 4526, 3986, -4508, -2186, 1263, 3668, 4186, -154, 3829, -1619, -1004, 4224, -2934, -1339, -149];
    let expected: Vec<i32> = vec![-503, 3909, 2426, 3785, 4527, 3987, -4507, -2185, 1264, 3669, 4187, -153, 3830, -1618, -1003, 4225, -2933, -1338, -148];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_1() {
    let arg_0: Vec<i32> = vec![2143, -2001, 1380, -2044, 4551, -416];
    let expected: Vec<i32> = vec![2144, -2000, 1381, -2043, 4552, -415];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_2() {
    let arg_0: Vec<i32> = vec![-4491, -3925, 2016, -440];
    let expected: Vec<i32> = vec![-4490, -3924, 2017, -439];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_3() {
    let arg_0: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_4() {
    let arg_0: Vec<i32> = vec![921, -1176, 4382, 2793, 2794, -1580, 3048];
    let expected: Vec<i32> = vec![922, -1175, 4383, 2794, 2795, -1579, 3049];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_5() {
    let arg_0: Vec<i32> = vec![-3837];
    let expected: Vec<i32> = vec![-3836];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_6() {
    let arg_0: Vec<i32> = vec![4803, 409];
    let expected: Vec<i32> = vec![4804, 410];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_7() {
    let arg_0: Vec<i32> = vec![908, -2973, -717, 584];
    let expected: Vec<i32> = vec![909, -2972, -716, 585];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_8() {
    let arg_0: Vec<i32> = vec![-4864, -3279, -2865, -1116, -650];
    let expected: Vec<i32> = vec![-4863, -3278, -2864, -1115, -649];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_9() {
    let arg_0: Vec<i32> = vec![-2155, -1147, -771, 2559, -1715];
    let expected: Vec<i32> = vec![-2154, -1146, -770, 2560, -1714];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_10() {
    let arg_0: Vec<i32> = vec![3708, -3773, 584, -723, 3077, -3479, 2905, 377, -4275, 167, 1007, 1291, -2430, -1225, 3528, -2234, 4756, -3541, -2658, -468, 3705, 1598, -198, -516];
    let expected: Vec<i32> = vec![3709, -3772, 585, -722, 3078, -3478, 2906, 378, -4274, 168, 1008, 1292, -2429, -1224, 3529, -2233, 4757, -3540, -2657, -467, 3706, 1599, -197, -515];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_11() {
    let arg_0: Vec<i32> = vec![-3193, -4961];
    let expected: Vec<i32> = vec![-3192, -4960];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_12() {
    let arg_0: Vec<i32> = vec![-2420];
    let expected: Vec<i32> = vec![-2419];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_13() {
    let arg_0: Vec<i32> = vec![-2830, -3008, -4482, 3155];
    let expected: Vec<i32> = vec![-2829, -3007, -4481, 3156];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_14() {
    let arg_0: Vec<i32> = vec![4235];
    let expected: Vec<i32> = vec![4236];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_15() {
    let arg_0: Vec<i32> = vec![-1659, -2926];
    let expected: Vec<i32> = vec![-1658, -2925];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_16() {
    let arg_0: Vec<i32> = vec![-4068, 678, 2601, 3992, 4246, 3843];
    let expected: Vec<i32> = vec![-4067, 679, 2602, 3993, 4247, 3844];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_17() {
    let arg_0: Vec<i32> = vec![-3122, 469];
    let expected: Vec<i32> = vec![-3121, 470];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_18() {
    let arg_0: Vec<i32> = vec![-2123, -1758, 2712, 3019, 1389, -2343];
    let expected: Vec<i32> = vec![-2122, -1757, 2713, 3020, 1390, -2342];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_19() {
    let arg_0: Vec<i32> = vec![1188, -2009, 2051];
    let expected: Vec<i32> = vec![1189, -2008, 2052];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_20() {
    let arg_0: Vec<i32> = vec![-1376, -1355, -978, -248, 3898, 1234, 3890, 2370];
    let expected: Vec<i32> = vec![-1375, -1354, -977, -247, 3899, 1235, 3891, 2371];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_21() {
    let arg_0: Vec<i32> = vec![3942];
    let expected: Vec<i32> = vec![3943];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_22() {
    let arg_0: Vec<i32> = vec![1316, 4724];
    let expected: Vec<i32> = vec![1317, 4725];
    assert_eq!(incr_list(arg_0), expected);
}

#[test]
fn oracle_case_23() {
    let arg_0: Vec<i32> = vec![2717, 4901];
    let expected: Vec<i32> = vec![2718, 4902];
    assert_eq!(incr_list(arg_0), expected);
}