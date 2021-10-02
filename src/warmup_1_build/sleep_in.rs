// sleep_in.rs
/*
 * The parameter weekday is true if it is a weekday, and the 
 * parameter vacation is true if we are on vacation. We sleep in if 
 * it is not a weekday or we're on vacation. Return true if we 
 * sleep in.
 */

fn sleep_in(weekday: bool, vacation: bool) -> bool {
   return !weekday || vacation;
}

// Some quick dirty test
fn main() {
    println!("weekday = false, vacation = false");
    println!("{}", sleep_in(false, false));
    println!("weekday = true, vacation = false");
    println!("{}", sleep_in(true, false));
    println!("weekday = false, vacation = true");
    println!("{}", sleep_in(false, true));
    println!("weekday = true, vacation = true");
    println!("{}", sleep_in(true, true));
}
