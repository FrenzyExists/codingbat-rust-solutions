//! impl warmup_2 {}

//==========//
// Warmup 2 //
//==========//

/// Given a string, return a string made of
/// the chars at indexes 0,1, 4,5, 8,9 ...
/// so "kittens" yields "kien".
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// altPairs("kitten") → "kien"
/// altPairs("Chocolate") → "Chole"
/// altPairs("CodingHorror") → "Congrr"
/// ```
///
/// Pairs are like n, n+1; skip n+2 and  n+3; n+4
/// which means that every n+2 and n+3 is skipped
pub fn alt_pairs(string: &str) -> String {
    let mut _chars = string.chars();
    let len = string.len();
    let mut n = 1;
    loop {
        if len % n != 0 {
            break;
        }
        n += 1;
    }
    let mut new_string: String = String::with_capacity(n);
    for _i in 0..n {
        new_string.push_str(&*_chars.next().iter().collect::<String>());
        new_string.push_str(&*_chars.next().iter().collect::<String>());
        _chars.next();
        _chars.next();
    }
    return new_string;
}

/// Given an array of ints, return true if
/// the sequence of numbers 1, 2, 3 appears
/// in the array somewhere.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// array123([1, 1, 2, 3, 1]) → true
/// array123([1, 1, 2, 4, 1]) → false
/// array123([1, 1, 2, 1, 2, 3]) → true
/// ```
pub fn array_123(nums: &mut [i32]) -> bool {
    if nums.is_empty() {
        return nums.is_empty();
    }
    for i in 0..nums.len() - 2 {
        if nums[i..i + 3].eq(&[1, 2, 3]) {
            return true;
        }
    }
    return false;
}

/// Given an array of ints, return the
/// number of times that two 6's are
/// next to each other in the array.
/// Also count instances where the
/// second "6" is actually a 7.
///
/// ```
/// array667([6, 6, 2]) → 1
/// array667([6, 6, 2, 6]) → 1
/// array667([6, 7, 2, 6]) → 1
/// ```
pub fn array_667(nums: &mut [i32]) -> i32 {
    let mut counter: i32 = 0;
    let mut temp: i32 = 0;
    for i in nums.iter() {
        if temp == 6 && (i.eq(&6) || i.eq(&7)) {
            counter += 1;
        }
        temp = *i;
    }
    return counter;
}

//arrayCount9
/// Given an array of ints, return the
/// number of 9's in the array.
///
/// arrayCount9([1, 2, 9]) → 1
/// arrayCount9([1, 9, 9]) → 2
/// arrayCount9([1, 9, 9, 3, 9]) → 3
pub fn array_count_9() {}

//arrayFront9

//countXX
/// Count the number of "xx" in the given string.
/// We'll say that overlapping is allowed, so "xxx"
/// contains 2 "xx".
///
/// ```
/// countXX("abcxx") → 1
/// countXX("xxx") → 2
/// countXX("xxxx") → 3
/// ```
pub fn count_xx() {}

/// Given a string, return true if the first instance
/// of "x" in the string is immediately followed by
/// another "x".
///
/// doubleX("axxbb") → true
/// doubleX("axaxax") → false
/// doubleX("xxxxx") → true
pub fn double_x(string: &str) -> bool {
    return false;
}

/// Given a string and a non-negative int n,
/// we'll say that the front of the string
/// is the first 3 chars, or whatever is
/// there if the string is less than
/// length 3. Return n copies of the front;
///
/// frontTimes("Chocolate", 2) → "ChoCho"
/// frontTimes("Chocolate", 3) → "ChoChoCho"
/// frontTimes("Abc", 3) → "AbcAbcAbc
///
pub fn front_times(string: &str, n: i32) -> String {
    return "".to_string();
}

//has271
//last2
//noTriples
//stringBits

//stringMatch

/// Given a non-empty string like "Code" 
/// return a string like "CCoCodCode".
///
/// ```
/// stringSplosion("Code") → "CCoCodCode"
/// stringSplosion("abc") → "aababc"
/// stringSplosion("ab") → "aab"
///```
pub fn string_splosion() {

}

/// Given a string and a non-negative int n,
/// return a larger string that is n copies
/// of the original string.
///
/// stringTimes("Hi", 2) → "HiHi"
/// stringTimes("Hi", 3) → "HiHiHi"
/// stringTimes("Hi", 1) → "Hi"
pub fn string_times(string: &str, n: i32) -> String {
    return "".to_string();
}

/// Given a string, return a version where
/// all the "x" have been removed. Except
/// an "x" at the very start or end should
/// not be removed.
///
/// stringX("xxHxix") → "xHix"
/// stringX("abxxxcd") → "abcd"
/// stringX("xabxxxcdx") → "xabcdx"
pub fn string_x(string: &str) -> String {
    if string.is_empty() {
        return "".to_string();
    }
    let mut iter = string.chars().into_iter().peekable();
    let mut new_string = String::new();
    let mut pointer: Option<char> = iter.next();
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

/// Suppose the string "yak" is unlucky. Given a
/// string, return a version where all the "yak"
/// are removed, but the "a" can be any char. The
/// "yak" strings will not overlap.
///
/// ```
/// stringYak("yakpak") → "pak"
/// stringYak("pakyak") → "pak"
/// stringYak("yak123ya") → "123ya"
/// ```
pub fn string_yak(string: &str) -> String {
    let mut rev_str = string.to_string().clone().chars().rev().collect::<String>();
    rev_str.push_str(&string);
    return rev_str;
}
