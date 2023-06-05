use std::vec;
use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

   pub fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }

    pub fn clone(&self) -> Token {
        Token {
            token_type: self.token_type.clone(),
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line: self.line.clone()
        }
    }
}

