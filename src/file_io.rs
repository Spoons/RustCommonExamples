use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn run() {
    println!("\n\nProblem 4: File IO\nCounting words in file.txt");

    let input = read_to_string("input.txt");
    println!("String: {}", input);

    let count = word_count(&input);
    println!("found {} words\n\n", count);

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn word_count(input: &str) -> i64 {
    let mut wc = 0;
    let mut new_line = true;
    for (i, c) in input.chars().enumerate() {
        if (new_line && c.is_alphanumeric()) {
            new_line=false;
            wc+=1;
        }
        if (c == '\n') {
            new_line=true;
        }
        if (c == ' ') {
            wc+=1;
        }
    }
    return wc;
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
