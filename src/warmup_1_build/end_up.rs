
/*
 * Given a string, return a new string where the last 3 chars are now in upper case. If the string has less than 3 chars, uppercase whatever is there. Note that str.toUpperCase() returns the uppercase version of a string.

endUp("Hello") → "HeLLO"
endUp("hi there") → "hi thERE"
endUp("hi") → "HI"
*/

pub fn end_up(string: &str) -> String {
    let mut new_string: String = "".to_string();
    let mut white_counter = String::from(string).chars().filter(|c| !c.is_whitespace()).count();

    if string.len() < 2 {
        return string.clone().to_uppercase();
    }

    for j in string.split_whitespace() {
        new_string.push_str(&j.chars().take(2).collect::<String>());
        new_string.push_str(&j.chars().skip(2).collect::<String>().to_uppercase());
        if white_counter > 0 {
            new_string.push(' ');
            white_counter -= 1;
        }
    }

    return new_string;
}

/*
pub fn end_up_II(string: &str) -> String {
  let mut output = String::with_capacity(string.len());
  let whole_len = string.len();
  let prefix_len = whole_len - 3;
  output.push_str(&string[0..prefix_len]);
  let bytes = string.bytes();
  for i in prefix_len..whole_len {
    bytes.collect::<String>();
   // output.push(bytes[i].to_ascii_uppercase());
  }

  return output;
}
*/
