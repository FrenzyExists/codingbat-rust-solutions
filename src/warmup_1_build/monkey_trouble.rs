// monkey_trouble.rs

/*
 * We have two monkeys, a and b, and the parameters aSmile 
 * and bSmile indicate if each is smiling. We are in trouble 
 * if they are both smiling or if neither of them is smiling. 
 * Return true if we are in trouble.
 *
 * monkeyTrouble(true, true) → true
 * monkeyTrouble(false, false) → true
 * monkeyTrouble(true, false) → false
 */

pub fn monkey_trouble(a_smile: bool, b_smile: bool) -> bool {
    // TL;DR This is an inverted XOR gate, sort of
    return !( (a_smile && !b_smile) || (!a_smile && b_smile) );
}
