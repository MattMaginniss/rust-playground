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

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");
    let rocket = rocket::build();

    rocket
        .mount("/", routes![index])
        .mount("/", routes![farts])
        .mount("/", routes![hello])
        .mount("/", routes![world])
}
