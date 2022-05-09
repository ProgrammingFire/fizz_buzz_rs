use std::{fs::File, io::Write};

use crate::{traits::Colored, types::fizz_buzz::FizzBuzz};

/// ### Usage
/// Adds the FizzBuzz sequence to a vector and return it.
///
/// ### Parameters
/// * `from` - The first number in the sequence.
/// * `to` - The last number in the sequence.
///
/// ### Returns
/// A vector of FizzBuzz values.
///
/// ### Example
/// ```rust
/// use fizz_buzz;
/// let fizz_buzz_seq = fizz_buzz::run(1, 100);
/// for i in fizz_buzz_seq {
///    println!("{}", i.to_string());
/// }
/// ```
pub fn run(from: i64, to: i64) -> Vec<FizzBuzz> {
    let mut fizz_buzz: Vec<FizzBuzz> = vec![];

    for i in from..to - 1 {
        let mut result = String::from("");

        if i % 3 == 0 {
            result.push_str("Fizz");
        }

        if i % 5 == 0 {
            result.push_str("Buzz");
        }

        if result.len() == 0 {
            fizz_buzz.append(&mut vec![FizzBuzz::Number(i as u32)]);
        } else {
            fizz_buzz.append(&mut vec![FizzBuzz::from_str(&result)]);
        }
    }
    return fizz_buzz;
}

/// ### Usage
/// Prints the FizzBuzz sequence to the console.
///
/// ### Parameters
/// * `from` - The first number in the sequence.
/// * `to` - The last number in the sequence.
///
/// ### Example
/// ```rust
/// use fizz_buzz;
/// fizz_buzz::console(1, 100);
/// ```
pub fn console(from: i64, to: i64) {
    let fizz_buzz = run(from, to);
    for i in fizz_buzz {
        println!("{}", i.colorize());
    }
}

/// ### Usage
/// Prints the FizzBuzz sequence to a file.
/// 
/// ### Parameters
/// * `from` - The first number in the sequence.
/// * `to` - The last number in the sequence.
/// * `file_name` - The name of the file to write to.
/// 
/// ### Example
/// ```rust
/// use fizz_buzz;
/// fizz_buzz::file(1, 100, "fizz_buzz.txt");
/// ```
pub fn file(from: i64, to: i64, file_name: &str) {
    let fizz_buzz = run(from, to);
    let mut file = File::create(file_name).unwrap();
    for i in fizz_buzz {
        file.write_all(i.to_string().as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}
