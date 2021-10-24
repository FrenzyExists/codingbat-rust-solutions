/*
 * Difficulty: 242.0
 *
 * Given an array of ints, we'll say that 
 * a triple is a value appearing 3 times 
 * in a row in the array. Return true if 
 * the array does not contain any triples.
 *
 * noTriples([1, 1, 2, 2, 1]) → true
 * noTriples([1, 1, 2, 2, 2, 1]) → false
 * noTriples([1, 1, 1, 2, 2, 2, 1]) → false
 *
 */
pub fn no_triples(nums: &mut [i32]) -> i32 {
    let len = nums.len();
    let mut temp = 0;
    for _i in nums.iter() {
        temp = *_i;

    }
    return 0;
}
