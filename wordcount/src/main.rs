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
fn word_count(input: String) -> i32 {
    input.split(' ').count().try_into().unwrap_or(-1)
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

    #[test]
    fn test_char_count_three_words() {
        assert_eq!(character_count("Hello world, Gary".to_string()), 15);
    }

    #[test]
    fn test_word_count_hello() {
        assert_eq!(word_count("Hello".to_string()), 1);
    }

    #[test]
    fn test_word_count_hello_world() {
        assert_eq!(word_count("Hello world".to_string()), 2);
    }

    #[test]
    fn test_word_count_three_words() {
        assert_eq!(word_count("Hello world, Gary".to_string()), 3);
    }
}
