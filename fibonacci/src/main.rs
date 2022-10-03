use std::io;

fn main() {
    println!("Generate the nth Fibonacci number!");
    println!("Enter the number you would like to generate:");

    // store the input in get_input function
    let n = get_input();
    let fib_num = generate_fib_num(&n);

    println!("The Fibonacci number at position {} is {}", n, fib_num);
}

fn get_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Please input a positive, whole number ({})", e);
            get_input()
        }
    };
    input
}

fn generate_fib_num(n: &u32) -> u32 {
    if *n <= 1 {
        *n
    } else {
        generate_fib_num(&(n-1)) + generate_fib_num(&(n-2))
    }
} 
