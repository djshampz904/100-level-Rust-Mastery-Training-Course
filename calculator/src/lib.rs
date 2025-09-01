pub fn add_num(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract_num(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply_num(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide_num(a: f64, b: f64) -> Result<f64, String>{
    if b == 0.0 {
        Err("Error: Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}



#[cfg(test)]
mod test {
    use super::*;

    // ✅ Basic integer arithmetic
    #[test]
    fn test_add_integers() {
        assert_eq!(add_num(2.0, 3.0), 5.0);
        assert_eq!(add_num(-4.0, 6.0), 2.0);
    }

    #[test]
    fn test_subtract_integers() {
        assert_eq!(subtract_num(10.0, 4.0), 6.0);
        assert_eq!(subtract_num(0.0, 5.0), -5.0);
    }

    #[test]
    fn test_multiply_integers() {
        assert_eq!(multiply_num(3.0, 5.0), 15.0);
        assert_eq!(multiply_num(-2.0, 3.0), -6.0);
    }

    // ✅ Division returns Result and handles normal cases
    #[test]
    fn test_divide_integers() {
        assert_eq!(divide_num(8.0, 2.0), Ok(4.0));
        assert_eq!(divide_num(-9.0, 3.0), Ok(-3.0));
    }

    // ✅ Division by zero returns Err, not panic
    #[test]
    fn test_divide_by_zero() {
        let result = divide_num(5.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error: Cannot divide by zero".to_string());
    }

    // ✅ Precision-safe float check without unwrap
    #[test]
    fn test_precision_safe_addition() {
        let result = add_num(0.1, 0.2);
        assert!((result - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_precision_safe_division() {
        match divide_num(1.0, 3.0) {
            Ok(val) => assert!((val - 0.333333333333).abs() < 1e-10),
            Err(_) => panic!("Division should not fail here"),
        }
    }

    // ✅ Simulated CLI parsing tests
    fn parse_arg(arg: &str) -> Result<f64, String> {
        arg.parse::<f64>().map_err(|_| "Please use a number".to_string())
    }

    #[test]
    fn test_invalid_number_input() {
        let result = parse_arg("abc");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Please use a number");
    }

    #[test]
    fn test_valid_number_input() {
        let result = parse_arg("42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42.0);
    }
}
