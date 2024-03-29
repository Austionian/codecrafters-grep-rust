use crate::check::reg_match;

#[derive(Debug)]
pub struct MatchQueue<'a>(pub Vec<MatchType<'a>>);

#[derive(Debug, PartialEq)]
pub enum MatchType<'a> {
    Str(&'a str, Varient),
    Digit(Varient),
    Word(Varient),
    Set {
        pattern: &'a str,
        negated: bool,
        varient: Varient,
    },
    Any,
    Alternation(&'a str, &'a str, Varient),
}

#[derive(Debug, PartialEq)]
pub enum Varient {
    Start,
    End,
    None,
    Plus,
    Question,
}

impl<'a> MatchQueue<'a> {
    pub fn from(pattern: &'a str) -> Self {
        let mut p = pattern;
        let mut match_queue = Vec::new();
        while let Some((m, p_rest)) = get_match_type(p) {
            match_queue.push(m);
            p = p_rest.unwrap_or("");
        }
        MatchQueue(match_queue)
    }

    pub fn check(&self, input: &'a str) -> bool {
        let mut res = true;
        let mut i = input.trim_end();
        for mtch in &self.0 {
            let (bool, rest_input) = reg_match(&mtch, i);
            res = res && bool;
            i = rest_input.unwrap_or("");
            if i == "" {
                if mtch != self.0.last().unwrap() {
                    res = false;
                }
                break;
            }
        }
        res
    }
}

