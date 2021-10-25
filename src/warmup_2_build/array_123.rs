/*
 * Given an array of ints, return true if 
 * the sequence of numbers 1, 2, 3 appears 
 * in the array somewhere.
 *
 * array123([1, 1, 2, 3, 1]) → true
 * array123([1, 1, 2, 4, 1]) → false
 * array123([1, 1, 2, 1, 2, 3]) → true
 *
 */

// Smart way
pub fn array_123(nums: &mut [i32]) -> bool {
    if nums.is_empty() {
        return nums.is_empty();
    }
    for i in 0..nums.len()-2 {
        if nums[i..i+3].eq(&[1, 2, 3]) {
            return true;
        }
    }
    return false;
}

// Dumb way
pub fn array_123_2(nums: &mut [i32]) -> bool {
    if nums.is_empty() {
        return nums.is_empty();
    }
    let mut iter = nums.iter().peekable();
    let mut pointer = iter.next().unwrap();
    let mut counter = 0;
    while iter.peek().ne(&None) {
        if (*iter.peek().unwrap() - *pointer).abs() == 1 {
            counter += 1;
        } else {
            counter = 0;
        }
        if counter == 2 {
            return true;
        }
        pointer = iter.next().unwrap();
    }
    return false;
}
