use colored::ColoredString;

pub trait Colored {
    fn colorize(&self) -> ColoredString;
}
