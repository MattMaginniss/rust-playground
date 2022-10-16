use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world! Welcome to the GUESSING GAME");
    let correct_answer = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 1;
    loop {
        println!("Please guess a number from 1-100");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if !(1..=100).contains(&guess) {
            println!("You need to guess between 1-100. You entered {guess}");
        } else {
            match guess.cmp(&correct_answer) {
                Ordering::Less => println!("{}", "TOO LOW".red().italic()),
                Ordering::Greater => println!("{}", "TOO HIGH".yellow().italic()),
                Ordering::Equal => {
                    println!("{}", "You're right!".green().bold());
                    println!("It took you {guesses} guesses.");
                    println!("You won!");
                    println!("How does it feel?");
                    break;
                }
            }
            guesses += 1;
        }
    }
}
