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
}
