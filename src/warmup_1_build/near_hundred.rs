// near_hundred.rs

/*
 * Given an int n, return true if it is within 10 of 100 or 200. 
 * Note: Math.abs(num) computes the absolute value of a number.
 *
 * nearHundred(93) → true
 * nearHundred(90) → true
 * nearHundred(89) → false
 */

pub fn near_hundred(n: i32) -> bool {
    return ( (100 - n).abs() < 10 ) || ( (200 - n).abs() < 10 );
}
