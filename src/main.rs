extern crate rand;
use rand::Rng;

mod missing_element;
mod file_io;
mod permutations;
mod threeofcrime;

fn main () {
    missing_element::missing_element();
    println!("Missing problem 2");
    permutations::print_permutations("abc");
    permutations::print_permutations("put");
    file_io::run();
    threeofcrime::run();
}
