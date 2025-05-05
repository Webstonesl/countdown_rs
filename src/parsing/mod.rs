use std::{collections::VecDeque, fmt::Debug, str::FromStr};

use crate::base_types::numbers::NumberType;

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

pub mod token_reader;
pub trait Parsable: Sized {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String>;
}
impl<T: Parsable> Parsable for Vec<T> {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        match tokens.pop_front() {
            | Some(Token::Punctuation('[')) => {}
            | e => return Err(format!("Expected '[' found {:?}", e)),
        };
        let mut result = Vec::new();
        loop {
            result.push(T::parse(tokens)?);
            match tokens.pop_front() {
                | Some(Token::Punctuation(',')) => continue,
                | Some(Token::Punctuation(']')) => break,
                | a => return Err(format!("expected ']' or ',' found {a:?}")),
            }
        }
        Ok(result)
    }
}
impl<T: FromStr<Err: Debug> + Clone + Debug + NumberType> Parsable for T {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, String> {
        match tokens.pop_front() {
            | Some(Token::Number(t) | Token::Word(t)) => {
                Ok(T::from_str(&t).map_err(|e| format!("{e:?}"))?)
            }
            | a => Err(format!("Expected number found {:?}", a)),
        }
    }
}
