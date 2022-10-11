fn main() {
    println!("Hello, world!");
    println!("\nFizz Buzz:");
    fizz_buzz();
    println!("\nPalindromes:");
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
    is_palindrome("word");
    is_palindrome("racecar");
    is_palindrome("RaCEcAr");
    is_palindrome("poopy");
    is_palindrome("gary");
    is_palindrome("poop");
}

fn is_palindrome(word: &str) -> bool {
    let palindrome_value =
        word.to_lowercase() == word.to_lowercase().chars().rev().collect::<String>();
    println!(
        "{word} {} a palindrome",
        if palindrome_value { "is" } else { "is not" }
    );
    palindrome_value
}
