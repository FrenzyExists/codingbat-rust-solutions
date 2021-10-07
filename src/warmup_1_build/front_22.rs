// front_22

/*
 * Given a string, take the first 2 
 * chars and return the string with the 
 * 2 chars added at both the front and
 * back, so "kitten" yields"kikittenki". If
 * the string length is less than 2, use
 * whatever chars are there.
 *
 * front22("kitten") → "kikittenki"
 * front22("Ha") → "HaHaHa"
 * front22("abc") → "ababcab"
*/

pub fn front_22(string: &str) -> String {
    let mut new_string: String = "".to_string();

    new_string.push_str(&string.chars().take(2).collect::<String>());
    new_string.push_str(&string);
    new_string.push_str(&string.chars().take(2).collect::<String>());

    return new_string;
}
