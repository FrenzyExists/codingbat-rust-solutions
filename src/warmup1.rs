//! impl warmup_1 {}

use std::cmp::max;

//==========//
// Warmup 1 //
//==========//

/// Given a string, return a new string where the
/// last 3 chars are now in upper case. If the
/// string has less than 3 chars, uppercase whatever
/// is there.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// end_up("Hello"); → "HeLLO"
/// end_up("hi there"); → "hi thERE"
/// end_up("hi"); → "HI"
/// ```
pub fn end_up(string: &str) -> String {
    let mut new_string: String = "".to_string();
    let mut white_counter = String::from(string)
        .chars()
        .filter(|c| !c.is_whitespace())
        .count();

    if string.len() < 2 {
        return string.clone().to_uppercase();
    }

    for j in string.split_whitespace() {
        new_string.push_str(&j.chars().take(2).collect::<String>());
        new_string.push_str(&j.chars().skip(2).collect::<String>().to_uppercase());
        if white_counter > 0 {
            new_string.push(' ');
            white_counter -= 1;
        }
    }

    return new_string;
}

/// Given two int values, return their sum. Unless
/// the two values are the same, then return double
/// their sum.
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// sumDouble(1, 2); → 3
/// sumDouble(3, 2); → 5
/// sumDouble(2, 2); → 8
/// ```
pub fn sum_double(a: i32, b: i32) -> i32 {
    return if a == b { (a + b) * 2 } else { a + b };
}

/// Return true if the given string contains between
/// 1 and 3 'e' chars.
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// stringE("Hello") → true
/// stringE("Heelle") → true
/// stringE("Heelele") → false
/// ```
pub fn string_e(e: &str) -> bool {
    if e.len() == 0 {
        return false;
    }
    if e.len() == 1 && e != "e" {
        return false;
    }
    let mut counter: i32 = 0;
    for i in e.split("") {
        if i == "e" {
            counter += 1;
        }
    }
    return counter >= 1 && counter <= 3;
}

/// Given a string, take the first 2
/// chars and return the string with the
/// 2 chars added at both the front and
/// back, so "kitten" yields"kikittenki". If
/// the string length is less than 2, use
/// whatever chars are there.
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// front22("kitten") → "kikittenki"
/// front22("Ha") → "HaHaHa"
/// front22("abc") → "ababcab"
/// ```
pub fn front_22(string: &str) -> String {
    let mut new_string: String = "".to_string();

    new_string.push_str(&string.chars().take(2).collect::<String>());
    new_string.push_str(&string);
    new_string.push_str(&string.chars().take(2).collect::<String>());

    return new_string;
}

/// The parameter weekday is true if it is a
/// weekday, and the parameter vacation is true
/// if we are on vacation. We sleep in if it is
/// not a weekday or we're on vacation. Return
/// true if we sleep in.
///
/// # Example
///
/// Basic Usage:
///
/// ```
///
/// ```
pub fn sleep_in(weekday: bool, vacation: bool) -> bool {
    return !weekday || vacation;
}

/// Given 2 int values, return true if one is
/// negative and one is positive. Except if the
/// parameter "negative" is true, then return
/// true only if both are negative
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// posNeg(1, -1, false); → true
/// posNeg(-1, 1, false); → true
/// posNeg(-4, -5, true); → true
/// ```
pub fn pos_neg(a: i32, b: i32, negative: bool) -> bool {
    if negative {
        return a < 0 && b < 0;
    }
    return (a < 0 && b > 0) || (a > 0 && b < 0);
}

/// We have a loud talking parrot. The "hour"
/// parameter is the current hour time in the
/// range 0..23. We are in trouble if the parrot
/// is talking and the hour is before 7 or after
/// 20. Return true if we are in trouble.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// parrotTrouble(true, 6) → true
/// parrotTrouble(true, 7) → false
/// parrotTrouble(false, 6) → false
/// ```
pub fn parrot_trouble(talking: bool, hour: i32) -> bool {
    return (hour < 7 || hour > 20) && talking;
}

/// Given a string, return a new string where "not"
/// has been added to the front. However, if the
/// string already begins with "not", return the string
/// unchanged. Note: use .equals() to compare 2 strings.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// notString("candy"); → "not candy"
/// ```
pub fn not_string(string: &str) -> String {
    for s in string.split_whitespace() {
        if s == "not" {
            return string.to_string();
        }
    }
    let mut new_string: String = "not ".to_owned();
    new_string.push_str(&string);
    return new_string;
}

