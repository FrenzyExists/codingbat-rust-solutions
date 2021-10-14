/**
 * Given a string, return a string made of 
 * the chars at indexes 0,1, 4,5, 8,9 ...
 * so "kittens" yields "kien".
 *
 * altPairs("kitten") → "kien"
 * altPairs("Chocolate") → "Chole"
 * altPairs("CodingHorror") → "Congrr"
 *
 */

// 1,2, 5, 6, 9, 10

// Pairs are like n, n+1; skip n+2 and  n+3; n+4
// which means that every n+2 and n+3 is skipped
pub fn alt_pairs(string: &str) -> String {
    let mut new_string: String = "".to_string();
    let _len = string.len();
    let mut _end = 0;
    for (j, i) in string.chars().enumerate().step_by(4) {
        _end = j+2;
        if _end > _len {
            _end = _len;
        }
        new_string.push_str(&string.chars().skip(j).take(_end).collect::<String>());
    }
    return new_string;
}
