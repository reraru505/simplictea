use std::cell::RefCell;
use std::rc::Rc;

pub mod parsing_symbols;

pub mod binaryexp;
pub mod binaryexp_handle;
pub mod binaryexp_helpers;
pub mod binaryexp_traits;

pub mod expressions;
pub mod collector;

pub mod grammer;

pub mod function;
pub mod function_grammer;

pub mod parser;
pub mod scope;

extern crate lexer;
extern crate symboltable;

use crate::lexer::lex::Lexeme;
use crate::parser::parser;
use expressions::Expression;

pub fn parse(inp_lexemes: Vec<Lexeme>) -> Vec<Expression> {
    parser(inp_lexemes)
}
