extern crate rand;
use rand::Rng;

pub fn missing_element() {

    println!("Generating vectors...");
    let n_elements = 10;
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    //we begin by inserting 10 random numbers (between 0 and 100)
    //into our vec_ator
    generate_random_vec_ator(&mut vec_a, n_elements);

    //clone a into b
    vec_b = vec_a.clone();

    //we use the shuffle method from the rng crate
    rand::thread_rng().shuffle(&mut vec_a);

    //next we delete a random element
    let index_to_delete: usize = rand::thread_rng().gen_range(0, n_elements as usize);
    vec_a.remove(index_to_delete);

    println!("Vector A: {:?}", vec_a);
    println!("Vector B: {:?}", vec_b);

    let missing_element = find_missing_element(&vec_a, &vec_b);
    println!("Missing element is: {}", missing_element);
}

fn generate_random_vec_ator(v: &mut Vec<i32>, n: i32) {
    let mut rng = rand::thread_rng();
    for i in 0..n {
        v.push(rng.gen_range(0, 100));
    }
}

fn find_missing_element(v_a: &Vec<i32>, v_b: &Vec<i32>) -> i32 {
    let mut v_combined = v_a.clone();
    v_combined.append(&mut v_b.clone());


    let mut result = 0;
    for n in &v_combined {
        result = result ^ n;
    }

    return result;
}
