pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

pub fn multiply(left: usize, right: usize) -> usize {
    left * right
}

pub fn divide(left: usize, right: usize) -> usize {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtract_works() {
        let result = subtract(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn multiply_works() {
        let result = multiply(24, 2);
        assert_eq!(result, 48);
    }

    #[test]
    fn divide_works() {
        let result = divide(24, 2);
        assert_eq!(result, 12);
    }
}
