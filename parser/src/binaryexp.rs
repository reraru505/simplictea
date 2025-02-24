#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::lexer::token_type::*;
use crate::symboltable::symbol::{DataType , Qualifier};

#[derive(Debug)]
pub enum BinaryExpressionType{
    Assignment_op,
    Addition_op,
    Subtraction_op,
    Multiplication_op,
    Division_op,

    And_op ,
    Or_op  ,
    Xor_op ,

    Check_equal_op  ,
    Not_equal_op    ,
    Greater_than_op ,
    Lesser_than_op  ,
}

#[derive(Debug)]
pub struct BinaryExpression {

    pub exp_value : Option<Token> ,
    pub exp_type : Option<BinaryExpressionType>,
    pub exp_left : Option<Token>,
    pub exp_right :Option<Token> ,
}

#[derive(Debug)]
pub struct BinaryExpressionTree{
    pub tree : Vec<BinaryExpression>,
}



#[derive(Debug)]
pub struct BinaryExpressionBlock{
    pub b_tree : BinaryExpressionTree,
    pub b_type : DataType,
    pub b_qualifier : Qualifier,
}

