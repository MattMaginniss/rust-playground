fn main() {
    println!("Hello, world!");
    println!("{}", character_count("".to_owned()));
}

fn character_count(input: String) -> i32 {
    let stripped_input: String = input.split_whitespace().collect();
    match stripped_input.len().try_into() {
        Ok(value) => value,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_char_count_hello() {
        assert_eq!(character_count("Hello".to_string()), 5);
    }

    #[test]
    fn test_char_count_hello_world() {
        assert_eq!(character_count("Hello World".to_string()), 10);
    }
}
