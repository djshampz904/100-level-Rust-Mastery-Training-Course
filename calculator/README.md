# 🧮 Memory-Safe Calculator

A robust, panic-free command-line calculator built in Rust that demonstrates memory safety, error handling, and proper Result type usage without ever calling `.unwrap()`.

## 🎯 Project Overview

This calculator is part of the **100-Level Rust Mastery Training Course** - specifically **Level 1: Memory-Safe Calculator**. It serves as a foundational project that teaches core Rust concepts through practical implementation.

## ✨ Features

- **Memory Safe**: Never panics, handles all edge cases gracefully
- **Error Handling**: Comprehensive error propagation using the `?` operator pattern
- **Arithmetic Operations**: Supports `+`, `-`, `*`, `/` operations
- **Division by Zero Protection**: Explicit handling with meaningful error messages
- **Command-Line Interface**: Parse arguments safely with proper validation
- **Comprehensive Testing**: Full test coverage including edge cases

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (edition 2024)
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd calculator
```

2. Build the project:
```bash
cargo build
```

3. Run the calculator:
```bash
cargo run -- <number1> <operator> <number2>
```

### Usage Examples

```bash
# Addition
cargo run -- 5 + 3
# Output: 5 + 3 = 8

# Subtraction
cargo run -- 10 - 4
# Output: 10 - 4 = 6

# Multiplication
cargo run -- 6 * 7
# Output: 6 * 7 = 42

# Division
cargo run -- 15 / 3
# Output: 15 / 3 = 5

# Division by zero (handled gracefully)
cargo run -- 10 / 0
# Output: Error: Cannot divide by zero
```

## 🏗️ Architecture

### Core Functions

The calculator is built around these core arithmetic functions:

- `add_num(a: f64, b: f64) -> f64` - Safe addition
- `subtract_num(a: f64, b: f64) -> f64` - Safe subtraction  
- `multiply_num(a: f64, b: f64) -> f64` - Safe multiplication
- `divide_num(a: f64, b: f64) -> Result<f64, String>` - Safe division with error handling

### Error Handling Strategy

- **Parsing Errors**: Graceful handling of invalid number inputs
- **Missing Arguments**: Clear error messages for incomplete input
- **Invalid Operators**: Validation of arithmetic symbols
- **Division by Zero**: Explicit error handling without panics

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

### Test Coverage

- ✅ Basic integer arithmetic operations
- ✅ Division by zero error handling
- ✅ Precision-safe floating-point operations
- ✅ CLI argument parsing validation
- ✅ Error message verification

## 📚 Learning Objectives

This project demonstrates:

1. **Memory Safety**: No `.unwrap()` calls, proper error handling
2. **Error Propagation**: Using `Result<T, E>` types effectively
3. **Command-Line Parsing**: Safe argument handling with `std::env::args()`
4. **String Parsing**: Converting strings to numbers with error handling
5. **Pattern Matching**: Comprehensive `match` statements for control flow
6. **Testing**: Writing thorough tests for edge cases

## 🔧 Dependencies

- `rust_decimal`: High-precision decimal arithmetic
- `rust_decimal_macros`: Decimal literal macros for precision

## 🚫 What This Project Does NOT Do

- Use `.unwrap()` or `.expect()` (violates memory safety principles)
- Panic on any input (maintains program stability)
- Ignore error cases (comprehensive error handling)
- Use unsafe code blocks

## 🎓 Success Criteria Met

- ✅ **Never Panics**: All error cases handled gracefully
- ✅ **Edge Case Coverage**: Division by zero, invalid inputs, missing arguments
- ✅ **Meaningful Errors**: Clear, user-friendly error messages
- ✅ **Memory Safe**: Proper use of Result types and error propagation
- ✅ **Comprehensive Testing**: Full coverage of functionality and edge cases

## 🌟 Advanced Features

- **Precision Handling**: Uses `rust_decimal` for accurate floating-point arithmetic
- **Extensible Design**: Easy to add new operations or modify existing ones
- **Production Ready**: Robust error handling suitable for real-world use

## 🤝 Contributing

This project follows conventional commit standards:

```bash
git checkout -b feature/calculator
# Make your changes
git commit -m "feat: add new arithmetic operation"
git push origin feature/calculator
```

## 📖 Rust Concepts Demonstrated

- **Ownership & Borrowing**: Safe string and number handling
- **Error Handling**: `Result<T, E>` and `?` operator patterns
- **Pattern Matching**: Comprehensive `match` statements
- **Modules**: Library and binary crate organization
- **Testing**: Unit tests with edge case coverage
- **Command-Line Interfaces**: Safe argument parsing

## 🔍 Code Quality

- **Zero Clippy Warnings**: Clean, idiomatic Rust code
- **Comprehensive Documentation**: Clear function documentation
- **Consistent Error Handling**: Uniform approach across all operations
- **Memory Efficient**: No unnecessary allocations or unsafe operations

## 📈 Next Steps

After mastering this foundation, consider:

- Adding more mathematical operations (power, square root, etc.)
- Implementing a REPL (Read-Eval-Print Loop) interface
- Adding support for complex numbers
- Building a graphical calculator interface
- Implementing unit conversion capabilities

---

**Built with ❤️ and Rust** | **Level 1 Foundation Project** | **100-Level Rust Mastery Training Course**
