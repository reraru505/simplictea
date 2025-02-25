extern crate lexer;
extern crate symboltable;

pub mod error_handler;
pub mod decimal;
pub mod lextypechecker;
pub mod double_operator;
pub mod key_operators;
pub mod if_else;

use crate::lexer::lex::Lexeme;
use crate::decimal::merge_decimal;
use crate::error_handler::ErrorQueue;
use crate::double_operator::merge_operators;
use crate::key_operators::find_operators_as_keywords;
use crate::if_else::find_else_if_statement;

pub fn validate(file_path : String , lexvec : Vec<Lexeme>) -> Vec<Lexeme> {

    let mut error_handler = ErrorQueue::new(file_path);
    
    let retval =  merge_decimal(lexvec , &mut error_handler );
    let retval =  merge_operators(retval , &mut error_handler );
    let retval =  find_operators_as_keywords(retval);
    let retval =  find_else_if_statement(retval);

    error_handler.display_if_error();

    return retval;

}

