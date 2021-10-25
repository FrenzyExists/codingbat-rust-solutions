/*
 *
 * Given a string, return true if the first instance
 * of "x" in the string is immediately followed by 
 * another "x".

doubleX("axxbb") → true
doubleX("axaxax") → false
doubleX("xxxxx") → true

pub fn double_x(string: &str) -> bool { 
    let mut str_arr = string.chars();
    let mut temp = &str_arr.next();
    for i in str_arr {
        if temp.eq(&Some(i)) {return true;}
        temp = &str_arr.next();
    }
    return false;
}

*/
