mod matches;

use matches::{get_match_type, MatchType, Varient};
use std::env;
use std::io;
use std::process;

fn reg_match<'a>(pattern: &MatchType, input_line: &'a str) -> (bool, Option<&'a str>) {
    match pattern {
        MatchType::Word(Varient::Start) => (
            input_line.chars().next().unwrap().is_alphanumeric(),
            input_line.get(1..),
        ),
        MatchType::Word(Varient::None) => {
            for (i, c) in input_line.chars().enumerate() {
                if c.is_alphanumeric() {
                    return (true, input_line.get(i + 1..));
                }
            }
            return (false, None);
        }
        MatchType::Word(Varient::End) => (
            input_line.chars().last().unwrap().is_alphanumeric(),
            input_line.get(0..input_line.len() - 1),
        ),
        MatchType::Word(Varient::Plus) => {
            let mut res = false;
            for (i, c) in input_line.chars().enumerate() {
                if c.is_alphanumeric() {
                    res = true;
                } else {
                    if res {
                        return (true, input_line.get(i - 1..));
                    }
                }
            }
            if res {
                return (true, None);
            }
            return (false, None);
        }
        MatchType::Digit(Varient::Start) => (
            input_line.chars().next().unwrap().is_digit(10),
            input_line.get(1..),
        ),
        MatchType::Digit(Varient::None) => {
            for (i, c) in input_line.chars().enumerate() {
                if c.is_digit(10) {
                    return (true, input_line.get(i + 1..));
                }
            }
            return (false, None);
        }
        MatchType::Digit(Varient::End) => (
            input_line.chars().last().unwrap().is_digit(10),
            input_line.get(0..input_line.len() - 1),
        ),
        MatchType::Digit(Varient::Plus) => {
            let mut res = false;
            for (i, c) in input_line.chars().enumerate() {
                if c.is_digit(10) {
                    res = true;
                } else {
                    if res {
                        return (true, input_line.get(i - 1..));
                    }
                }
            }
            if res {
                return (true, None);
            }
            return (false, None);
        }
        MatchType::Set {
            pattern,
            negated: true,
            varient: Varient::Start,
        } => {
            for c in pattern.chars() {
                if input_line.chars().next().unwrap() == c {
                    return (false, None);
                }
            }
            return (true, input_line.get(1..));
        }
        MatchType::Set {
            pattern,
            negated: false,
            varient: Varient::Start,
        } => {
            for c in pattern.chars() {
                if input_line.chars().next().unwrap() == c {
                    return (true, input_line.get(1..));
                }
            }
            return (false, None);
        }
        MatchType::Set {
            pattern,
            negated: true,
            varient: Varient::None,
        } => {
            for c in pattern.chars() {
                if input_line.contains(c) {
                    return (false, None);
                }
            }
            return (true, input_line.get(1..));
        }
        MatchType::Set {
            pattern,
            negated: false,
            varient: Varient::None,
        } => {
            for (i, c) in pattern.chars().enumerate() {
                if input_line.contains(c) {
                    return (true, input_line.get(i + 1..));
                }
            }
            return (false, None);
        }
        MatchType::Str(p, Varient::Start) => (input_line.starts_with(p), input_line.get(1..)),
        MatchType::Str(p, Varient::None) => {
            if input_line.contains(p) {
                return (true, Some(input_line.split_once(p).unwrap().1));
            }
            return (false, None);
        }
        MatchType::Str(p, Varient::End) => {
            if input_line.contains(p)
                && input_line
                    .split(p)
                    .collect::<Vec<_>>()
                    .last()
                    .is_some_and(|last| last.is_empty())
            {
                return (true, None);
            }
            return (false, None);
        }
        MatchType::Str(p, Varient::Plus) => {
            if input_line.contains(p) {
                let x = input_line.split_once(p).unwrap();
                for (i, c) in x.1.chars().enumerate() {
                    if &c.to_string().as_str() == p {
                        continue;
                    }
                    return (true, input_line.get(i..));
                }
                return (true, None);
            }
            return (false, None);
        }
        MatchType::Str(p, Varient::Question) => {
            if input_line.contains(p) {
                let x = input_line.split_once(p).unwrap();
                for (i, c) in x.1.chars().enumerate() {
                    if &c.to_string().as_str() == p {
                        continue;
                    }
                    return (true, input_line.get(i..));
                }
                return (true, None);
            }
            return (true, Some(input_line));
        }
        MatchType::Any => return (true, input_line.get(1..)),
        _ => unimplemented!(),
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
    let mut i = input_line.as_str().trim_end();
    let mut match_queue = Vec::new();
    // Parse the expression into match types.
    while let Some((m, p_rest)) = get_match_type(p) {
        match_queue.push(m);
        p = p_rest.unwrap_or("");
    }
    for mtch in &match_queue {
        let (bool, rest_input) = reg_match(&mtch, i);
        res = res && bool;
        i = rest_input.unwrap_or("");
        if i == "" {
            if mtch != match_queue.last().unwrap() {
                res = false;
            }
            break;
        }
    }
    // if m.is_end_varient() && !first && i.is_empty() && !p.is_empty() {
    //     res = false;
    // }
    // while !i.is_empty() {
    //         let (bool, rest_input) = reg_match(&m, i);
    //         res = res && bool;
    //         i = rest_input.unwrap_or("");
    //         first = false;
    //     } else {
    //         break;
    //     }
    // }
    if res {
        println!("pass");
        process::exit(0);
    } else {
        println!("fail");
        process::exit(1);
    }
}
