use lab2;
use lab3;
use std::io;
fn main() {
    println!("Hello, world! 123");
    let num = 100;
    println!("AAA: {}", lab2::add_one(num));
    println!("BBB: {}", lab3::add_two(num));

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
