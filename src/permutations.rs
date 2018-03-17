pub fn print_permutations(input: &str) {
    println!("generating perumatations from: {}", input);
    permutate(input, 0, input.len() as i32);
}

fn permutate(input: &str, left: i32, right: i32) {
    let mut perms = input.to_string();
    let mut i: i32;
    if (left == right) {
        println!("{}", input);
    } else {
        for i in left..right {
            perms = swap_characters(&perms, left as usize, i as usize);
            permutate(&perms, left+1, right);
            perms = swap_characters(&perms, left as usize, i as usize);
        }
    }
}

fn swap_characters(input: &str, l: usize, r: usize) -> String {
    let mut modified_string = String::new();

    for (i, c) in input.chars().enumerate() {
        if (i == l) {
            let c_r: char = input.chars().nth(r).unwrap();
            modified_string.push(c_r);
            continue;
        } else if i == r {
            let c_l = input.chars().nth(l).unwrap();
            modified_string.push(c_l);
            continue;
        } else {
            modified_string.push(c);
        }
    }

    return modified_string;

}
