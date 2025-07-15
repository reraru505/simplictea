#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::token_type::*;
use std::fmt;
impl PartialEq for Literal {

    fn eq(&self , other : &Self) -> bool{

	match (self , other){
	    (Literal::character_literal(a) ,Literal::character_literal(b) ) => return true ,
	    (Literal::string_literal(a) ,Literal::string_literal(b) ) => return true ,
	    (Literal::integer_literal(a) ,Literal::integer_literal(b) ) => return true ,
	    (Literal::decimal_literal(a) ,Literal::decimal_literal(b) ) => return true ,
	    _ => return false,
	}
	
    }
}

impl Clone for Literal {

    fn clone(&self ) -> Self{

	match self {
	    Literal::character_literal(a)  => Literal::character_literal(a.clone())  ,
	    Literal::string_literal(a)     => Literal::string_literal(a.clone())     ,
	    Literal::integer_literal(a)    => Literal::integer_literal(a.clone())    ,
	    Literal::decimal_literal(a)    => Literal::decimal_literal(a.clone())    ,
	}
	
    }
}



impl PartialEq for Data_Type {

    fn eq(&self , other : &Self ) -> bool {

	match (self , other ){
	    
	    (Data_Type::CHAR(a) ,   Data_Type::CHAR(b) ) => return true,
	    (Data_Type::VOID(a) ,   Data_Type::VOID(b) ) => return true,
	    (Data_Type::I32(a) ,    Data_Type::I32(b) ) => return true,
	    (Data_Type::I64(a) ,    Data_Type::I64(b) ) => return true,
	    (Data_Type::F32(a) ,    Data_Type::F32(b) ) => return true,
	    (Data_Type::F64(a) ,    Data_Type::F64(b) ) => return true,
	    (Data_Type::STRING(a) , Data_Type::STRING(b) ) => return true,
	    _ => return false,
	}
    }
    
}



impl Clone for Data_Type {

    fn clone(&self ) -> Self {

	match self{
	    
	    Data_Type::CHAR(a)   =>   Data_Type::CHAR(a.clone()) ,
	    Data_Type::VOID(a)   =>   Data_Type::VOID(a.clone()) ,
	    Data_Type::I32(a)    =>    Data_Type::I32(a.clone()) ,
	    Data_Type::I64(a)    =>    Data_Type::I64(a.clone()) ,
	    Data_Type::F32(a)    =>    Data_Type::F32(a.clone()) ,
	    Data_Type::F64(a)    =>    Data_Type::F64(a.clone()) ,
	    Data_Type::STRING(a) => Data_Type::STRING(a.clone()) ,
	}
    }
    
}



impl PartialEq for Statement{

    fn eq(&self , other : &Self) -> bool{

	match (self , other) {

	    (Statement::function_marker(a) , Statement::function_marker(b) ) => return true,
	    (Statement::if_statement(a) , Statement::if_statement(b) ) => return true,
	    (Statement::else_statement(a) , Statement::else_statement(b) ) => return true,
	    (Statement::else_if_statement(a) , Statement::else_if_statement(b) ) => return true,
	    (Statement::return_statement(a) , Statement::return_statement(b) ) => return true,
	    (Statement::for_statement(a) , Statement::for_statement(b) ) => return true,
	    (Statement::in_statement(a) , Statement::in_statement(b) ) => return true,
	    (Statement::to_statement(a) , Statement::to_statement(b) ) => return true,
	    (Statement::and_operator(a) , Statement::and_operator(b)) => return true,
	    (Statement::or_operator(a) , Statement::or_operator(b) ) => return true ,
	    (Statement::xor_operator(a) , Statement::xor_operator(b)) => return true,
	    _ => return false,
	    
	}
	
    }
}



impl Clone for Statement{

    fn clone(&self) -> Self{

	match self {

	    Statement::function_marker(a)  => Statement::function_marker(a.clone())  ,
	    Statement::if_statement(a)     => Statement::if_statement(a.clone())     ,
	    Statement::else_statement(a)   => Statement::else_statement(a.clone())   ,
	    Statement::else_if_statement(a)   => Statement::else_if_statement(a.clone())   ,
	    Statement::return_statement(a) => Statement::return_statement(a.clone()) ,
	    Statement::for_statement(a)    => Statement::for_statement(a.clone())    ,
	    Statement::in_statement(a)    => Statement::in_statement(a.clone())    ,
	    Statement::to_statement(a)    => Statement::to_statement(a.clone())    ,
            Statement::struct_statement(a) => Statement::struct_statement(a.clone()),

	    Statement::and_operator(a)    => Statement::and_operator(a.clone())    ,
	    Statement::or_operator(a)     => Statement::or_operator(a.clone())     ,
	    Statement::xor_operator(a)    => Statement::xor_operator(a.clone())    ,
	}
	
    }
}


impl PartialEq for Keyword{

