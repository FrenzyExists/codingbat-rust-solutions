/*
 * Given 2 positive int values, return the 
 * larger value that is in the range 10..20
 * inclusive, or return 0 if neither is in
 * that range.
 *
 * max1020(11, 19) → 19
 * max1020(19, 11) → 19
 * max1020(11, 9) → 11
 * 
 */
use std::cmp::max;

pub fn max_1020(a: i32, b: i32) -> i32 {
    let range = 10..20; 
    if range.contains(&a) || range.contains(&b) {
        return max(a, b);
    }
    return 0;
}
