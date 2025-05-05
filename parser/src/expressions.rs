#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use crate::lexer::token_type::Token;
use crate::function::{FunctionCall , FunctionDef , FunctionRet}
use crate::binaryexp::BinaryExpression;

#[derive(Debug)]
pub enum Expression {
    
    token(Token),
    fn_definition(FunctionDef),
    fn_call(FunctionCall),
    fn_return(FunctionRet),
    binary_exp(BinaryExpression),
}

impl Clone for Expression {

    fn clone(&self) -> Self {
        match self  {

        Expression::token(a) => return Expression::token(a.clone()),
        Expression::fn_definition(a) => Expression::fn_definition(a.clone()),
        Expression::fn_call(a) => Expression::fn_call(a.clone()),
	    Expression::fn_return(a) => return Expression::fn_return(a.clone()) ,
	    Expression::binary_exp(a) => return Expression::binary_exp(a.clone())
        }    
    }
}
        
