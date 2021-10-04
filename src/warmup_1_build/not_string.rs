// not_string.rs

/*
 * Given a string, return a new string where "not " has been added 
 * to the front. However, if the string already begins with "not", 
 * return the string unchanged. Note: use .equals() to compare 2 
 * strings.
 *
 * notString("candy") → "not candy"
 */


pub fn not_string(string: &str) -> String {
    for s in string.split_whitespace() {
        if s == "not" {
            return string.to_string();
        }
    }
    let mut new_string: String = "not ".to_owned();
    new_string.push_str(&string);
    return new_string;
}

