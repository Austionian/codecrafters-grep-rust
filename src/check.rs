use crate::matches::{MatchType, Varient};

pub fn reg_match<'a>(pattern: &MatchType, input_line: &'a str) -> (bool, Option<&'a str>) {
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
        MatchType::Alternation(first, second, Varient::Start) => {
            if input_line.starts_with(first) {
                return (true, input_line.get(first.len()..));
            }
            if input_line.starts_with(second) {
                return (true, input_line.get(second.len()..));
            }
            return (false, None);
        }
        MatchType::Alternation(first, second, Varient::None) => {
            if input_line.contains(first) {
                return (true, Some(input_line.split_once(first).unwrap().1));
            }
            if input_line.contains(second) {
                return (true, Some(input_line.split_once(second).unwrap().1));
            }
            return (false, None);
        }
        MatchType::Any => return (true, input_line.get(1..)),
        _ => unimplemented!(),
    }
}
