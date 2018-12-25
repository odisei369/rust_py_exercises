use std::io::{self, Write};

fn main() {
    let mut divisors: Vec<i32> = Vec::new();
    let value = read_line("Provide number: ");
    let number = value.parse::<i32>().expect("Can't parse provided number");
    let divisors: Vec<i32> = (1..number/2).filter(|&x| number % x == 0).collect();
    println!("Divisors of {}: {:?}", number, divisors);
}

fn read_line(message :&str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Can't flush to stdout");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Can't read stdin");
    let white_trailing_len = buffer.trim_end().len();
    buffer.truncate(white_trailing_len);
    buffer
}
