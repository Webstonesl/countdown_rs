use std::{collections::VecDeque, fmt::Debug, str::FromStr};

use crate::base_types::numbers::CountdownNumberBaseType;

#[derive(Clone, Debug)]
pub enum Token {
    Number(String),
    Punctuation(char),
    Word(String),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    Number,
    Punctuation,
    Word,
    None,
}

pub mod token_reader {
    use super::{Token, TokenType};
    pub fn read(line: String) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut old_type = TokenType::None;
        let mut s = String::new();
        for c in line.chars() {
            let c_type = match c {
                'A'..='Z' | 'a'..='z' => TokenType::Word,
                '0'..='9' => TokenType::Number,
                c if !c.is_whitespace() && c.is_ascii_graphic() => TokenType::Punctuation,
                _ => TokenType::None,
            };
            if old_type != c_type && !s.is_empty() {
                match old_type {
                    TokenType::Number => tokens.push(Token::Number(s)),
                    TokenType::Punctuation => unreachable!(),
                    TokenType::Word => tokens.push(Token::Word(s)),
                    TokenType::None => unreachable!(),
                }
                s = String::new();
            };
            match c_type {
                TokenType::Punctuation => tokens.push(Token::Punctuation(c)),
                TokenType::Number | TokenType::Word => {
                    s.push(c);
                }
                TokenType::None => {}
            };
            old_type = c_type;
        }
        if !s.is_empty() {
            match old_type {
                TokenType::Number => tokens.push(Token::Number(s)),
                TokenType::Word => tokens.push(Token::Word(s)),
                TokenType::Punctuation => unreachable!(),
                TokenType::None => unreachable!(),
            }
        }

        Ok(tokens)
    }
}
pub trait Parsable: Sized {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String>;
}
impl<T: Parsable> Parsable for Vec<T> {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        match tokens.pop_front() {
            Some(Token::Punctuation('[')) => {}
            e => return Err(format!("Expected '[' found {:?}", e)),
        };
        let mut result = Vec::new();
        loop {
            result.push(T::parse(tokens)?);
            match tokens.pop_front() {
                Some(Token::Punctuation(',')) => continue,
                Some(Token::Punctuation(']')) => break,
                a => return Err(format!("expected ']' or ',' found {a:?}")),
            }
        }
        Ok(result)
    }
}
impl<T: FromStr<Err: Debug> + Clone + Debug + CountdownNumberBaseType> Parsable for T {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        match tokens.pop_front() {
            Some(Token::Number(t) | Token::Word(t)) => {
                Ok(T::from_str(&t).map_err(|e| format!("{e:?}"))?)
            }
            a => Err(format!("Expected number found {:?}", a)),
        }
    }
}
