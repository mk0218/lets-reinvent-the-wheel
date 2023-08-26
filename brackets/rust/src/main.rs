use std::fmt;

fn main() {
    test("()");
    test("{");
    test("}");
    test_non_bracket("abc");
}

fn test(s: &str) {
    match parse(s) {
        Ok(_) => println!("pass"),
        Err(why) => println!("fail: {}", why),
    }
}

fn test_non_bracket(s: &str) {
    let mut ckr = BracketsCheck::new();
    for c in s.chars() {
        if let Err(why) = ckr.check(c) {
            println!("fail: {}", why);
            return;
        }
    }
    match ckr.end() {
        Ok(_) => println!("pass"),
        Err(why) => println!("pass: {}", why),
    };
}

fn parse(s: &str) -> Result<(), BracketCheckError> {
    let mut ckr = BracketsCheck::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' | ')' | '}' | ']' => ckr.check(c),
            _ => continue,
        }?
    }
    return Ok(ckr.end()?);
}

enum BracketCheckError {
    InvalidCharacter,
    UnMatchedRightBracket,
    UnmatchedLeftBracket,
}

impl fmt::Display for BracketCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BracketCheckError::InvalidCharacter =>
                write!(f, "Not a bracket."),
            BracketCheckError::UnMatchedRightBracket =>
                write!(f, "The right bracket do not matches."),
            BracketCheckError::UnmatchedLeftBracket =>
                write!(f, "Unmatched left bracket remains!"),
        }
    }
}

use BracketCheckError::*;

struct BracketsCheck(Vec<char>);

impl BracketsCheck {
    pub fn new() -> BracketsCheck {
        return BracketsCheck(Vec::new());
    }

    pub fn check(&mut self, b: char) -> Result<(), BracketCheckError> {
        return match b {
            '(' | '{' | '[' => { self.push(b); Ok(()) },
            ')' | '}' | ']' => self.check_match(b),
            _ => return Err(InvalidCharacter),
        };
    }

    pub fn end(self) -> Result<(), BracketCheckError> {
        return if self.0.is_empty() {
            Ok(())
        } else {
            Err(UnmatchedLeftBracket)
        };
    }

    fn check_match(&mut self, rb: char) -> Result<(), BracketCheckError> {
        if let Some(lb) = self.pop() {
            return match (lb, rb) {
                ('(', ')') | ('{', '}') | ('[', ']') => Ok(()),
                _ => { self.push(lb); Err(UnMatchedRightBracket) },
            }
        }
        return Err(UnMatchedRightBracket);
    }

    fn push(&mut self, b: char) {
        return self.0.push(b);
    }

    fn pop(&mut self) -> Option<char> {
        return self.0.pop();
    }
}