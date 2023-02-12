fn main() {
    let some_u8_value = 08;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}