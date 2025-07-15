#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use crate::token_type::*;


pub struct Keyword_Checker;

impl Keyword_Checker{

    pub fn find(&self , inp : Rc<RefCell<String>>) ->  Option<Token>{

	match find_data_type(Rc::clone(&inp)){
	    Some(val) => return Some(Token::t_keyword(Keyword::data_type(val))),
	    None => {} ,
	}

	match find_statement(Rc::clone(&inp)){
	    Some(val) =>  return Some(Token::t_keyword(Keyword::statement(val))),
	    None => {} ,
	}

	return None;
    }
    
    
}


pub fn find_data_type(input_string : Rc<RefCell<String>>) -> Option< Data_Type>{

    match input_string.borrow_mut() {
	s if *s == "char".to_string()   => return Some(Data_Type::CHAR("char".to_string())),
	s if *s == "void".to_string()   => return Some(Data_Type::VOID("void".to_string())),
	s if *s == "i32".to_string()    => return Some(Data_Type::I32("i32".to_string())),
	s if *s == "i64".to_string()    => return Some(Data_Type::I64("i64".to_string())),
	s if *s == "f32".to_string()    => return Some(Data_Type::F32("i32".to_string())),
	s if *s == "f64".to_string()    => return Some(Data_Type::F64("i64".to_string())),
	s if *s == "string".to_string() => return Some(Data_Type::STRING("string".to_string())),
	_ => return None,
	
    }
    
} 



pub fn find_statement(input_string : Rc<RefCell<String>>) -> Option<Statement> {
    
	match input_string.borrow_mut() {
	    s if *s == "fn".to_string()   => return Some(Statement::function_marker("fn".to_string())),
	    s if *s == "if".to_string()   => return Some(Statement::if_statement("if".to_string())),
	    s if *s == "else".to_string()    => return Some(Statement::else_statement("else".to_string())),
	    s if *s == "return".to_string()    => return Some(Statement::return_statement("return".to_string())),
	    s if *s == "for".to_string()    => return Some(Statement::for_statement("for".to_string())),
	    s if *s == "in".to_string()    => return Some(Statement::in_statement("in".to_string())),
	    s if *s == "to".to_string()    => return Some(Statement::to_statement("to".to_string())),
	    
	    //logical operators handled as keywords
	    s if *s == "and".to_string()    => return Some(Statement::and_operator("and".to_string())),
	    s if *s == "or".to_string()    => return Some(Statement::or_operator("or".to_string())),
	    s if *s == "xor".to_string()    => return Some(Statement::xor_operator("xor".to_string())),
	    s if *s == "struct".to_string()    => return Some(Statement::struct_statement("struct".to_string())),
	    _ => return None,
	    
	}
    
    
}


