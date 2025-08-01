#![allow(non_camel_case_types)]

use crate::silexer::silex::Operator;

#[derive(Debug, Clone)]
pub enum Data_Type {
    I32,
    I64 ,
    F32 ,
    F64,
    STRING,
    CHAR,
    STACK,
    POINTER,
}

#[derive(Debug, Clone)]
pub enum Node {
    // rudimentary 
    IDENTIFER(String),
    STRING(String),
    CHAR(char),
    INTEGER(i64),
    FLOAT(f64),

    // aggregate
    BINARY_EXPRESSION{
        operator : Operator,
        left : Box<Node>,
        right : Box<Node>,
    },

    FUNCTION_DEC{
        name : String ,
        r_type : Data_Type,
        params : Vec<Node>,
        body : Vec<Node> ,
    },

    PARAM {
        name : String,
        d_type : Data_Type,
    },

    STRUCTURE{
        name : String ,
        body : Vec<Node>,
    },

    VAR_DEC {
        name : String,
        d_type : Data_Type,
        initializer : Option<Box<Node>>,
    },

    FUNCTION_CALL {
        name : String ,
        args : Vec<Node>,
    },

    CONDITIONAL {
        condition : Box<Node>,
        body : Vec<Node>,
        next : Option<Box<Node>>,
    },

    ITERATOR {
        condition : Option<Box<Node>>,
        body : Vec<Node>,
    },

    ASSIGNMENT {
        lhs : Box<Node>,
        rhs : Box<Node>,
    },

}
