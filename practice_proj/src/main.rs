fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // variable tup binds to the entire tuple
    // To get individual values out of a tuple, we can use pattern
    // matching to destructure a tuple value like this:
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Another way to desctructure is by using a period (.) 
    // followed by the index of the value we want to access. Ex:

    let e: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = e.0;
    let six_point_four = e.1;
    let one = e.2;

    println!("The value of e.0 is: {}", five_hundred);
    println!("The value of e.1 is: {}", six_point_four);
    println!("The value of e.2 is: {}", one);
}
