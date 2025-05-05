#![allow(non_camel_case_types)]

use crate::lexer::token_type::Token;

#[derive(Debug)]
pub enum Operator {
    ADDITION,
    SUBTRACTION,
    DIVISION,
    MULTIPLICATION,

    ASSIGNMENT,

    AND,
    OR,
    NOT,

    CHECKEQUAL,
    CHECKNEQUAL,

    GREATER,
    LESSER,

    BRACKET,
}


#[derive(Debug)]
pub enum Operation {
    operand(Token),
    operator(Operator),
    subexp(usize),
}



#[derive(Debug)]
pub struct BinaryExpression {
    pub super_scope: String,
    pub tree: Vec<SingleExpression>,
}



#[derive(Debug)]
pub struct SingleExpression {
    pub id: Token,
    pub operator: Operator,
    pub left: Token,
    pub right: Token,
}


