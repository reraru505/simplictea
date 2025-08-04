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
    CUSTOM(String),
}

#[derive(Debug, Clone)]
pub enum PrefixOP{
    INCREMENT, 
    DECREMENT,
    REFERENCE,
    DREFERENCE,
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

    PREFIX_EXPRESSION{
        operator : PrefixOP,
        right : Box<Node>,
    },
    // exported
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

    RETURN {
       val : Option<Box<Node>>
    },

    // exported
    STRUCTURE{
        name : String ,
        body : Vec<Node>,
    },

    // this is ment as the initializer for all 
    // compound types , inc array , stack , struct
    COMPOUND_INIT {
        args : Vec<Node>
    },

    // exported
    VAR_DEC {
        name : String,
        d_type : Data_Type,
        initializer : Option<Box<Node>>,
    },

    // exported
    FUNCTION_CALL {
        name : String ,
        args : Vec<Node>,
    },

    CONDITIONAL {
        condition : Option<Box<Node>>,
        body : Vec<Node>,
    },

    // exported
    CONDTIONAL_CHAIN (Vec<Node>),

    // exported
    ITERATOR {
        condition : Option<Box<Node>>,
        body : Vec<Node>,
    },

    // exported
    ASSIGNMENT {
        lhs : Box<Node>,
        rhs : Box<Node>,
    },

    ITEM(Vec<String>),

    BREAK,
    NOP,

}
