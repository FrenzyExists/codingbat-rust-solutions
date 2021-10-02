// makes_10.rs

/*
 * Given 2 ints, a and b, return true if one if them is 10 or if their
 * sum is 10.
 *
 * makes_10(9, 10) → true
 * makes_10(9, 9) → false
 * makes_10(1, 9) → true
 */

fn makes_10(a: i32, b: i32) -> bool {
    return (a == 10) || (b == 10) || (a+b == 10);
}



