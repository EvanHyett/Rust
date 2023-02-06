fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    expression_test();

    let example = five();
    println!("The value of example is: {}", example);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn expression_test(){
    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
