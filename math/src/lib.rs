pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

pub fn multiply(left: i32, right: i32) -> i32 {
    left * right
}

pub fn divide(left: i32, right: i32) -> i32 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_same() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_0() {
        let result = add(2, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn subtract_same() {
        let result = subtract(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn subtract_to_neg() {
        let result = subtract(2, 4);
        assert_eq!(result, -2);
    }

    #[test]
    fn subtract_to_positive() {
        let result = subtract(8, 4);
        assert_eq!(result, 4);
    }

    #[test]
    fn multiply_by_2() {
        let result = multiply(24, 2);
        assert_eq!(result, 48);
    }

    #[test]
    fn multiply_0() {
        let result = multiply(24, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn multiply_neg() {
        let result = multiply(24, -1);
        assert_eq!(result, -24);
    }

    #[test]
    fn divide_to_half() {
        let result = divide(24, 2);
        assert_eq!(result, 12);
    }

    #[test]
    fn divide_self() {
        let result = divide(24, 24);
        assert_eq!(result, 1);
    }

    #[test]
    #[should_panic]
    fn divide_by_0_fails() {
        divide(24, 0);
    }
}
