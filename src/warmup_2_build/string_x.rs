/*
 * Given a string, return a version where
 * all the "x" have been removed. Except 
 * an "x" at the very start or end should
 * not be removed.
 *
 * stringX("xxHxix") → "xHix"
 * stringX("abxxxcd") → "abcd"
 * stringX("xabxxxcdx") → "xabcdx"
 *
*/

pub fn string_x(string: &str) -> String {
    if string.is_empty() { return "".to_string(); }
    let mut iter = string.chars().into_iter().peekable();
    let mut new_string = String::new();
    let mut pointer:Option<char> = iter.next();
    while pointer.ne(&None) {
        if iter.peek().eq(&None) {
            new_string.push(pointer.unwrap());
            break;
        }
        if pointer.unwrap().eq(&'x') && iter.peek().unwrap().eq(&'x') {
            pointer.replace(iter.next().unwrap());
            continue;
        }
        new_string.push(pointer.unwrap());
        pointer.replace(iter.next().unwrap());
    }
    return new_string;
}
