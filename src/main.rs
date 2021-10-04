
// Hello World, aka the first program to be done

/*
 * Cargo new <project-name> - Creates a project
 * pass --bin for binary program; --lib as a libra
 * Cargo check - Checks code, does not create executable
 * Cargo build - Creates an executable and checks
 * Cargo run   - Creates an executable, checks and runs the executable
 * Cargo run --release - Creates an optimized executable at target/release
 */

mod warmup_1_build;
use crate::warmup_1_build::every_nth::every_nth;
use crate::warmup_1_build::diff_21::diff_21;
use crate::warmup_1_build::makes_10::makes_10;
use crate::warmup_1_build::sum_double::sum_double;
use crate::warmup_1_build::string_e::string_e;
use crate::warmup_1_build::sleep_in::sleep_in;
use crate::warmup_1_build::pos_neg::pos_neg;
use crate::warmup_1_build::parrot_trouble::parrot_trouble;
use crate::warmup_1_build::not_string::not_string;
use crate::warmup_1_build::near_hundred::near_hundred;
use crate::warmup_1_build::monkey_trouble::monkey_trouble;
use crate::warmup_1_build::front_22::front_22;
use crate::warmup_1_build::front_3::front_3;
use crate::warmup_1_build::last_digit::last_digit;
use crate::warmup_1_build::front_back::front_back;
use crate::warmup_1_build::max_1020::max_1020;
use crate::warmup_1_build::missing_char::missing_char;
use crate::warmup_1_build::end_up::end_up;

use rand::Rng;

fn main() {
    println!("Hello, world!");
    random_vec(4, [0, 10].to_vec());
    
    // Test 1
    warmup_1();
}

// warmup 1
fn warmup_1() {
    // Diff 21
    println!("Testing Diff 21...");
    println!("diff_21(19) -> {}", diff_21(19));
    println!("diff_21(10) -> {}", diff_21(10)); 
    println!("diff_21(21) -> {}", diff_21(21)); 
    
    println!("\nTesting every_nth..."); 
    println!("every_nth(\"Miracle\" 2) -> {}", every_nth("Miracle", 2));
    println!("every_nth(\"abcdefg\", 2) -> {}", every_nth("abcdefg", 3));

    // sum_double
    println!("\nTesting sum_double...");
    println!("sum_double(1, 2) -> {} ", sum_double(1, 2));
    println!("sum_double(3, 2) -> {} ", sum_double(3, 2));
    println!("sum_double(2, 2) -> {} ", sum_double(2, 2));

    // string_e
    println!("\nTesting string_e...");
    println!("string_e(\"Hello\") -> {} ", string_e("Hello"));
    println!("string_e(\"Heelle\") -> {} ", string_e("Heelle"));
    println!("string_e(\"Heelele\") -> {} ", string_e("Heelele"));

    // sleep_in
    println!("\nTesting sleep_in...");
    println!("sleep_in(weekday = false, vacation = false) -> {}", sleep_in(false, false));
    println!("sleep_in(weekday = true, vacation = false) -> {}", sleep_in(true, false));
    println!("sleep_in(weekday = false, vacation = true) -> {}", sleep_in(false, true));
    println!("sleep_in(weekday = true, vacation = true) -> {}", sleep_in(true, true));

    // pos_neg
    println!("\nTesting pos_neg...");
    println!("pos_neg(a = 1, b = -1, negative = false) -> {}", pos_neg(1, -1, false));
    println!("pos_neg(a = -1, b = 1, negative = false) -> {}", pos_neg(-1, 1, false));
    println!("pos_neg(a = -4, b = -5, negative = true) -> {}", pos_neg(-4, -5, true));

    // parrot_trouble
    println!("\nTesting parrot_trouble...");
    println!("parrot_trouble(talking = true, hour = 6) -> {}", parrot_trouble(true, 6));
    println!("parrot_trouble(talking = true, hour = 7) -> {}", parrot_trouble(true, 7));
    println!("parrot_trouble(talking = false, hour = 6) -> {}", parrot_trouble(false, 6));

    // not_string
    println!("\nTesting not_string...");
    println!("not_string(\"candy\") -> {}", not_string("candy"));

    // makes_10
    println!("\n Testing makes_10...");
    println!("makes_10(a = 9, b = 10) -> {}", makes_10(9, 10));
    println!("makes_10(n = 9, b = 9) -> {}", makes_10(9, 9));
    println!("makes_10(n = 1, b = 9) -> {}", makes_10(1, 9));

    // near_100
    println!("\nTesting near_100...");
    println!("near_100(93) -> {}", near_hundred(93));
    println!("near_100(90) -> {}", near_hundred(90));
    println!("near_100(89) -> {}", near_hundred(89));

    // monkey_trouble
    println!("\nTesting monkey_trouble...");
    println!("monkey_trouble(a_smile = true, b_smile = true) -> {}", monkey_trouble(true, true));
    println!("monkey_trouble(a_smile = false, b_smile = false) -> {}", monkey_trouble(false, false));
    println!("monkey_trouble(a_smile = true, b_smile = false) -> {}", monkey_trouble(true, false));

    // missing_char
    println!("\nTesting missing_char...");
    println!("missing_char(\"kitten\", 1) {}", missing_char("kitten", 1));
    println!("missing_char(\"kitten\", 0) {}", missing_char("kitten", 0));
    println!("missing_char(\"kitten\", 4) {}", missing_char("kitten", 4));

    // last_digit
    println!("\nTesting last_digit...");
    println!("last_digit() {}", last_digit(7, 17));
    println!("last_digit() {}", last_digit(6, 17));
    println!("last_digit() {}", last_digit(3, 113));
    
    // front_3
    println!("\nTesting front_3...");
    println!("front_3(string = \"Java\") -> {}", front_3("Java"));
    println!("front_3(string = \"Chocolate\") -> {}", front_3("Chocolate"));
    println!("front_3(string = \"abc\") -> {}", front_3("abc"));

    // front_22
    println!("\nTesting front_22...");
    println!("front_22( = \"code\") -> {}", front_22("code"));
    println!("front_22( = \"a\") -> {}", front_22("a"));
    println!("front_22( = \"ab\") -> {}", front_22("ab"));

    // every_up
    println!("\nTesting last_digit...");
    println!("every_up(\"Hello\") -> {}", end_up("Hello"));
    println!("every_up(\"hi there\") -> {}", end_up("hi there"));
    println!("every_up(\"hi\") -> {}", end_up("hi"));

    // front_back
    println!("\nTesting front_back...");
    println!("front_back(string = \"code\") -> {}", front_back("code"));
    println!("front_back(string = \"a\") -> {}", front_back("a"));
    println!("front_back(string = \"ab\") -> {}", front_back("ab"));


    // max_1020
    println!("\nTesting max_1020...");
    println!("max_1020(a = 11, b = 19) -> {}", max_1020(11, 19));
    println!("max_1020(a = 19, b = 11) -> {}", max_1020(19, 11));
    println!("max_1020(a = 11, b = 9) -> {}", max_1020(11, 19));

}



// Generates a list of fixed or random size containing a 
// range or random numbers
// &'static Vec<i32>
fn random_vec(size: i32, num: Vec<i32>) -> Vec<i32> {
    // treat array as range
    let mut vecky:Vec<i32> = [].to_vec();
    let mut rng = rand::thread_rng();
    if num.len() == 2 {
        for _i in 0..size {
            vecky.push(rng.gen_range(num[0]..num[1]));
        } 
    }
    return vecky;
}


