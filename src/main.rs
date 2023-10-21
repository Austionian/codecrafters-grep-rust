use std::env;
use std::io;
use std::process;

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    if let Some(s) = pattern.strip_prefix('\\') {
        match s {
            "d" => {
                let bool = input_line
                    .chars()
                    .fold(false, |acc, c| acc || c.is_digit(10));
                if bool {
                    process::exit(0);
                } else {
                    process::exit(1);
                }
            }
            "w" => {
                let bool = input_line
                    .chars()
                    .fold(false, |acc, c| acc || c.is_digit(10) || c.is_alphabetic());
                if bool {
                    process::exit(0);
                } else {
                    process::exit(1);
                }
            }
            _ => unimplemented!(),
        }
    } else {
        if pattern.chars().next() == Some('[') {
            let pattern = pattern
                .strip_prefix('[')
                .and_then(|rest| rest.strip_suffix(']'))
                .unwrap();

            if pattern.chars().next() == Some('^') {
                let pattern = pattern.strip_prefix('^').unwrap();
                for c in pattern.chars() {
                    if input_line.contains(c) {
                        process::exit(1);
                    } else {
                        process::exit(0);
                    }
                }
            }

            for c in pattern.chars() {
                if input_line.contains(c) {
                    process::exit(0);
                } else {
                    process::exit(1);
                }
            }
        }
        if input_line.contains(&pattern) {
            process::exit(0);
        } else {
            process::exit(1);
        }
    }
}
