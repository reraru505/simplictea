use std::rc::Rc;
use std::cell::RefCell;

extern crate lexer;
extern crate symboltable;

pub mod expressions;
pub mod parsingdata;

pub mod parser;
pub mod global;

pub mod function;
pub mod function_traits;
pub mod function_args;
pub mod function_return;

pub mod variable;

pub mod binaryexp;
pub mod binaryexp_impl;
pub mod binaryexp_helpers;
pub mod binaryexp_traits;
pub mod binaryexp_handle;

pub mod iterator;
pub mod iterator_traits;

use crate::lexer::lex::Lexeme;
//use crate::lexer::token_type::Token;
//use crate::grammer::*;
//use crate::binaryexp::*;
//use crate::binaryexp_handle::*;
use crate::parsingdata::ParsingData;
use crate::function::{find_all_function_def ,
		      find_all_function_args ,
		      find_all_function_blocks ,
		      hanble_function_args,
		      handle_variable_defs_in_functions };
use crate::iterator::find_iterator_in_scope;

use crate::parser::parser;
pub fn parse(inp_lexemes : Vec<Lexeme> ) -> Vec<ParsingData>{


    let parsingdatavec = ParsingData::generate(inp_lexemes);

    let retval =  parser(parsingdatavec);
    
   for i in retval.clone(){
   	println!("{:#?}\n" , i);
   }

    return retval;
    
}
