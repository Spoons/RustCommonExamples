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

    let mut open_to_close_map = HashMap::new();
    open_to_close_map.insert(')', '(');
    open_to_close_map.insert('}', '{');
    open_to_close_map.insert(']', '[');



    for c in input.chars() {
        if open_chars.contains(&c) {
            character_stack.push(c);
        }
        if close_chars.contains(&c) {
            let c_c = character_stack.pop();
            let c_o = open_to_close_map.get(&c);

            if (c_o.unwrap() != &c_c.unwrap()) {
                return false;
            }
        }
    }
    return true;
}
