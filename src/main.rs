use std::env;
use std::io;
use std::process;

fn reg_match<'a>(
    pattern: &'a str,
    input_line: &'a str,
) -> (bool, Option<&'a str>, Option<&'a str>) {
    if let Some(s) = pattern.strip_prefix('\\') {
        match s.chars().next() {
            Some('d') => {
                let mut matched = false;
                let mut idx = 0;
                for (i, c) in input_line.chars().enumerate() {
                    if c.is_digit(10) {
                        idx = i;
                        matched = true;
                        break;
                    }
                }
                (matched, pattern.get(2..), input_line.get(idx + 1..))
            }
            Some('w') => {
                let mut matched = false;
                let mut idx = 0;
                for (i, c) in input_line.chars().enumerate() {
                    if c.is_digit(10) || c.is_alphabetic() {
                        idx = i;
                        matched = true;
                        break;
                    }
                }
                (matched, pattern.get(2..), input_line.get(idx + 1..))
            }
            _ => {
                if input_line.contains(&pattern) {
                    return (
                        true,
                        pattern.get(pattern.len()..),
                        input_line.get(pattern.len()..),
                    );
                } else {
                    return (
                        false,
                        pattern.get(pattern.len()..),
                        input_line.get(pattern.len()..),
                    );
                }
            }
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
                        return (false, pattern.get(pattern.len() + 3..), input_line.get(1..));
                    } else {
                        return (true, pattern.get(pattern.len() + 3..), input_line.get(1..));
                    }
                }
            }

            for c in pattern.chars() {
                if input_line.contains(c) {
                    return (true, pattern.get(pattern.len() + 2..), input_line.get(1..));
                } else {
                    return (false, pattern.get(pattern.len() + 2..), input_line.get(1..));
                }
            }
        }
        if input_line.contains(&pattern) {
            return (
                true,
                pattern.get(pattern.len()..),
                input_line.get(pattern.len()..),
            );
        } else {
            return (
                false,
                pattern.get(pattern.len()..),
                input_line.get(pattern.len()..),
            );
        }
    }
}

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

    let mut res = true;
    let mut p = pattern.as_str();
    let mut i = input_line.as_str();
    while !p.is_empty() && !i.is_empty() {
        let (bool, rest_pattern, rest_input) = reg_match(p, i);
        res = res && bool;
        p = rest_pattern.unwrap_or("");
        i = rest_input.unwrap_or("");
    }
    if res {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