    fn eq(&self , other : &Self) -> bool{

	match(self , other ) {

	    (Keyword::data_type(a) , Keyword::data_type(b)) => return true,
	    (Keyword::statement(a) , Keyword::statement(b)) => return true,
	    _ => false,
	    
	}
    }
    
}



impl Clone for Keyword{

    fn clone(&self) -> Self{

	match self {

	    Keyword::data_type(a) => Keyword::data_type(a.clone()),
	    Keyword::statement(a) => Keyword::statement(a.clone()),
	    
	}
    }
    
}


impl PartialEq for STC{

    fn eq(&self , other : &Self) -> bool{

	match (self , other ){
	    (STC::stc_scope_begin(a),STC::stc_scope_begin(b)) => return true,
	    (STC::stc_scope_end(a),STC::stc_scope_end(b)) => return true,
	    (STC::stc_comma_seperator(a) , STC::stc_comma_seperator(b)) => return true,
	    (STC::stc_end_expression(a),STC::stc_end_expression(b)) => return true,
	    (STC::stc_arg_begin(a),STC::stc_arg_begin(b)) => return true,
	    (STC::stc_arg_end(a),STC::stc_arg_end(b)) => return true,
	     (STC::stc_dot(a),STC::stc_dot(b)) => return true,
	    _ => return false,
	}
    }
}

impl Clone for STC{

    fn clone(&self) -> Self{

	match self {
	    STC::stc_scope_begin(a)    => STC::stc_scope_begin(a.clone())   ,
	    STC::stc_scope_end(a)      => STC::stc_scope_end(a.clone())     ,
	    STC::stc_comma_seperator(a) => STC::stc_comma_seperator(a.clone()) , 
	    STC::stc_end_expression(a) => STC::stc_end_expression(a.clone()),
	    STC::stc_arg_begin(a)      => STC::stc_arg_begin(a.clone())     ,
	    STC::stc_arg_end(a)        => STC::stc_arg_end(a.clone())       ,
	    STC::stc_dot(a)        => STC::stc_dot(a.clone())       ,

	}
    }
}



impl PartialEq for Operator{

    fn eq(&self , other : &Self) -> bool{

	match (self , other ){
	    (Operator::assignment_op(a),Operator::assignment_op(b)) => return true,
	    (Operator::type_assignment_op(a),Operator::type_assignment_op(b)) => return true,
	    (Operator::addition_op(a),Operator::addition_op(b)) => return true,
	    (Operator::subtraction_op(a),Operator::subtraction_op(b)) => return true,
	    (Operator::multiplication_op(a),Operator::multiplication_op(b)) => return true,
	    (Operator::division_op(a),Operator::division_op(b)) => return true,

	    (Operator::not_op(a),Operator::not_op(b)) => return true,
	    (Operator::check_equal_op(a),Operator::check_equal_op(b)) => return true,
	    (Operator::not_equal_op(a),Operator::not_equal_op(b)) => return true,
	    (Operator::greater_than_op(a),Operator::greater_than_op(b)) => return true,
	    (Operator::lesser_than_op(a) ,Operator::lesser_than_op(b)) => return true,

	    (Operator::and_op(a),Operator::and_op(b))  => return true,
	    (Operator::or_op(a),Operator::or_op(b))  => return true,
	    (Operator::xor_op(a),Operator::xor_op(b))  => return true,
	    
	    _ => return false,
	}
    }
}


impl Clone for Operator{

    fn clone(&self) -> Self{

	match self{
	    Operator::assignment_op(a)      =>  Operator::assignment_op(a.clone())    ,
	    Operator::type_assignment_op(a)      =>  Operator::type_assignment_op(a.clone())    ,
	    Operator::addition_op(a)        =>  Operator::addition_op(a.clone())      ,
	    Operator::subtraction_op(a)     =>  Operator::subtraction_op(a.clone())   ,
	    Operator::multiplication_op(a)  =>  Operator::multiplication_op(a.clone()),
	    Operator::division_op(a)        =>  Operator::division_op(a.clone())      ,
	    
	    Operator::not_op(a)             =>  Operator::not_op(a.clone())               ,
	    Operator::check_equal_op(a)     =>  Operator::check_equal_op(a.clone())       ,
	    Operator::not_equal_op(a)       =>  Operator::not_equal_op(a.clone())         ,
	    Operator::greater_than_op(a)    =>  Operator::greater_than_op(a.clone())      ,
	    Operator::lesser_than_op(a)     =>  Operator::lesser_than_op(a.clone())       ,
	    
	    Operator::and_op(a)    =>  Operator::and_op(a.clone())     ,
	    Operator::or_op(a)     =>  Operator::or_op(a.clone())      ,
	    Operator::xor_op(a)    =>  Operator::xor_op(a.clone())     ,
	}
    }
}



impl PartialEq for  Token{

