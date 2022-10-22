use inquire::Text;

fn main() {
    let name = prompt_response("What is your name?");
    println!("Your name is {name}.")
}

fn prompt_response(prompt: &str) -> String {
    match Text::new(format!("{prompt}").as_str()).prompt() {
        Ok(name) => name,
        Err(error) => error.to_string(),
    }
}
