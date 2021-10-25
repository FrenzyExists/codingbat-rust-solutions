/*
 * Given an array of ints, return true 
 * if it contains a 2, 7, 1 pattern: a
 * value, followed by the value plus 5,
 * followed by the value minus 1.
 * Additionally the 271 counts even if 
 * the "1" differs by 2 or less from the
 * correct value.
 *
 * has271([1, 2, 7, 1]) → true
 * has271([1, 2, 8, 1]) → false
 * has271([2, 7, 1]) → true
 *
 */
pub fn has_271(nums: &mut [i32]) -> bool {
    if nums.len().le(&2) {return false;}
    let mut temp = nums[0];
    for _i in 1..nums.len() {
        //if nums[_i].eq(temp+5) && ((nums[_i + 1] - temp - 1) * i32: &1.pow(2)) <=2 {
        if nums[_i].eq(&(temp+5)) {
            return true;
        }
    }
    return false;
}
