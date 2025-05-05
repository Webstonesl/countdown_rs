use super::{Token, TokenType};
pub fn read(line: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut old_type = TokenType::None;
    let mut s = String::new();
    for c in line.chars() {
        let c_type = match c {
            | 'A'..='Z' | 'a'..='z' => TokenType::Word,
            | '0'..='9' => TokenType::Number,
            | c if !c.is_whitespace() && c.is_ascii_graphic() => {
                TokenType::Punctuation
            }
            | _ => TokenType::None,
        };
        if old_type != c_type && !s.is_empty() {
            match old_type {
                | TokenType::Number => tokens.push(Token::Number(s)),
                | TokenType::Punctuation => unreachable!(),
                | TokenType::Word => tokens.push(Token::Word(s)),
                | TokenType::None => unreachable!(),
            }
            s = String::new();
        };
        match c_type {
            | TokenType::Punctuation => tokens.push(Token::Punctuation(c)),
            | TokenType::Number | TokenType::Word => {
                s.push(c);
            }
            | TokenType::None => {}
        };
        old_type = c_type;
    }
    if !s.is_empty() {
        match old_type {
            | TokenType::Number => tokens.push(Token::Number(s)),
            | TokenType::Word => tokens.push(Token::Word(s)),
            | TokenType::Punctuation => unreachable!(),
            | TokenType::None => unreachable!(),
        }
    }

    Ok(tokens)
}
