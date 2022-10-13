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
    let parsed_limit = str::parse(limit.as_str());
    format!("{:?}", fibonacci(parsed_limit))
}

#[post("/post", format = "application/json", data = "<something>")]
fn post_something(something: String) -> String {
    format!(
        "I am just giving you whatever you gave to us:\n{}",
        something
    )
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
