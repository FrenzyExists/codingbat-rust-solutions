
/*
 *Given two non-negative int values, return true if they have the same last digit, such as with 27 and 57. Note that the % "mod" operator computes remainders, so 17 % 10 is 7.

lastDigit(7, 17) → true
lastDigit(6, 17) → false
lastDigit(3, 113) → true
 */

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
