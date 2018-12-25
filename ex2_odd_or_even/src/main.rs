use std::io::{self, Write};

fn main() {
    let name = read_line("Provide number: ");
    let value: i32 = name.parse().expect("Can't parse provided number");
    println!("Value {} is {:?}", value, if value % 2 == 0 { "even" } else { "odd" });
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