#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::lexer::token_type::*;

#[derive(Debug)]
pub enum DataType {
    I32,
    I64,
    F32,
    F64,
    CHAR,
    STRING,
    VOID,
}

#[derive(Debug)]
pub enum Qualifier {
    CONSTANT,
    VARIABLE,
    FUNCTION, // returns will be handled by C for now
}

pub struct Symbol {
    symbol: Token,
    scope: String,
    datatype: DataType,
    qualifier: Qualifier,
}

pub struct SymbolTable {
    table: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { table: Vec::new() }
    }
}

pub fn get_data_type_from_token(tok: Token) -> DataType {
    if let Token::t_keyword(Keyword::data_type(dat)) = tok {
        match dat {
            Data_Type::CHAR(_) => return DataType::CHAR,
            Data_Type::VOID(_) => return DataType::VOID,
            Data_Type::I32(_) => return DataType::I32,
            Data_Type::I64(_) => return DataType::I64,
            Data_Type::F32(_) => return DataType::F32,
            Data_Type::F64(_) => return DataType::F64,
            Data_Type::STRING(_) => return DataType::STRING,
        }
    }

    //this is a guraded function and such case will never happen
    eprintln!("\n\nThe guarded section of the function was called , this case cannot happem \n\n");

    return DataType::VOID;
}
