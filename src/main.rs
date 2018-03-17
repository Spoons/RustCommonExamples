extern crate rand;
use rand::Rng;

mod missing_element;
mod file_io;
mod permutations;

fn main () {
    missing_element::missing_element();
    file_io::run();
    permutations::print_permutations("abc");
    permutations::print_permutations("put");

}
