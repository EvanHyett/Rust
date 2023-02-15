use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng().gen_range(1, 10);

    for _ in 1..10 {
        rng.gen::<f64>();
    }

    println!("Please choose a number");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("The number you typed was: {}", input);

}

     //     0
     //    /|\
     //    | | 
