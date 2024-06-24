use std::result;

use crate::token_types::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

type ExprBox = Box<Expression>;
type ExprList = Vec<Expression>;

#[derive(Debug)]
pub enum Expression {
    Unparsed(Vec<Token>),
    // just enough for goal-1.simple
    GetVariable(String),
    Call(ExprBox, ExprList),
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
        }
    }

    pub fn parse(&self) -> ExprList {
        use Expression::*;

        let tokens = self.tokens.clone();
        let result: ExprList = vec![Unparsed(tokens)];

        result
    }
}