/// Given an int n, return true if it is within 10 of 100 or 200.
/// Note: Math.abs(num) computes the absolute value of a number.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// nearHundred(93); → true
/// nearHundred(90); → true
/// nearHundred(89); → false
/// ```
pub fn near_hundred(n: i32) -> bool {
    return ((100 - n).abs() < 10) || ((200 - n).abs() < 10);
}

/// We have two monkeys, a and b, and the parameters
/// aSmile and bSmile indicate if each is smiling. We
/// are in trouble if they are both smiling or if neither
///  of them is smiling. Return true if we are in trouble.
///
/// TL;DR This is an inverted XOR gate, sort of
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// monkeyTrouble(true, true) → true
/// monkeyTrouble(false, false) → true
/// monkeyTrouble(true, false) → false
/// ```
pub fn monkey_trouble(a_smile: bool, b_smile: bool) -> bool {
    return !((a_smile && !b_smile) || (!a_smile && b_smile));
}

/// Given a non-empty string and an int n, return a new
/// string where the char at index n has been removed.
/// The value of n will be a valid index of a char in
/// the original string (i.e. n will be in the range
/// 0..str.length()-1 inclusive).
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// missing_char("kitten", 1) → "ktten"
/// missing_char("kitten", 0) → "itten"
/// missing_char("kitten", 4) → "kittn
/// ```
pub fn missing_char(string: &str, n: usize) -> String {
    let mut new_string: String = String::from(string);
    new_string.remove(n);
    return new_string;
}

/// Given 2 positive int values, return the larger
/// value that is in the range 10..20 inclusive, or
///  return 0 if neither is in that range.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// max1020(11, 19); → 19
/// max1020(19, 11); → 19
/// max1020(11, 9); → 11
/// ```
pub fn max_1020(a: i32, b: i32) -> i32 {
    let range = 10..20;
    if range.contains(&a) || range.contains(&b) {
        return max(a, b);
    }
    return 0;
}

/// Given 2 ints, a and b, return true if one if
/// them is 10 or if their sum is 10.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// makes_10(9, 10) → true
/// makes_10(9, 9) → false
/// makes_10(1, 9) → true
/// ```
pub fn makes_10(a: i32, b: i32) -> bool {
    return (a == 10) || (b == 10) || (a + b == 10);
}

/// Given two non-negative int values, return true
///  if they have the same last digit, such as
/// with 27 and 57. Note that the % "mod" operator
/// computes remainders, so 17 % 10 is 7.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// lastDigit(7, 17) → true
/// lastDigit(6, 17) → false
/// lastDigit(3, 113) → true
/// ```
pub fn last_digit(a: i32, b: i32) -> bool {
    let mut _a = a;
    let mut _b = b;
    while _a > 10 || _b > 10 {
        if _a >= 10 {
            _a = _a % 10;
        }
        if _b >= 10 {
            _b = _b % 10;
        }
    }
    return _a == _b;
}

/// Given an int n, return the absolute difference
/// between n and 21, except return double the absolute
/// difference if n is over 21.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// diff21(19) → 2
/// diff21(10) → 11
/// diff21(21) → 0
/// ```
pub fn diff_21(n: i32) -> i32 {
    let val: i32 = n - 21;
    return val.abs();
}

/// Given a string, return a new string where the
/// first and last chars have been exchanged.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// frontBack("code") → "eodc"
/// frontBack("a") → "a"
/// frontBack("ab") → "ba"
/// ```
pub fn front_back(string: &str) -> String {
    return string.chars().rev().collect::<String>();
}

/// Given a string, we'll say that the front is the first
/// 3 chars of the string. If the string length is less
/// than 3, the front is whatever is there. Return a
/// new string which is 3 copies of the front.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// front3("Java") → "JavJavJav"
/// front3("Chocolate") → "ChoChoCho"
/// front3("abc") → "abcabcabc"
/// ```
pub fn front_3(string: &str) -> String {
    let mut new_string: String = "".to_string();
    let boi = &string.chars().take(3).collect::<String>();
    for _i in 0..3 {
        new_string.push_str(boi);
    }
    return new_string;
}

/// Given a string, take the last char and return a new 
/// string with the last char added at the front and 
/// back, so "cat" yields "tcatt". The original string 
/// will be length 1 or more.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// backAround("cat") → "tcatt"
/// backAround("Hello") → "oHelloo"
/// backAround("a") → "aaa"
/// ```
pub fn back_around(string: &str) -> String {
    return "".to_string();
}

