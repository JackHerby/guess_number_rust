use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    for n in 1..=10 {
        println!("Try guessing a number between 1 and 100 in less than 10 attempts.");
        println!("Round: {n}");
        println!("");
        println!("Type your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is: {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("Congratulations, you've guessed right!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
        }

        if n == 10 {
            println!("You loose! The secret number was: {secret_number}.");
        }
    }
}
