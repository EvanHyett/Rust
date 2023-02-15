use std::io;

fn main() {

    println!("Please choose a number");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("The number you typed was: {}", input);

}

     //     0
     //    /|\
     //    | | 
