fn main() {
    println!("Hello, world!");
    println!(
        "Hello World characters: {}",
        character_count("Hello World".to_owned())
    );
    println!(
        "Hello World words: {}",
        word_count("Hello World".to_owned())
    );
}

fn character_count(input: String) -> i32 {
    let stripped_input: String = input.split_whitespace().collect();
    stripped_input.len().try_into().unwrap_or(-1)
}

fn word_count(input: String) -> i32 {
    input.split(' ').count().try_into().unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_count_spaces() {
        assert_eq!(character_count("    ".to_string()), 0);
    }

    #[test]
    fn test_word_count_word_and_spaces() {
        assert_eq!(character_count("     WORD   ".to_string()), 4);
    }

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
