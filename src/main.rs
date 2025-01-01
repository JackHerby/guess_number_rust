use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Try guessing a number between 1 and 100.");
        println!("Type your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too little!"),
            Ordering::Equal => {
                println!("Congratulations, you've guessed right!");
                break;
            }
            Ordering::Greater => println!("Too much!"),
        }
    }
}
