// diff_21

/*
 * Given an int n, return the absolute difference between n and
 * 21, except return double the absolute difference if n is 
 * over 21.
 * 
 * diff21(19) → 2
 * diff21(10) → 11
 * diff21(21) → 0
 */

pub fn diff_21(n: i32) -> i32 {
    let val: i32 = n - 21;
    return val.abs();
}
