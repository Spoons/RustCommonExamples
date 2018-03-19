use std::collections::HashMap;

pub fn run() {
    let s1 = "({[]})";
    let s2 = "([)]";

    println!("{} balanced: {}", s1, match_parantheses(s1));
    println!("{} balanced: {}", s2, match_parantheses(s2));
}


pub fn match_parantheses(input: &str) -> bool {
    let mut character_stack: Vec<char> = Vec::new();
    let open_chars: Vec<char> = vec!['(', '{', '['];
    let close_chars: Vec<char> = vec![')', '}', ']'];

    let mut close_to_open_map = HashMap::new();
    close_to_open_map.insert(')', '(');
    close_to_open_map.insert('}', '{');
    close_to_open_map.insert(']', '[');



    for c in input.chars() {
        if open_chars.contains(&c) {
            character_stack.push(c);
        }
        if close_chars.contains(&c) {
            let last_open_char = character_stack.pop();
            let matching_char = close_to_open_map.get(&c);

            if (matching_char.unwrap() != &last_open_char.unwrap()) {
                return false;
            }
        }
    }
    return true;
}
