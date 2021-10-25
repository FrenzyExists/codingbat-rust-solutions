/**
 * Suppose the string "yak" is unlucky. Given a 
 * string, return a version where all the "yak" 
 * are removed, but the "a" can be any char. The 
 * "yak" strings will not overlap.
 * 
 * stringYak("yakpak") → "pak"
 * stringYak("pakyak") → "pak"
 * stringYak("yak123ya") → "123ya"
 * 
 */
pub fn string_yak(string: &str) -> String {
    let mut rev_str = string.to_string().clone().chars().rev().collect::<String>();
    rev_str.push_str(&string);
    return rev_str;

}

