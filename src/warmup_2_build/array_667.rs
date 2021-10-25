/*
 * Given an array of ints, return the
 * number of times that two 6's are 
 * next to each other in the array.
 * Also count instances where the 
 * second "6" is actually a 7.
 *
 * array667([6, 6, 2]) → 1
 * array667([6, 6, 2, 6]) → 1
 * array667([6, 7, 2, 6]) → 1
 *
 */
pub fn array_667(nums: &mut [i32]) -> i32 {
    let mut counter: i32 = 0;
    let mut temp: i32 = 0;
    for i in nums.iter() {
        if temp == 6 && (i.eq(&6) || i.eq(&7))  {counter += 1;}
        temp = *i;
    }
    return counter;
}
