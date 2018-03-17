extern crate rand;
use rand::Rng;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Criminal {
    name: i32,
    perp: bool,
}
pub fn run() {
    let num_criminals = 7;
    let num_perps = 3;
    let num_players = 1;

    //list of criminals
    let mut criminal_list:Vec<Criminal> = Vec::new();

    //choose three criminals to be perps
    let perp_choices = choose_n_in_range(num_perps, 0, num_criminals);

    //generate the criminal list and assign perp status
    for i in 0..num_criminals {
        let mut perp = false;
        if perp_choices.contains(&i) {
            perp = true;
        }
        criminal_list.push(Criminal { name: i, perp: perp  });
    }

    loop {
        let selected_criminals = choose_n_in_range(3, 0, num_criminals);
        println!("Three criminals have been selected: {} {} {}", selected_criminals[0], selected_criminals[1],selected_criminals[2]);

        let mut c = 0;
        for n in selected_criminals {
            if criminal_list[n as usize].perp == true {
                c+=1;
            }
        }

        println!("{} of these criminals are the perps.\nWould you like to guess the perps? [y/n]", c);

        break;
    }

    println!("{:?}", criminal_list);
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
