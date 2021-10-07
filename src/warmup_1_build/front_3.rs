/*
 * Given a string, we'll say that the front is the first
 * 3 chars of the string. If the string length is less
 * than 3, the front is whatever is there. Return a
 * new string which is 3 copies of the front.
 *
 * front3("Java") → "JavJavJav"
 * front3("Chocolate") → "ChoChoCho"
 * front3("abc") → "abcabcabc"
 */
pub fn front_3(string: &str) -> String {
    let mut new_string: String = "".to_string();
    let boi = &string.chars().take(3).collect::<String>();
    for _i in 0..3 { new_string.push_str(boi); }
    return new_string;
}
