
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::io::{self, Write};

fn main() {
    let string = read_line("Provide string: ");
    let is_palindrome =  is_palindrome(&string);
    println!("String {} is {}", string, if is_palindrome { "palindorme" } else { "not palindrome" });
}

fn is_palindrome(string: &str) -> bool {
    let grapheme_clusters = UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>();
    let last_index = grapheme_clusters.len()-1;
    for n in 0..last_index/2 {
        if grapheme_clusters[n] != grapheme_clusters[last_index-n] {
            return false
        }
    }
    true
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