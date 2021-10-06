// every_nth

/*
 * Given a non-empty string and an int N, return 
 * the string made starting with char 0, and then 
 * every Nth char of the string. So if N is 3, use
 * char 0, 3, 6, ... and so on. N is 1 or more.
 *
 * everyNth("Miracle", 2) → "Mrce"
 * everyNth("abcdefg", 2) → "aceg"
 * everyNth("abcdefg", 3) → "adg"
 */

pub fn every_nth(s: &str, n: usize) -> String {
    let mut new_string: String = "".to_string().to_owned();
    let length:usize = s.len();
    let mut _b = 0;
    if n%2 == 0 {
        _b = n - 1;
    } else {
        _b = n - 2;
    }
    for (i, j) in s.chars().enumerate() {
        if i%n < _b {
            new_string.push(j);
        }
    }
    return new_string;
}
