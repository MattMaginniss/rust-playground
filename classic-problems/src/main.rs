fn main() {
    println!("Hello, world!");
    println!("\nFizz Buzz");
    fizz_buzz();
    println!("\nPalindromes");
    palindrome_checker();
}

fn fizz_buzz() {
    let mut number = 1;

    while number < 101 {
        if number % 15 == 0 {
            println!("Fizz Buzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
        number += 1;
    }
}

fn palindrome_checker() {
    println!("{}", is_palindrome("word"));
    println!("{}", is_palindrome("racecar"));
    println!("{}", is_palindrome("RaCEcAr"));
    println!("{}", is_palindrome("poopy"));
    println!("{}", is_palindrome("gary"));
    println!("{}", is_palindrome("poop"));
}

fn is_palindrome(word: &str) -> bool {
    println!(
        "{0} - {1}",
        word.to_lowercase(),
        word.to_lowercase().chars().rev().collect::<String>()
    );
    word.to_lowercase() == word.to_lowercase().chars().rev().collect::<String>()
}