    fn eq(&self , other : &Self) -> bool{

	match(self , other  ){

	    (Token::t_keyword(a) , Token::t_keyword(b)) => return true,
	    (Token::t_literal(a) , Token::t_literal(b)) => return true,
	    (Token::t_stc(a) , Token::t_stc(b)) => return true,
	    (Token::t_operator(a) , Token::t_operator(b)) => return true,
	    (Token::t_identifier(a) , Token::t_identifier(b)) => return true,
	    _ => return false,
	}
	
    }

}

impl Clone for  Token{

    fn clone(&self) -> Self{

	match self {

	    Token::t_keyword(a)    =>  Token::t_keyword(a.clone())    ,
	    Token::t_literal(a)    =>  Token::t_literal(a.clone())    ,
	    Token::t_stc(a)        =>  Token::t_stc(a.clone())        ,
	    Token::t_operator(a)   =>  Token::t_operator(a.clone())   ,
	    Token::t_identifier(a) =>  Token::t_identifier(a.clone()) ,
            Token::t_eof => Token::t_eof,
	}
	
    }

}


impl Clone for Position {

    fn clone(&self) -> Self {
	Self{
	    x : self.x,
	    y : self.y,
	}
    }
}


impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::character_literal(c) => write!(f, "'{}'", c),
            Literal::string_literal(s) => write!(f, "\"{}\"", s),
            Literal::integer_literal(i) => write!(f, "{}", i),
            Literal::decimal_literal(d) => write!(f, "{}", d),
        }
    }
}

impl fmt::Display for Data_Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data_Type::CHAR(s) => write!(f, "Char({})", s),
            Data_Type::VOID(s) => write!(f, "Void({})", s),
            Data_Type::I32(s) => write!(f, "I32({})", s),
            Data_Type::I64(s) => write!(f, "I64({})", s),
            Data_Type::F32(s) => write!(f, "F32({})", s),
            Data_Type::F64(s) => write!(f, "F64({})", s),
            Data_Type::STRING(s) => write!(f, "String({})", s),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::function_marker(s) => write!(f, "fn {}", s),
            Statement::if_statement(s) => write!(f, "if {}", s),
            Statement::else_statement(s) => write!(f, "else {}", s),
            Statement::else_if_statement(s) => write!(f, "else if {}", s),
            Statement::return_statement(s) => write!(f, "return {}", s),
            Statement::for_statement(s) => write!(f, "for {}", s),
            Statement::in_statement(s) => write!(f, "in {}", s),
            Statement::to_statement(s) => write!(f, "to {}", s),
            Statement::and_operator(s) => write!(f, "and {}", s),
            Statement::or_operator(s) => write!(f, "or {}", s),
            Statement::xor_operator(s) => write!(f, "xor {}", s),
            Statement::struct_statement(s) => write!(f, "struct {}" , s),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::data_type(dt) => write!(f, "Keyword::DataType({})", dt),
            Keyword::statement(stmt) => write!(f, "Keyword::Statement({})", stmt),
        }
    }
}

impl fmt::Display for STC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            STC::stc_scope_begin(c) => write!(f, "{}", c),        // Typically '{'
            STC::stc_scope_end(c) => write!(f, "{}", c),          // Typically '}'
            STC::stc_comma_seperator(c) => write!(f, "{}", c),    // Typically ','
            STC::stc_end_expression(c) => write!(f, "{}", c),     // Typically ';'
            STC::stc_arg_begin(c) => write!(f, "{}", c),          // Typically '('
            STC::stc_arg_end(c) => write!(f, "{}", c),            // Typically ')'
            STC::stc_dot(c) => write!(f, "{}", c),                // Typically '.'
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::assignment_op(c) => write!(f, "{}", c),
            Operator::type_assignment_op(s) => write!(f, "{}", s),
            Operator::addition_op(c) => write!(f, "{}", c),
            Operator::subtraction_op(c) => write!(f, "{}", c),
            Operator::multiplication_op(c) => write!(f, "{}", c),
            Operator::division_op(c) => write!(f, "{}", c),
            Operator::not_op(c) => write!(f, "{}", c),
            Operator::check_equal_op(s) => write!(f, "{}", s),
            Operator::not_equal_op(s) => write!(f, "{}", s),
            Operator::greater_than_op(c) => write!(f, "{}", c),
            Operator::lesser_than_op(c) => write!(f, "{}", c),
            Operator::and_op(s) => write!(f, "{}", s),
            Operator::or_op(s) => write!(f, "{}", s),
            Operator::xor_op(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::t_keyword(kw) => write!(f, "Keyword: [{}]", kw),
            Token::t_literal(lit) => write!(f, "Literal: [{}]", lit),
            Token::t_stc(stc) => write!(f, "STC: [{}]", stc),
            Token::t_operator(op) => write!(f, "Operator: [{}]", op),
            Token::t_identifier(id) => write!(f, "Identifier: [{}]", id),
            Token::t_eof => write!(f , "EOF"),
        }
    }
}
