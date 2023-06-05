use std::fmt;
use std::fmt::{Formatter, write};

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

    Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl TokenType{
    pub fn clone(&self) -> TokenType {
        match self {
            TokenType::LeftParen => TokenType::LeftParen,
            TokenType::RightParen => TokenType::RightParen,
            TokenType::LeftBrace => TokenType::LeftBrace,
            TokenType::RightBrace => TokenType::RightBrace,
            TokenType::Comma => TokenType::Comma,
            TokenType::Dot => TokenType::Dot,
            TokenType::Minus => TokenType::Minus,
            TokenType::Plus => TokenType::Plus,
            TokenType::Semicolon => TokenType::Semicolon,
            TokenType::Slash => TokenType::Slash,
            TokenType::Star => TokenType::Star,
            TokenType::Bang => TokenType::Bang,
            TokenType::BangEqual => TokenType::BangEqual,
            TokenType::Equal => TokenType::Equal,
            TokenType::EqualEqual => TokenType::EqualEqual,
            TokenType::Greater => TokenType::Greater,
            TokenType::GreaterEqual => TokenType::GreaterEqual,
            TokenType::Less => TokenType::Less,
            TokenType::LessEqual => TokenType::LessEqual,
            TokenType::Identifier => TokenType::Identifier,
            TokenType::String => TokenType::String,
            TokenType::Number => TokenType::Number,
            TokenType::And => TokenType::And,
            TokenType::Class => TokenType::Class,
            TokenType::Else => TokenType::Else,
            TokenType::False => TokenType::False,
            TokenType::Fun => TokenType::Fun,
            TokenType::For => TokenType::For,
            TokenType::If => TokenType::If,
            TokenType::Nil => TokenType::Nil,
            TokenType::Or => TokenType::Or,
            TokenType::Print => TokenType::Print,
            TokenType::Return => TokenType::Return,
            TokenType::Super => TokenType::Super,
            TokenType::This => TokenType::This,
            TokenType::True => TokenType::True,
            TokenType::Var => TokenType::Var,
            TokenType::While => TokenType::While,
            TokenType::Eof => TokenType::Eof
        }
    }
}