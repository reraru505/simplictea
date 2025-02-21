extern crate lexer;
extern crate symboltable;

pub mod error_handler;
pub mod decimal;
pub mod lextypechecker;

use crate::lexer::lex::Lexeme;
use crate::decimal::merge_decimal;
use crate::error_handler::ErrorQueue;

pub fn validate(file_path : String , lexvec : Vec<Lexeme>) -> Vec<Lexeme> {

    let mut error_handler = ErrorQueue::new(file_path);
    
    let retval =  merge_decimal(lexvec , &mut error_handler );

    error_handler.display_if_error();

    return retval;

}

