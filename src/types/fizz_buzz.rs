use crate::traits::Colored;
use colored::{ColoredString, Colorize};

pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl FizzBuzz {
    pub fn from_str(s: &str) -> FizzBuzz {
        match s {
            "Fizz" => FizzBuzz::Fizz,
            "Buzz" => FizzBuzz::Buzz,
            "FizzBuzz" => FizzBuzz::FizzBuzz,
            _ => FizzBuzz::Number(s.parse().unwrap()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FizzBuzz::FizzBuzz => "FizzBuzz".to_string(),
            FizzBuzz::Fizz => "Fizz".to_string(),
            FizzBuzz::Buzz => "Buzz".to_string(),
            FizzBuzz::Number(n) => n.to_string(),
        }
    }
}

impl Colored for FizzBuzz {
    fn colorize(&self) -> ColoredString {
        match self {
            FizzBuzz::FizzBuzz => self.to_string().bright_green().bold(),
            FizzBuzz::Fizz => self.to_string().bright_purple().bold(),
            FizzBuzz::Buzz => self.to_string().bright_cyan().bold(),
            FizzBuzz::Number(n) => n.to_string().bright_yellow().bold(),
        }
    }
}