// Returns the pattern, the remaining pattern and its length
fn get_match_type<'a>(pattern: &'a str) -> Option<(MatchType, Option<&'a str>)> {
    if pattern.is_empty() {
        return None;
    }

    if &pattern[0..1] == "." {
        return Some((MatchType::Any, pattern.get(1..)));
    }

    let mut varient = Varient::None;

    let pattern = if pattern.starts_with('^') {
        varient = Varient::Start;
        &pattern[1..]
    } else {
        pattern
    };

    // Checks for word or digit.
    if let Some(s) = pattern.strip_prefix(r"\") {
        let mut i = 2;
        if s.get(1..2).unwrap_or("") == "$" {
            varient = Varient::End;
            i += 1;
        }
        if s.get(1..2).unwrap_or("") == "+" {
            varient = Varient::Plus;
            i += 1;
        }
        if s.get(1..2).unwrap_or("") == "?" {
            varient = Varient::Question;
            i += 1;
        }
        return match s.chars().next() {
            Some('d') => Some((MatchType::Digit(varient), pattern.get(i..))),
            Some('w') => Some((MatchType::Word(varient), pattern.get(i..))),
            // Matches the back tick as a string, be it another backtick or a special
            // character.
            _ => Some((
                MatchType::Str(pattern.get(1..2).unwrap(), varient),
                pattern.get(2..),
            )),
        };
    };

    // Checks for alternation.
    if let Some(s) = pattern.strip_prefix(r"(") {
        let alternation = s.split_once(r")");
        return match alternation {
            Some((pattern, rest)) => {
                if let Some(s) = pattern.split_once(r"|") {
                    return Some((MatchType::Alternation(s.0, s.1, varient), Some(rest)));
                } else {
                    // Invalid alternation type without the pipe.
                    return Some((MatchType::Str(&pattern[..1], varient), Some(&pattern[1..])));
                }
            }
            // Invalid alternation type without the closing paren.
            None => Some((MatchType::Str(&pattern[..1], varient), Some(&pattern[1..]))),
        };
    }

    // Checks for set.
    if let Some(s) = pattern.strip_prefix('[') {
        let set_tuple = s.split_once(']');

        // This is checking whether is this an actual set or just matching on
        // the opening square bracket
        return match set_tuple {
            Some((pattern, rest)) => {
                if rest.chars().next().unwrap_or('a') == '$' {
                    varient = Varient::End;
                }
                if rest.chars().next().unwrap_or('a') == '+' {
                    varient = Varient::Plus;
                }
                if rest.chars().next().unwrap_or('a') == '?' {
                    varient = Varient::Question;
                }
                if pattern.chars().next() == Some('^') {
                    return Some((
                        MatchType::Set {
                            negated: true,
                            pattern: &pattern[1..],
                            varient,
                        },
                        Some(rest),
                    ));
                }
                return Some((
                    MatchType::Set {
                        negated: false,
                        pattern,
                        varient,
                    },
                    Some(rest),
                ));
            }
            None => Some((MatchType::Str(&pattern[..1], varient), Some(&pattern[1..]))),
        };
    }

    // If not above, must be a exact string match.
    if pattern.get(1..2).unwrap_or("") == "+" {
        return Some((
            MatchType::Str(&pattern[0..1], Varient::Plus),
            pattern.get(2..),
        ));
    }
    if pattern.get(1..2).unwrap_or("") == "$" {
        return Some((
            MatchType::Str(&pattern[0..1], Varient::End),
            pattern.get(2..),
        ));
    }
    if pattern.get(1..2).unwrap_or("") == "?" {
        return Some((
            MatchType::Str(&pattern[0..1], Varient::Question),
            pattern.get(2..),
        ));
    }
    return Some((MatchType::Str(&pattern[0..1], varient), pattern.get(1..)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res = get_match_type("");
        assert_eq!(res, None);
    }

    #[test]
    fn digit_match_type() {
        let res = get_match_type(r"\d").unwrap();
        assert_eq!(res.0, MatchType::Digit(Varient::None));
    }

    #[test]
    fn word_match_type() {
        let res = get_match_type(r"\wabc").unwrap();
        assert_eq!(res.0, MatchType::Word(Varient::None));
        assert_eq!(res.1, Some("abc"));
    }

    #[test]
    fn backtick() {
        let res = get_match_type(r"\[").unwrap();
        assert_eq!(res.0, MatchType::Str("[", Varient::None));
        assert_eq!(res.1, Some(""));
    }

    #[test]
    fn set() {
        let res = get_match_type(r"[abc]\d").unwrap();
        assert_eq!(
            res.0,
            MatchType::Set {
                pattern: "abc",
                negated: false,
                varient: Varient::None
            }
        );
        assert_eq!(res.1, Some(r"\d"));
    }

    #[test]
    fn negated_set() {
        let res = get_match_type(r"[^abc]\d").unwrap();
        assert_eq!(
            res.0,
            MatchType::Set {
                pattern: "abc",
                negated: true,
                varient: Varient::None
            }
        );
        assert_eq!(res.1, Some(r"\d"));
    }

    #[test]
    fn empty_set() {
        let res = get_match_type(r"[abe").unwrap();
        assert_eq!(res.0, MatchType::Str("[", Varient::None));
        assert_eq!(res.1, Some("abe"));
    }

    #[test]
    fn varient() {
        let res = get_match_type(r"^abc").unwrap();
        assert_eq!(res.0, MatchType::Str("a", Varient::Start));
        assert_eq!(res.1, Some("bc"));
    }

    #[test]
    fn varient_end() {
        let res = get_match_type(r"a$").unwrap();
        assert_eq!(res.0, MatchType::Str("a", Varient::End));
        assert_eq!(res.1, Some(""));
    }

    #[test]
    fn varient_end_digit() {
        let res = get_match_type(r"\d$").unwrap();
        assert_eq!(res.0, MatchType::Digit(Varient::End));
    }

    #[test]
    fn varient_end_set() {
        let res = get_match_type(r"[abc]$").unwrap();
        assert_eq!(
            res.0,
            MatchType::Set {
                pattern: "abc",
                negated: false,
                varient: Varient::End
            }
        )
    }

    #[test]
    fn string_it() {
        let res = get_match_type(r"abc\d").unwrap();
        assert_eq!(res.0, MatchType::Str("a", Varient::None));
        assert_eq!(res.1, Some(r"bc\d"));
    }

    #[test]
    fn tricky() {
        let res = get_match_type(r"^\dabc").unwrap();
        assert_eq!(res.0, MatchType::Digit(Varient::Start));
        assert_eq!(res.1, Some("abc"))
    }

    #[test]
    fn extra_tricky() {
        let (m, r) = get_match_type(r"^abc\d[^abc]").unwrap();
        assert_eq!(m, MatchType::Str("a", Varient::Start));
        assert_eq!(r, Some(r"bc\d[^abc]"));

        let (m, r) = get_match_type(r.unwrap()).unwrap();
        assert_eq!(m, MatchType::Str("b", Varient::None));
        assert_eq!(r, Some(r"c\d[^abc]"));

        let (m, r) = get_match_type(r.unwrap()).unwrap();
        assert_eq!(m, MatchType::Str("c", Varient::None));
        assert_eq!(r, Some(r"\d[^abc]"));

        let (m, r) = get_match_type(r.unwrap()).unwrap();
        assert_eq!(m, MatchType::Digit(Varient::None));
        assert_eq!(r, Some("[^abc]"));

        let (m, r) = get_match_type(r.unwrap()).unwrap();
        assert_eq!(
            m,
            MatchType::Set {
                pattern: "abc",
                negated: true,
                varient: Varient::None
            }
        );
        assert_eq!(r, Some(""));

        let res = get_match_type(r.unwrap());
        assert_eq!(res, None);
    }
}
