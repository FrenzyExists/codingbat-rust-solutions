
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


