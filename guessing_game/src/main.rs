use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // io::stdin().read_line(&mut guess);
        // warning: unused `Result` that must be used
        // = note: `#[warn(unused_must_use)]` on by default
        // = note: this `Result` may be an `Err` variant, which should be handled

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // println!("You guessed: {}");
        // error: 1 positional argument in format string, but no arguments were given

        // println!("You guessed: ", guess);
        // error: argument never used

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
