#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use crate::lexer::token_type::Token;
use crate::function::{FunctionCall , FunctionDef , FunctionRet}


#[derive(Debug)]
pub enum Expression {
    
    token(Token),
    fn_definition(FunctionDef),
    fn_call(FunctionCall),
    fn_return(FunctionRet),

    var_definition(),
    var_assignment(),

    iterator(),

    conditional(),
}

impl Clone for Expression {

    fn clone(&self) -> Self {
        match self  {

        Expression::token(a) => Expression::token(a.clone()),
        Expression::fn_definition(a) => Expression::fn_definition(a.clone()),
        Expression::fn_call(a) => Expression::fn_definition(a.clone()),
	    Expression::fn_return(a) => Expression::fn_return(a.clone()) ,
	    
	    Expression::var_defition(a) => Expression::var_definition(a.clone()),
	    Expression::var_assignment(a) => Expression::var_assignment(a.clone())
        }    
    }
}
        
