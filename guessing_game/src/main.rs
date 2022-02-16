use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // io::stdin().read_line(&mut guess);
    // warning: unused `Result` that must be used
    // = note: `#[warn(unused_must_use)]` on by default
    // = note: this `Result` may be an `Err` variant, which should be handled

    println!("You guessed: {}", guess);

    // println!("You guessed: {}");
    // error: 1 positional argument in format string, but no arguments were given

    // println!("You guessed: ", guess);
    // error: argument never used
}
