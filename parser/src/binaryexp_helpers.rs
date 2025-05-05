#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use lexer::operator::Operator_Checker;

use crate::binaryexp::Operator;
use std::process;

use crate::lexer::token_type::{
    Token ,
    Operator as T_operator,
};

pub fn get_operator_from_token(tok : Token ) -> Operator {

    match tok {
        Token::t_operator(T_operator::assignment_op(_)) => return Operator::ASSIGNMENT,
        Token::t_operator(T_operator::addition_op(_)) => return Operator::ADDITION,
        Token::t_operator(T_operator::subtraction_op(_)) => return Operator::SUBTRACTION,
        Token::t_operator(T_operator::multiplication_op(_)) => return Operator::MULTIPLICATION,
        Token::t_operator(T_operator::division_op(_)) => return Operator::DIVISION,

        Token::t_operator(T_operator::not_op(_)) => return Operator::NOT,
        Token::t_operator(T_operator::check_equal_op(_)) => return Operator::CHECKEQUAL,
        Token::t_operator(T_operator::not_equal_op(_)) => return Operator::CHECKEQUAL,
        Token::t_operator(T_operator::greater_than_op(_)) => return Operator::GREATER,
        Token::t_operator(T_operator::lesser_than_op(_)) => return Operator::LESSER,
        Token::t_operator(T_operator::and_op(_)) => return Operator::AND,
        Token::t_operator(T_operator::or_op(_)) => return Operator::OR,

        

        _ => {
            println!("The argument passed to function [get_operator_from_token ] is not an operator");
            process::exit(-1);
        },
        
    }

}
