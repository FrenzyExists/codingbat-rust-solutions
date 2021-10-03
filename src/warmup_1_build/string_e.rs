// string_e

/*
 * Return true if the given string contains between 
 * 1 and 3 'e' chars.
 *
 * stringE("Hello") → true
 * stringE("Heelle") → true
 * stringE("Heelele") → false
 */

pub fn string_e(e: &str) -> bool {
    if e.len() == 0 {return false;}
    if e.len() == 1 && e != "e" {return false;}
    let mut counter: i32 = 0;
    for i in e.split("") {
        if i == "e" {counter +=1;}
    }
    return counter >=1 && counter <=3;
}
