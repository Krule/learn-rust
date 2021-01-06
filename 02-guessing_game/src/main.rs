// https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    println!("Please input the guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess);
}
