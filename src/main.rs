use std::env;
use std::io;
use std::process;

fn string_match<'a>(
    pattern: &'a str,
    input_line: &'a str,
) -> (bool, Option<&'a str>, Option<&'a str>, Option<usize>) {
    let end_condition = pattern.split_at(pattern.len() - 1).1 == "$";
    if end_condition {
        let pattern = pattern.split_at(pattern.len() - 1).0;
        if input_line.contains(&pattern) {
            return (
                true,
                pattern.get(pattern.len()..),
                input_line.get(pattern.len()..),
                input_line.starts_with(pattern).then(|| 0),
            );
        } else {
            return (
                false,
                pattern.get(pattern.len()..),
                input_line.get(pattern.len()..),
                None,
            );
        }
    } else {
        if input_line.contains(&pattern) {
            return (
                true,
                pattern.get(pattern.len()..),
                input_line.get(pattern.len()..),
                input_line.starts_with(pattern).then(|| 0),
            );
        } else {
            return (
                false,
                pattern.get(pattern.len()..),
                input_line.get(pattern.len()..),
                None,
            );
        }
    }
}

fn reg_match<'a>(
    pattern: &'a str,
    input_line: &'a str,
) -> (bool, Option<&'a str>, Option<&'a str>, Option<usize>) {
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
                (
                    matched,
                    pattern.get(2..),
                    input_line.get(idx + 1..),
                    Some(idx),
                )
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
                (
                    matched,
                    pattern.get(2..),
                    input_line.get(idx + 1..),
                    Some(idx),
                )
            }
            _ => string_match(pattern, input_line),
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
                        return (
                            false,
                            pattern.get(pattern.len() + 3..),
                            input_line.get(1..),
                            None,
                        );
                    } else {
                        return (
                            true,
                            pattern.get(pattern.len() + 3..),
                            input_line.get(1..),
                            Some(0),
                        );
                    }
                }
            }

            for c in pattern.chars() {
                if input_line.contains(c) {
                    return (
                        true,
                        pattern.get(pattern.len() + 2..),
                        input_line.get(1..),
                        input_line.starts_with(c).then(|| 0),
                    );
                } else {
                    return (
                        false,
                        pattern.get(pattern.len() + 2..),
                        input_line.get(1..),
                        None,
                    );
                }
            }
        }
        if pattern.chars().next() == Some('^') {
            let pattern = pattern.strip_prefix('^').unwrap();
            // check if there's a match and if it's in the first position
            let (m, _, _, starts_with) = reg_match(pattern, input_line);
            if m && starts_with.unwrap_or(1) == 0 {
                return (true, pattern.get(1..), Some(input_line), Some(0));
            } else {
                return (false, pattern.get(1..), Some(input_line), None);
            }
        }
        // Pattern has special chars later on
        if pattern.contains('\\') {
            let idx = pattern.chars().position(|c| c == '\\').unwrap();
            let (character_pattern, rest) = pattern.split_at(idx);
            if input_line.starts_with(character_pattern) {
                return (
                    true,
                    Some(rest),
                    input_line.get(character_pattern.len()..),
                    Some(0),
                );
            } else {
                return (
                    false,
                    Some(rest),
                    input_line.get(character_pattern.len()..),
                    None,
                );
            }
        }
        string_match(pattern, input_line)
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
        let (bool, rest_pattern, rest_input, _) = reg_match(p, i);
        res = res && bool;
        p = rest_pattern.unwrap_or("");
        i = rest_input.unwrap_or("");
        if i.is_empty() && !p.is_empty() {
            res = false;
        }
    }
    if res {
        println!("pass");
        process::exit(0);
    } else {
        println!("fail");
        process::exit(1);
    }
}
