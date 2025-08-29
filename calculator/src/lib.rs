
pub fn add_num(a: i64, b: i64) -> i64 {
    a + b
}

pub fn subtract_num(a: i64, b: i64) -> i64 {
    a - b
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        let result = add_num(4, 5);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_add_negative_numbers() {
        let result = add_num(-4, -2);
        assert_eq!(result, -6);
    }

    #[test]
    #[should_panic]
    fn test_add_should_panic() {
        panic!("This test will pass because it panics");
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(subtract_num(2, 1), 1);
        assert_eq!(subtract_num(-1, -2), 1);
    }
}




