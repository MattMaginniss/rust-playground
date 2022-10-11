fn main() {
    println!("Hello, world!");
    fizz_buzz();
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
