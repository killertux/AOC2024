use std::fs::read_to_string;
use std::iter::Peekable;
use std::str::Chars;

fn main() -> std::io::Result<()> {
    part_1("input.txt")?;
    part_2("input.txt")?;
    Ok(())
}

fn part_1(file_name: &str) -> std::io::Result<()> {
    let input = read_to_string(file_name)?;
    let mut tokenizer = Tokenizer::new(&input).peekable();
    let mut result = 0;
    while let Some(token) = tokenizer.next() {
        if token == Token::Mul {
            if tokenizer.next_if_eq(&Token::LeftParen).is_none() {
                continue;
            }
            let Some(Token::Number(a)) = tokenizer.next_if(|t| matches!(t, Token::Number(_)))
            else {
                continue;
            };
            if tokenizer.next_if_eq(&Token::Comma).is_none() {
                continue;
            }
            let Some(Token::Number(b)) = tokenizer.next_if(|t| matches!(t, Token::Number(_)))
            else {
                continue;
            };
            if tokenizer.next_if_eq(&Token::RightParen).is_none() {
                continue;
            }
            result += a * b;
        }
    }
    println!("Result {result}");
    Ok(())
}

fn part_2(file_name: &str) -> std::io::Result<()> {
    let input = read_to_string(file_name)?;
    let parser = Parser::new(&input);
    let mut result = 0;
    for ast in parser {
        match ast {
            Ast::Mul(a, b) => result += a * b,
            Ast::Do => {}
        }
    }
    println!("Result {result}");
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Ast {
    Mul(i64, i64),
    Do,
}

struct Parser<'s> {
    tokenizer: Peekable<Tokenizer<'s>>,
    processing_dont: bool,
}

impl<'s> Parser<'s> {
    pub fn new(input: &'s str) -> Self {
        Self {
            tokenizer: Tokenizer::new(input).peekable(),
            processing_dont: false,
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = Ast;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tokenizer.next() {
                None => return None,
                Some(Token::Mul) => {
                    if self.tokenizer.next_if_eq(&Token::LeftParen).is_none() {
                        continue;
                    }
                    let Some(Token::Number(a)) =
                        self.tokenizer.next_if(|t| matches!(t, Token::Number(_)))
                    else {
                        continue;
                    };
                    if self.tokenizer.next_if_eq(&Token::Comma).is_none() {
                        continue;
                    }
                    let Some(Token::Number(b)) =
                        self.tokenizer.next_if(|t| matches!(t, Token::Number(_)))
                    else {
                        continue;
                    };
                    if self.tokenizer.next_if_eq(&Token::RightParen).is_none() {
                        continue;
                    }
                    return Some(Ast::Mul(a, b));
                }
                Some(Token::Do) => {
                    if self.tokenizer.next_if_eq(&Token::LeftParen).is_none() {
                        continue;
                    }
                    if self.tokenizer.next_if_eq(&Token::RightParen).is_none() {
                        continue;
                    }
                    return Some(Ast::Do);
                }
                Some(Token::Dont) if !self.processing_dont => {
                    if self.tokenizer.next_if_eq(&Token::LeftParen).is_none() {
                        continue;
                    }
                    if self.tokenizer.next_if_eq(&Token::RightParen).is_none() {
                        continue;
                    }
                    self.processing_dont = true;
                    loop {
                        let Some(ast) = self.next() else {
                            break;
                        };
                        if Ast::Do == ast {
                            break;
                        }
                    }
                    self.processing_dont = false;
                    return self.next();
                }
                _ => continue,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Mul,
    Do,
    Dont,
    LeftParen,
    RightParen,
    Number(i64),
    Comma,
    Ignored,
}

struct Tokenizer<'s> {
    chars: Peekable<Chars<'s>>,
}

impl<'s> Tokenizer<'s> {
    pub fn new(input: &'s str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                None => return None,
                Some('(') => return Some(Token::LeftParen),
                Some(')') => return Some(Token::RightParen),
                Some(',') => return Some(Token::Comma),
                Some('m') => {
                    if self.chars.next_if_eq(&'u').is_none() {
                        continue;
                    }
                    if self.chars.next_if_eq(&'l').is_none() {
                        continue;
                    }
                    return Some(Token::Mul);
                }
                Some('d') => {
                    if self.chars.next_if_eq(&'o').is_none() {
                        continue;
                    }
                    if self.chars.next_if_eq(&'n').is_none() {
                        return Some(Token::Do);
                    }
                    if self.chars.next_if_eq(&'\'').is_none() {
                        return Some(Token::Do);
                    }
                    if self.chars.next_if_eq(&'t').is_none() {
                        return Some(Token::Do);
                    }
                    return Some(Token::Dont);
                }
                Some(c)
                    if c.is_ascii_digit()
                        || c == '-'
                            && self
                                .chars
                                .peek()
                                .map(|c| c.is_ascii_digit())
                                .unwrap_or(false) =>
                {
                    let mut result_as_string = String::new();
                    result_as_string.push(c);
                    while let Some(c) = self.chars.next_if(|c| c.is_ascii_digit()) {
                        result_as_string.push(c);
                    }
                    return Some(Token::Number(
                        result_as_string.parse().expect("Error parsing number"),
                    ));
                }
                _ => return Some(Token::Ignored),
            }
        }
    }
}
