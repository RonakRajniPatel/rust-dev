use std::io;

fn main() {
    println!("Hello, world!");
    println!("The rust println function uses a !");
    println!("Guess a number and input your guess!");
    let mut guess = String::new(); // mutatable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);

}
