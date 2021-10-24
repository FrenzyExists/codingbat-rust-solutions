
/*
 *
 * Given a string, return a string made of 
 * the chars at indexes 0,1, 4,5, 8,9 ...
 * so "kittens" yields "kien".
 *
 * altPairs("kitten") → "kien"
 * altPairs("Chocolate") → "Chole"
 * altPairs("CodingHorror") → "Congrr"
 *
 * Pairs are like n, n+1; skip n+2 and  n+3; n+4
 * which means that every n+2 and n+3 is skipped
 */

pub fn alt_pairs(string: &str) -> String {
    let mut new_string: String = "".to_string();
    let mut _chars = string.chars();
    let len = string.len();
    let mut n = 1;
    
    loop {
        if len%n != 0 { break; }
        n+=1;
    }

    for _i in 0..n {
        new_string.push_str(&*_chars.next().iter().collect::<String>());
        new_string.push_str(&*_chars.next().iter().collect::<String>());
        _chars.next();_chars.next();
    }
    return new_string;
}

pub fn alt_pairs_2(string: &str) -> String {
    let mut new_string: String = "".to_string();
    let _len = string.len();
    let mut _end = 0;
    for (j, _i) in string.chars().enumerate().step_by(4) {
        _end = j+2;
        if _end > _len {
            _end = _len;
        }
        new_string.push_str(&string.chars().skip(j).take(_end).collect::<String>());
    }
    return new_string;
}
