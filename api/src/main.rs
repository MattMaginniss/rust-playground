use rand::{random, thread_rng, Rng};
use rand_derive2::RandGen;
use std::str::FromStr;
use strum_macros::EnumString;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "world!"
}

#[get("/farts")]
fn farts() -> &'static str {
    "Oops!...I did it again"
}

#[get("/add/<num1>/<num2>")]
fn add(num1: i64, num2: i64) -> String {
    (num1 + num2).to_string()
}

#[get("/subtract/<num1>/<num2>")]
fn subtract(num1: i64, num2: i64) -> String {
    (num1 - num2).to_string()
}

#[get("/multiply/<base>/<multiplier>")]
fn multiply(base: i64, multiplier: i64) -> String {
    (base * multiplier).to_string()
}

#[get("/divide/<num1>/<num2>")]
fn divide(num1: f64, num2: f64) -> String {
    (num1 / num2).to_string()
}

#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: String) -> String {
    println!("{}", user);
    user
}

#[post("/calc_fibonacci", format = "text", data = "<limit>")]
fn calc_fibonacci(limit: String) -> String {
    match limit.as_str().parse::<i32>() {
        Ok(parsed_limit) => {
            format!("{:?}", fibonacci(parsed_limit))
        }
        Err(e) => {
            println!("{:?}", e);
            format!("{:?}", e)
        }
    }
}

#[post("/post", format = "application/json", data = "<something>")]
fn post_something(something: String) -> String {
    format!(
        "I am just giving you whatever you gave to us:\n{}",
        something
    )
}

#[post("/dice", format = "text", data = "<dice>")]
fn dice(dice: String) -> String {
    format!("{}", roll_dice(dice))
}

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");

    let rocket = rocket::build();

    rocket
        .mount("/", routes![index])
        .mount("/", routes![farts])
        .mount("/", routes![hello])
        .mount("/", routes![world])
        .mount("/", routes![add])
        .mount("/", routes![subtract])
        .mount("/", routes![multiply])
        .mount("/", routes![divide])
        .mount("/", routes![calc_fibonacci])
        .mount("/", routes![new_user])
        .mount("/", routes![post_something])
        .mount("/", routes![dice])
}

fn fibonacci(limit: i32) -> Vec<i32> {
    let mut list_of_numbers: Vec<i32> = vec![0, 1];
    let mut current_index = 1;
    let mut next_number = 1;
    while next_number <= limit {
        next_number = list_of_numbers[current_index - 1] + list_of_numbers[current_index];
        if next_number <= limit {
            list_of_numbers.push(next_number);
            current_index += 1;
        }
    }
    list_of_numbers
}

#[derive(Debug, RandGen, Eq, PartialEq)]
enum Dice {
    D4,
    D6,
    D10,
    D12,
    D20,
}

impl FromStr for Dice {
    type Err = ();

    fn from_str(input: &str) -> Result<Dice, Self::Err> {
        match input {
            "d4" => Ok(Dice::D4),
            "4" => Ok(Dice::D4),
            "d6" => Ok(Dice::D6),
            "6" => Ok(Dice::D6),
            "d10" => Ok(Dice::D10),
            "10" => Ok(Dice::D10),
            "d12" => Ok(Dice::D12),
            "12" => Ok(Dice::D12),
            "d20" => Ok(Dice::D20),
            "20" => Ok(Dice::D20),
            _ => Err(()),
        }
    }
}

fn roll_dice(dice: String) -> i32 {
    let dice_name = Dice::from_str(dice.as_str()).unwrap();
    let mut rng = thread_rng();
    match dice_name {
        Dice::D4 => rng.gen_range(1..4),
        Dice::D6 => rng.gen_range(1..6),
        Dice::D10 => rng.gen_range(1..10),
        Dice::D12 => rng.gen_range(1..12),
        Dice::D20 => rng.gen_range(1..20),
    }
}
