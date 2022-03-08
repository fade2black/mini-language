#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Comma,
    Define,
    Identifier,
    Number,
    InvalidChar,
    If,
    Else,
    Then,
    Lpar,
    Rpar,
    Equal,
    Less,
    Greater,
    Plus,
    Minus,
    Star,
    Slash,
    Or,
    And,
    Semicolon,
    None,
    NotEq,
}

impl Token {
    pub fn is_addition_operator(&self) -> bool {
        match *self {
            Token::Plus | Token::Minus | Token::Or => true,
            _ => false,
        }
    }
}

impl Token {
    pub fn is_multiplication_operator(&self) -> bool {
        match *self {
            Token::Star | Token::Slash | Token::And => true,
            _ => false,
        }
    }
}

impl Token {
    pub fn is_comparison_operator(&self) -> bool {
        match *self {
            Token::Less | Token::Greater | Token::Equal | Token::NotEq => true,
            _ => false,
        }
    }
}
