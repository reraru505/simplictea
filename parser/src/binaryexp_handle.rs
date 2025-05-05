#![allow(unused_imports)]

use crate::{
    binaryexp::Operation,
    expressions::Expression,
    lexer::token_type::Token ,
    lexer::token_type::{
        STC,
        Operator,
    },
    binaryexp_helpers::get_operator_from_token,
};

pub fn make_operation( invec : Vec<Expression > ) -> Vec<Operation> {

    let mut retval : Vec<Operation> = Vec::new();

    for i in invec.iter() {

        if let Expression::token(tok ) = i.clone(){
           if matches!(tok.clone() , Token::t_identifier(_)){
               retval.push(Operation::operand(tok.clone()));
           }
           if matches!(tok.clone() , Token::t_operator(_)){
               retval.push(Operation::operator(
                   get_operator_from_token(tok)
               ));
           }
        }

    }

    return retval;
}
