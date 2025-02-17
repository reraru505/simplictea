#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::lexer::token_type::*;

#[derive(Debug)]
pub enum DataType{
    I32,
    I64,
    F32,
    F64,
    CHAR,
    STRING,
    VOID,
}

#[derive(Debug)]
pub enum Qualifier{
    CONSTANT,
    VARIABLE,
    FUNCTION , // returns will be handled by C for now 
}


pub struct Symbol{
    symbol : Token,
    scope : String ,
    datatype : DataType,
    qualifier : Qualifier,
}

pub struct SymbolTable{
    table : Vec<Symbol>,
}

impl SymbolTable {

    pub fn new() -> Self{

	Self{
	    
	    table : Vec::new(),
	}
	
    }
    
}
