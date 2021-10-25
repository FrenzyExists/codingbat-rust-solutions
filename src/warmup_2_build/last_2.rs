/*
 * Given a string, return the count of the
 * number of times that a substring length
 * 2 appears in the string and also as the 
 * last 2 chars of the string, so "hixxxhi"
 * yields 1 (we won't count the end substring).
 *
 * last2("hixxhi") → 1
 * last2("xaxxaxaxx") → 1
 * last2("axxxaaxx") → 2
 *
 */
pub fn last_2(string: &str) -> i32 {
    if string.len() <=3 {return 0;}
    let char_vec: Vec<char> = string.chars().collect();
    let last = char_vec.iter().skip(string.len() - 2).collect::<String>();
    let mut temp = String::with_capacity(2);
    let mut counter = 0;
    for i in 0..char_vec.len() - 3 {
        temp = (&char_vec[i..i+2]).iter().collect();
        if last == temp {
            counter += 1;
        }
    }
    return counter;
}
