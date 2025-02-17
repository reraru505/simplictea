#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use crate::binaryexp::*;
use crate::lexer::token_type::Token;
use crate::symboltable::symbol::DataType;
use crate::symboltable::symbol::Qualifier;


pub struct Scope{
    scope : Vec<String>,
}
#[derive(Debug)]
pub enum VAR_Definition_Type{
    def_with_type,
    def_with_type_as_arg,
    def_with_type_value,
    def_with_infered_value,
}



//pub struct ARGS{
//    
//}
//
//
//#[derive(Debug)]
//pub struct VAR_Definition{
//    def_type : VAR_Definition_Type ,
//    data_type : Token,
//    data_id : Token,
//    data_value : Token,
//}
//
//
//#[derive(Debug)]
//pub enum Expression{
//    binary_exp(BinaryExpressionBlock),
//    var_definition(VAR_Definition),
////    fn_definition(FN_Definition),
////    if_statement(IF_Statement),
////    else_if_statement(ELSE_IF_Statement),
////    else_statement(ELSE_Statement),
////    for_statement(FOR_Statement),
//    scope_block(Block),
//    return_statement(BinaryExpressionBlock),
//}
