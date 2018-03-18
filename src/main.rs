extern crate rand;
use rand::Rng;

mod missing_element;
mod file_io;
mod permutations;
mod threeofcrime;
mod balanced_paranthesis;

fn main () {
    println!("Problem 1: ");
    missing_element::missing_element();
    println!("Problem 2: ");
    balanced_paranthesis::run();
    println!("Problem 3: ");
    permutations::print_permutations("abc");
    permutations::print_permutations("put");
    println!("Problem 4: ");
    file_io::run();
    println!("Problem 5: ");
    threeofcrime::run();
}
