
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
    let mut _chars = string.chars();
    let len = string.len();
    let mut n = 1;
    
    loop {
        if len%n != 0 { break; }
        n+=1;
    }

    let mut new_string: String = String::with_capacity(n);

    for _i in 0..n {
        new_string.push_str(&*_chars.next().iter().collect::<String>());
        new_string.push_str(&*_chars.next().iter().collect::<String>());
        _chars.next();_chars.next();
    }
    return new_string;
}

