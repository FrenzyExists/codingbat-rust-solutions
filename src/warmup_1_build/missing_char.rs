// missing_char

/*
 * Given a non-empty string and an int n, 
 * return a new string where the char at index
 * n has been removed. The value of n will be
 * a valid index of a char in the original 
 * string (i.e. n will be in the range 
 * 0..str.length()-1 inclusive).
 *
 * missing_char("kitten", 1) → "ktten"
 * missing_char("kitten", 0) → "itten"
 * missing_char("kitten", 4) → "kittn
 */

pub fn missing_char(string: &str, n: usize) -> String {
    let mut new_string: String = String::from(string);
    new_string.remove(n);
    return new_string;
}
