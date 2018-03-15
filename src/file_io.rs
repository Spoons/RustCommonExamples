use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn run() {
    println!("Problem 4: File IO\nCounting words in file.txt\n");

    let input = read_to_string("input.txt");

    println!("{}", input);
    let mut word_count = 0;



    println!("Found {} words", word_count);

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn read_to_string(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("Unable to open file {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Unable to read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("File read"),
    }

    return s;
}
