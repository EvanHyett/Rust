use std::ops::Mul;

// Square of a given number
fn square<T>(x: T) -> T
where 
    T: Copy + Mul<Output = T>,
{
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4)
    }
}

fn main() {
    println!("Hello, world!");
}
