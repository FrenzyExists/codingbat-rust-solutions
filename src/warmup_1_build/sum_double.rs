// sum_double.rs
/*
 * Given two int values, return their sum. Unless the two 
 * values are the same, then return double their sum.
 *
 * sumDouble(1, 2) → 3
 * sumDouble(3, 2) → 5
 * sumDouble(2, 2) → 8
 */

pub fn sum_double(a: i32, b: i32) -> i32 {
    return if a == b { (a + b) * 2 } else { a + b };
}
