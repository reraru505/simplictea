#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use symboltable::symbol::DataType;

use crate::expressions::Expression;
use crate::lexer::token_type::Token;
use crate::lexer::token_type::{
    Data_Type,
    Statement,
    Keyword,
    Literal,
    STC,
    Operator
};
    
pub enum Grammer {

    function_def_args(Vec<Expression>),
    function_def_no_args(Vec<Expression>),

    function_call(Vec<Expression>),

    function_return(Vec<Expression>),

    variable_def(Vec<Expression>),
    variable_def_value(Vec<Expression>),

    variable_assignment(Vec<Expression>),

    iterator_over_range(Vec<Expression>),
    iterator_over_value(Vec<Expression>),
    iterator_over_condition(Vec<Expression>),

    condition_if(Vec<Expression>),
    condition_else_if(Vec<Expression>),
    condition_else(Vec<Expression>),
    
}

impl Grammer{

    pub fn get(&self ) -> Grammer {

        match self {
            Grammer::function_def_args(_) => return Grammer::function_def_args(vec![
                Expression::token(Token::t_identifier("".to_string())),
                Expression::token(Token::t_operator(Operator::type_assignment_op("".to_string()))),
                Expression::token(Token::t_keyword(Keyword::data_type(Data_Type::VOID("".to_string())))),
            ])
        }
    }
}

