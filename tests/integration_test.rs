use codingbat_rust::warmup1::*;
use codingbat_rust::warmup2::*;
use std::collections::HashMap;

//==========//
// Warmup 1 //
//==========//

/// backAround
#[test]
fn p1() {
    let result = HashMap::from([
        ("cat", "tcatt"),
        ("Hello", "oHelloo"),
        ("a", "aaa"),
        ("abc", "cabcc"),
        ("read", "dreadd"),
        ("boo", "obooo"),
    ]);
    for (input, output) in &result {
        assert_eq!(back_around(input), output);
    }
}

/// close10
#[test]
fn p2() {
    let result = HashMap::from([([8, 13], 8)]);
}

/// delDel
#[test]
fn p3() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

/// diff21
#[test]
fn p4() {
    let result = HashMap::from([
        (19, 2),
        (10, 11),
        (21, 0),
        (22, 2),
        (25, 8),
        (30, 18),
        (0, 21),
        (1, 20),
        (2, 19),
        (-1, 22),
        (-2, 23),
        (50, 58),
    ]);
    for (input, output) in &result {
        assert_eq!(diff_21(*input), output);
    }
}

/// endUp
#[test]
fn p5() {
    let result = HashMap::from([
        ("Hello", "HeLLO"),
        ("hi there", "hi thERE"),
        ("woo hoo", "woo HOO"),
        ("xyz12", "woo HOO"),
        ("x", "X"),
        ("", ""),
    ]);
    for (input, output) in &result {
        assert_eq!(end_up(input), output);
    }
}

/// Every Nth
#[test]
fn p6() {
    let result = HashMap::from([
        (("Miracle", 2), "Mrce"),
        (("abcdefg", 2), "aceg"),
        (("abcdefg", 3), "adg"),
        (("Chocolate", 3), "Cca"),
        (("Chocolates", 3), "Ccas"),
        (("Chocolates", 4), "Coe"),
        (("Chocolates", 100), "C"),
    ]);
    for (input, output) in &result {
        assert_eq!(every_nth(input.0, input.1), output.to_string());
    }
}

/// front22
#[test]
fn p7() {
    let result = HashMap::from([
        ("kitten", "kikittenki"),
        ("Ha", "HaHaHa"),
        ("abc", "ababcab"),
        ("ab", "ababab"),
        ("a", "aaa"),
        ("", ""),
        ("Logic", "LoLogicLo"),
    ]);
    for (input, output) in &result {
        assert_eq!(front_22(input), output);
    }
}

/// front3
#[test]
fn p8() {
    let result = HashMap::from([
        ("Java", "JavJavJav"),
        ("Chocolate", "ChoChoCho"),
        ("abc", "abcabcabc"),
        ("abcXYZ", "abcabcabc"),
        ("ab", "ababab"),
        ("a", "aaa"),
        ("", ""),
    ]);
    for (input, output) in &result {
        assert_eq!(front_3(input), output);
    }
}

/// frontBack
#[test]
fn p9() {
    let result = HashMap::from([
        ("code", "eodc"),
        ("a", "a"),
        ("ab", "ba"),
        ("abc", "cba"),
        ("", ""),
        ("Chocolate", "ehocolatC"),
        ("aavJ", "Java"),
        ("Hello", "oellH"),
    ]);
    for (input, output) in &result {
        assert_eq!(front_back(input), output);
    }
}

/// hasTeen
#[test]
fn p10() {
    let result = HashMap::from([
        ([13, 20, 10], true),
        ([20, 19, 10], true),
        ([20, 10, 13], true),
        ([1, 20, 12], false),
        ([19, 20, 12], true),
        ([12, 20, 19], true),
        ([12, 9, 20], false),
        ([12, 18, 20], true),
        ([14, 2, 20], true),
        ([4, 2, 20], false),
        ([11, 22, 22], false),
    ]);
    for (input, output) in &result {
        assert_eq!(has_teen(input[0], input[1], input[2]), output);
    }
}

/// icyHot
#[test]
fn p11() {
    let result = HashMap::from([
        ([120, -1], true),
        ([-1, 120], true),
        ([2, 120], false),
        ([-1, 100], false),
        ([-2, -2], false),
        ([120, 120], false),
    ]);
    for (input, output) in &result {
        assert_eq!(icy_hot(input[0], input[1]), output);
    }
}

/// in1020
#[test]
fn p12() {
    let result = HashMap::from([
        ([12, 99], true),
        ([21, 12], true),
        ([8, 99], false),
        ([99, 10], true),
        ([20, 20], true),
        ([21, 21], false),
        ([9, 9], false),
    ]);
    for (input, output) in &result {
        assert_eq!(in_1020(input[0], input[1]), output);
    }
}

/// in3050
#[test]
fn p13() {
    let result = HashMap::from([
        ([30, 31], true),
        ([30, 41], false),
        ([40, 50], true),
        ([40, 51], false),
        ([39, 50], false),
        ([50, 39], false),
        ([40, 39], true),
        ([49, 48], true),
        ([50, 40], true),
        ([50, 51], false),
        ([35, 36], true),
        ([35, 45], false),
    ]);
    for (input, output) in &result {
        assert_eq!(in_3050(input[0], input[1]), output);
    }
}

// intMax
#[test]
fn p14() {
    let result = HashMap::from([
        ([1, 2, 3], 3),
        ([1, 3, 2], 3),
        ([3, 2, 1], 3),
        ([9, 3, 3], 9),
        ([3, 9, 3], 9),
        ([8, 2, 3], 8),
        ([-3, -1, -2], -1),
        ([6, 2, 5], 6),
        ([5, 6, 2], 6),
        ([5, 2, 6], 6),
    ]);
    for (input, output) in &result {
        assert_eq!(int_max(input[0], input[1], input[2]), output);
    }

}

// lastDigit

// makes10

// max1020
// missingChar
// mixStart
// monkeyTrouble
// nearHundred
// notString
// or35
// parrotTrouble
// posNeg
// sleepIn
// startHi
// startOz
// stringE
// sumDouble
