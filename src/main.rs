
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
use rand::Rng;


fn main() {
    println!("Hello, world!");
    println!("{}", every_nth("abcdefg", 3));
    random_vec(4, [0, 10].to_vec());
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
