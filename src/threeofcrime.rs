extern crate rand;
use rand::Rng;
use std::io;


static NUM_CRIMINALS: i32 = 7;
static NUM_PERPS: i32 = 3;
static NUM_PLAYERS: i32 = 1;


#[derive(Debug)]
pub struct Criminal {
    name: i32,
    perp: bool,
}
pub fn run() {

    //list of criminals
    let mut criminal_list:Vec<Criminal> = Vec::new();

    //choose three criminals to be perps
    let perp_choices = choose_n_in_range(NUM_PERPS, 0, NUM_CRIMINALS);

    //generate the criminal list and assign perp status
    for i in 0..NUM_CRIMINALS {
        let mut perp = false;
        if perp_choices.contains(&i) {
            perp = true;
        }
        criminal_list.push(Criminal { name: i, perp: perp  });
    }

    loop {
        let selected_criminals = choose_n_in_range(3, 0, NUM_CRIMINALS);
        println!("Three criminals have been selected: {} {} {}", selected_criminals[0], selected_criminals[1],selected_criminals[2]);

        let mut c = 0;
        for n in selected_criminals {
            if criminal_list[n as usize].perp == true {
                c+=1;
            }
        }

        println!("{} of these criminals are the perps.\nWould you like to guess the perps? [yes/no]", c);
        let choice: bool = player_input_yes_or_no();

        if !choice {
            continue;
        }

        println!("Valid input: \"1 2 3\"\nEnter your guess: ");

        let choices = get_player_choices();



        break;
    }

    println!("{:?}", criminal_list);
}
fn get_player_choices() -> Vec<i32> {
    let stdin = io::stdin();
    let mut choices: Vec<i32>;
    loop {
        let mut buffer = String::new();
        let _read_result = stdin.read_line(&mut buffer);
        buffer = buffer.trim().to_string();

        choices = buffer.split(" ").filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();

        if choices.len() != 3 {
            println!("Incorrect number of choices.\nTry Again: ");
            continue
        }

        for n in &choices {
            if n < &0 || n > &NUM_CRIMINALS {
                println!("Invalid selection. Selection out of range.\nTry again: ");
            }
        }
        break;
    }

    return choices;
}
fn player_input_yes_or_no () -> bool {
    let stdin = io::stdin();
    let mut buffer = String::new();
    loop {
        let _read_result = stdin.read_line(&mut buffer);
        buffer = buffer.trim().to_string();
        match buffer.as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("invalid input.\nanswer yes or no."),
        }
    }
}
fn choose_n_in_range(n: i32, l: i32, r: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    let mut result: Vec<i32> = Vec::new();
    let mut counter = 0;
    loop {
        let new_perp = rng.gen_range(l, r);
        if result.contains(&new_perp) == false {
            result.push(new_perp);
            counter += 1;
        }

        if counter == n {
            break;
        }
    }

    return result;

}
