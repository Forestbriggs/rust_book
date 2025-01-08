use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut tries = 5;
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        if tries <= 0 {
            println!("You Lose, major loser!");
            println!("The secret number was {secret_number}");
            break;
        }

        println!("You have {tries} tries left");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too low!");
                tries -= 1;
            }
            Ordering::Greater => {
                println!("Too high!");
                tries -= 1;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
