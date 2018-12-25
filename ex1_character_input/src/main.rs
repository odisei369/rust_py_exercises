use std::io::{self, Write};

fn main() {
    let name = read_line("Give me your name: ");
    print!("Hello {}!\n", name);
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