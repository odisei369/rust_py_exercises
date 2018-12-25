use std::io::{self, Write};

fn main() {
    let mut values: Vec<i32> = Vec::new();
    loop {
        let value = read_line("Provide number(or write 'enough', to end inputing numbers): ");
        if value == "enough" { 
            break
        }
        values.push(value.parse().expect("Can't parse provided number"));
    }
    values.retain(|&x| x < 10);
    println!("Values less than 10: {:?}", values);
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