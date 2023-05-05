use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Name(String),
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Times,
    Divide,
    Power,
    Illegal(String),
    EoI,
}

impl Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Number(f) => write!(fmt, "{}", f),
            Token::Name(s) => write!(fmt, "{}", s),
            Token::OpenParenthesis => write!(fmt, "("),
            Token::CloseParenthesis => write!(fmt, ")"),
            Token::Plus => write!(fmt, "+"),
            Token::Minus => write!(fmt, "-"),
            Token::Times => write!(fmt, "*"),
            Token::Divide => write!(fmt, "/"),
            Token::Power => write!(fmt, "^"),
            Token::Illegal(s) => write!(fmt, "Illegal Token: {}", s),
            Token::EoI => write!(fmt, "End of Input"),
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}
