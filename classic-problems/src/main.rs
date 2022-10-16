fn main() {
    println!("Hello, world!");
    println!("\nFizz Buzz:");
    fizz_buzz();
    println!("\nPalindromes:");
    palindrome_checker();
    collatz(1253);
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
    is_palindrome("Esme");
    is_palindrome("RaCEcAr");
    is_palindrome("spooky");
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

fn collatz(starting_num: u64) -> i32 {
    let mut steps = 0;
    let mut current_num = starting_num;
    println!("\nCollatz Conjecture starting at: {current_num}");
    println!("Starting at: {current_num}");
    while current_num != 1 {
        steps += 1;
        if current_num % 2 == 0 {
            current_num /= 2;
        } else {
            current_num = (current_num * 3) + 1;
        }
        println!("{current_num}");
    }
    println!("It took {steps} steps to get from {starting_num} to 1");
    steps
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_palindrome_cats() {
        assert!(!is_palindrome("cats"));
    }

    #[test]
    fn test_palindrome_racecar() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn test_palindrome_racecar_caps() {
        assert!(is_palindrome("raCeCAr"));
    }

    #[test]
    fn test_palindrome_esme() {
        assert!(!is_palindrome("Esme"));
    }

    #[test]
    fn test_palindrome_spooky() {
        assert!(!is_palindrome("Spooky"));
    }

    #[test]
    fn test_collatz_9() {
        assert_eq!(collatz(9), 19);
    }

    #[test]
    fn test_collatz_97() {
        assert_eq!(collatz(97), 118);
    }

    #[test]
    fn test_collatz_871() {
        assert_eq!(collatz(871), 178);
    }

    #[test]
    fn test_collatz_6171() {
        assert_eq!(collatz(6171), 261);
    }

    #[test]
    fn test_collatz_989345275647() {
        assert_eq!(collatz(989345275647), 1348);
    }
}
