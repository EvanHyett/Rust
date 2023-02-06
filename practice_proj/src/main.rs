fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // variable tup binds to the entire tuple
    // To get individual values out of a tuple, we can use pattern
    // matching to destructure a tuple value like this:
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
