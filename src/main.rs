
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

mod warmup_2_build;
use crate::warmup_2_build::alt_pairs::alt_pairs;
use crate::warmup_2_build::array_667::array_667;
use crate::warmup_2_build::last_2::last_2;
use crate::warmup_2_build::string_x::string_x;
use crate::warmup_2_build::array_123::array_123;

/*
 *
 * 
use crate::warmup_2_build::last_2::last_2;
use crate::warmup_2_build::array_count_9::array_count_9;

use crate::warmup_2_build::string_yak::sting_yak;
use crate::warmup_2_build::string_times::string_times;
use crate::warmup_2_build::string_splosion::string_splosion;
use crate::warmup_2_build::string_match::string_match;
use crate::warmup_2_build::string_bits::string_bits;
use crate::warmup_2_build::no_triples::no_triples;
use crate::warmup_2_build::count_xx::count_xx;
use crate::warmup_2_build::double_x::double_x;
use crate::warmup_2_build::front_times::front_times;
use crate::warmup_2_build::has_271::has_271;
*/

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
    println!("diff_21(num = 19) -> {}", diff_21(19));
    println!("diff_21(num = 10) -> {}", diff_21(10)); 
    println!("diff_21(num = 21) -> {}", diff_21(21)); 
    
    println!("\nTesting every_nth..."); 
    println!("every_nth(string = \"Miracle\" 2) -> {}", every_nth("Miracle", 2));
    println!("every_nth(string = \"abcdefg\", 3) -> {}", every_nth("abcdefg", 3));

    // sum_double
    println!("\nTesting sum_double...");
    println!("sum_double(a = 1, b = 2) -> {} ", sum_double(1, 2));
    println!("sum_double(a = 3, b = 2) -> {} ", sum_double(3, 2));
    println!("sum_double(a = 2, b= 2) -> {} ", sum_double(2, 2));

    // string_e
    println!("\nTesting string_e...");
    println!("string_e(string = \"Hello\") -> {} ", string_e("Hello"));
    println!("string_e(string = \"Heelle\") -> {} ", string_e("Heelle"));
    println!("string_e(string = \"Heelele\") -> {} ", string_e("Heelele"));

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
    println!("not_string(string = \"candy\") -> {}", not_string("candy"));

    // makes_10
    println!("\n Testing makes_10...");
    println!("makes_10(a = 9, b = 10) -> {}", makes_10(9, 10));
    println!("makes_10(n = 9, b = 9) -> {}", makes_10(9, 9));
    println!("makes_10(n = 1, b = 9) -> {}", makes_10(1, 9));

    // near_100
    println!("\nTesting near_100...");
    println!("near_100(num = 93) -> {}", near_hundred(93));
    println!("near_100(num = 90) -> {}", near_hundred(90));
    println!("near_100(num = 89) -> {}", near_hundred(89));

    // monkey_trouble
    println!("\nTesting monkey_trouble...");
    println!("monkey_trouble(a_smile = true, b_smile = true) -> {}", monkey_trouble(true, true));
    println!("monkey_trouble(a_smile = false, b_smile = false) -> {}", monkey_trouble(false, false));
    println!("monkey_trouble(a_smile = true, b_smile = false) -> {}", monkey_trouble(true, false));

    // missing_char
    println!("\nTesting missing_char...");
    println!("missing_char(string = \"kitten\", at = 1) {}", missing_char("kitten", 1));
    println!("missing_char(string = \"kitten\", at = 0) {}", missing_char("kitten", 0));
    println!("missing_char(string = \"kitten\", at = 4) {}", missing_char("kitten", 4));

    // last_digit
    println!("\nTesting last_digit...");
    println!("last_digit(a = 7, b = 17) {}", last_digit(7, 17));
    println!("last_digit(a = 7, b = 17) {}", last_digit(6, 17));
    println!("last_digit(a = 3, b = 113) {}", last_digit(3, 113));
    
    // front_3
    println!("\nTesting front_3...");
    println!("front_3(string = \"Java\") -> {}", front_3("Java"));
    println!("front_3(string = \"Chocolate\") -> {}", front_3("Chocolate"));
    println!("front_3(string = \"abc\") -> {}", front_3("abc"));

    // front_22
    println!("\nTesting front_22...");
    println!("front_22(string = \"code\") -> {}", front_22("code"));
    println!("front_22( string = \"a\") -> {}", front_22("a"));
    println!("front_22(strng = \"ab\") -> {}", front_22("ab"));

    // every_upcuz if so, 
    println!("\nTesting end_up...");
    println!("end_up(string = \"Hello\") -> {}", end_up("Hello"));
    println!("end_up(string = \"hi there\") -> {}", end_up("hi there"));
    println!("end_up(string = \"hi\") -> {}", end_up("hi"));

    // front_back
    println!("\nTesting front_back...");
    println!("front_back(string = \"code\") -> {}", front_back("code"));
    println!("front_back(string = \"a\") -> {}", front_back("a"));
    println!("front_back(string = \"ab\") -> {}", front_back("ab"));


    // max_1020
    println!("\nTesting max_1020...");
    println!("max_1020(a = 11, b = 19) -> {}", max_1020(11, 19));
    println!("max_1020(a = 19, b = 11) -> {}", max_1020(19, 11));
    println!("max_1020(a = 11, b = 9) -> {}", max_1020(11, 9));

    // icy_hot
    println!("\nTestng icy_hot...");
    println!("icyHot(120, -1)");
    println!("icyHot(-1, 120");
    println!("icyHot(2, 120)");

    // del_del

    //***************
    // WARMUP No. 2 *
    //***************
    
    // alt_pairs
    println!("\nTestng alt_pairs...");
    println!("alt_pairs(string = \"kitten\") -> {}", alt_pairs("kitten"));
    println!("alt_pairs(string = \"Chocolate\") -> {}", alt_pairs("Chocolate"));
    println!("alt_pairs(string = \"CodingHorror\") -> {}", alt_pairs("CodingHorror"));

    // array_667
    println!("\nTestng array_667...");
    println!("arrays_667(nums = [6, 6, 2]) -> {}", array_667(&mut [6,6,2]));
    println!("arrays_667(nums = [6, 6, 2, 6]) -> {}", array_667(&mut[6,6,2,6]));
    println!("arrays_667(nums = [6, 7, 2, 6]) -> {}", array_667(&mut [6,7,2,6]));

    // last_2
    println!("\nTestng last_2...");
    println!("last_2(string = \"hixxhi\") -> {}", last_2("hixxhi"));
    println!("last_2(string = \"xaxxaxaxx\") -> {}", last_2("xaxxaxaxx"));
    println!("last_2(string = \"xaxxaxaxx\") -> {}", last_2("axxxaaxx")); 

    // string_x (
    println!("\nTestng string_x...");
    println!("string_x(string = \"xxHxix\") -> {}", string_x("xxHxix"));
    println!("string_x(string = \"abxxxcd\") -> {}", string_x("abxxxcd"));
    println!("string_x(string = \"xabxxxcdx\") -> {}", string_x("xabxxxcdx")); 


    // array_123
    println!("array_123(string = [1, 1, 2, 3, 1]) -> {}",  array_123(&mut [1, 1, 2, 3, 1]));
    println!("array_123(string = [1, 1, 2, 4, 1]) -> {}", array_123(&mut [1, 1, 2, 4, 1]));
    println!("array_123(string = [1, 1, 2, 1, 2, 3]) -> {}",  array_123(&mut [1, 1, 2, 1, 2, 3]));
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