/// Given 2 int values, return whichever value is nearest 
/// to the value 10, or return 0 in the event of a tie. 
/// Note that Math.abs(n) returns the absolute value of 
/// a number.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// close10(8, 13) → 8
/// close10(13, 8) → 8
/// close10(13, 7) → 0
/// ```
pub fn close_10(n: i32) -> i32 {
    return 0;
}

/// Given a string, if the string "del" appears starting 
/// at index 1, return a string where that "del" has been
///  deleted. Otherwise, return the string unchanged.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// delDel("adelbc") → "abc"
/// delDel("adelHello") → "aHello"
/// delDel("adedbc") → "adedbc"
/// ```
pub fn del_del(string: &str) -> String {
    return "".to_string();
}

/// Given a non-empty string and an int N, return 
/// the string made starting with char 0, and then 
/// every Nth char of the string. So if N is 3, use
/// char 0, 3, 6, ... and so on. N is 1 or more.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// every_nth("Miracle", 2) → "Mrce"
/// every_nth("abcdefg", 2) → "aceg"
/// every_nth("abcdefg", 3) → "adg"
/// ```
pub fn every_nth(s: &str, n: usize) -> String {
    let mut new_string: String = "".to_string().to_owned();
    let mut _b = 0;
    if n%2 == 0 {
        _b = n - 1;
    } else {
        _b = n - 2;
    }
    for (i, j) in s.chars().enumerate() {
        if i%n < _b {
            new_string.push(j);
        }
    }
    return new_string;
}

/// We'll say that a number is "teen" if it is in the 
/// range 13..19 inclusive. Given 3 int values, return
///  true if 1 or more of them are teen.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// hasTeen(13, 20, 10) → true
/// hasTeen(20, 19, 10) → true
/// hasTeen(20, 10, 13) → true
/// ```
pub fn has_teen() {}

/// Given two temperatures, return true if one is less 
/// than 0 and the other is greater than 100.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// icyHot(120, -1) → true
/// icyHot(-1, 120) → true
/// icyHot(2, 120) → false
/// ```
pub fn icy_hot() {}

/// Given 2 int values, return true if either of them 
/// is in the range 10..20 inclusive.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// in1020(12, 99) → true
/// in1020(21, 12) → true
/// in1020(8, 99) → false
/// ```
pub fn in_1020(a: i32, b: i32) -> bool {
    return false;
}

/// Given 2 int values, return true if they are both 
/// in the range 30..40 inclusive, or they are both 
/// in the range 40..50 inclusive.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// in3050(30, 31) → true
/// in3050(30, 41) → false
/// in3050(40, 50) → true
/// ```
pub fn in_3050(a: i32, b:i32) -> bool {
    return false;
}

/// Given three int values, a b c, return the largest.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// intMax(1, 2, 3) → 3
/// intMax(1, 3, 2) → 3
/// intMax(3, 2, 1) → 3
/// ```
pub fn int_max(a: i32, b: i32, c: i32) -> i32 {
    return 0;
}

/// Return true if the given non-negative number is a 
/// multiple of 3 or a multiple of 5. Use the % "mod" 
/// operator -- see 
/// [Introduction to Mod](https://codingbat.com/doc/practice/mod-introduction.html)
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// or35(3) → true
/// or35(10) → true
/// or35(8) → false
/// ```
pub fn or_35(n: i32) -> bool {
    return false;
}

/// Given a string, return a string made of the first 2 
/// chars (if present), however include first char only 
/// if it is 'o' and include the second only if it is 'z',
///  so "ozymandias" yields "oz".
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// startOz("ozymandias") → "oz"
/// startOz("bzoo") → "z"
/// startOz("oxx") → "o"
/// ```
pub fn start_oz() {}

/// Return true if the given string begins with "mix", 
/// except the 'm' can be anything, so "pix", "9ix" .. 
/// all count.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// mixStart("mix snacks") → true
/// mixStart("pix snacks") → true
/// mixStart("piz snacks") → false
/// ```
pub fn mix_start() {}

/// Given a string, return true if the string starts with
/// "hi" and false otherwise.
/// 
/// # Example
///
/// Basic Usage:
///
/// ```
/// startHi("hi there") → true
/// startHi("hi") → true
/// startHi("hello hi") → false
/// ```
pub fn start_hi(string: &str) -> bool {
    return false;
}