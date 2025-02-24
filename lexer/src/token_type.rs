

#![allow(non_camel_case_types)]
#![allow(unused_variables)]

#[derive(Debug)]
pub struct Position{

    pub x : usize ,
    pub y : usize ,
}


#[derive(Debug)]
pub enum Data_Type{
    CHAR(String) ,
    VOID(String) ,
    I32(String)  ,
    I64(String)  ,
    F32(String)  ,
    F64(String)  ,
    STRING(String) ,
}

#[derive(Debug)]
pub enum Statement{
    function_marker(String),
    
    if_statement(String),
    else_statement(String),
    
    return_statement(String),
    
    for_statement(String),
    in_statement(String),
    to_statement(String),

    
    //logical operators are handled as keywords 
    and_operator(String),        // and
    or_operator(String),	// or
    xor_operator(String),	// xor
    
} 

#[derive(Debug)]
pub enum Keyword{

    data_type(Data_Type),
    statement(Statement),
}


#[derive(Debug)]
pub enum Literal{
    character_literal(String) ,
    string_literal(String) ,
    integer_literal(String),
    decimal_literal(String),
}

#[derive(Debug)]
pub enum STC {
    stc_scope_begin(String),    // {
    stc_scope_end(String),      // }

    stc_comma_seperator(String),
    stc_end_expression(String), // ;

    stc_arg_begin(String), // (
    stc_arg_end(String),   // )

    stc_dot(String),
}

#[derive(Debug)]
pub enum Operator{

    assignment_op(String),      // =
    type_assignment_op(String), // :
    
    addition_op(String),        // +
    subtraction_op(String),     // -
    multiplication_op(String),  // *
    division_op(String),        // /

    //comparision operatos
    not_op(String),             // !
    check_equal_op(String),     // ==
    not_equal_op(String),       // !=
    greater_than_op(String),    // >
    lesser_than_op(String),     // <

    
    //logical operators  
    and_op(String),     // and
    or_op(String),	// or
    xor_op(String),	// xor

    
}


#[derive(Debug)]
pub enum Token{

    t_keyword(Keyword),
    t_literal(Literal),
    t_stc(STC),
    t_operator(Operator),
    t_identifier(String),

}



