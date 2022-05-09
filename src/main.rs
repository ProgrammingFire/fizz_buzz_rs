use fizz_buzz::fizz_buzz;

fn main() {
    fizz_buzz::console(1, 100);
    fizz_buzz::file(1, 100, "fizz_buzz.txt");
}
