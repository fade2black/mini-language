use crate::token::Token;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Or,
    And,
    Less,
    Greater,
    Equal,
    NotEq,
    Neg,
}

impl Operator {
    pub fn to_wat(&self) -> String {
        match self {
            Self::Plus => String::from("f32.add\n"),
            Self::Minus => String::from("f32.sub\n"),
            Self::Mul => String::from("f32.mul\n"),
            Self::Div => String::from("f32.div\n"),
            Self::Or => String::from("i32.or\n"),
            Self::And => String::from("i32.and\n"),
            Self::Greater => String::from("f32.gt\n"),
            Self::Less => String::from("f32.lt\n"),
            Self::Equal => String::from("f32.eq\n"),
            Self::NotEq => String::from("f32.ne\n"),
            Self::Neg => String::from("f32.neg\n"),
        }
    }
}

impl From<&Token> for Operator {
    fn from(item: &Token) -> Self {
        match *item {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            Token::Star => Operator::Mul,
            Token::Slash => Operator::Div,
            Token::Or => Operator::Or,
            Token::And => Operator::And,
            Token::Less => Operator::Less,
            Token::Greater => Operator::Greater,
            Token::Equal => Operator::Equal,
            Token::NotEq => Operator::NotEq,
            _ => panic!("Unexpected token {:?}", item),
        }
    }
}
