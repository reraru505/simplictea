#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use crate::token_type::*;

pub struct Literal_Checker;

impl Literal_Checker{
    
    pub fn find(&self , inp : Rc<RefCell<String>> ) -> Option<Token>{

	if let Some(a) = is_character_literal(Rc::clone(&inp)) { return Some(Token::t_literal(a)); }
	
	if let Some(a) = is_string_literal(Rc::clone(&inp)) { return Some(Token::t_literal(a)); }
	
	if let Some(a) = is_integer_literal(Rc::clone(&inp)) { return Some(Token::t_literal(a)); }
	
	if let Some(a) = is_decimal_literal(Rc::clone(&inp)) { return Some(Token::t_literal(a)); }
	
	None	
    }
    
}

fn is_character_literal(inp : Rc<RefCell<String>>) -> Option<Literal>{

    if inp.borrow_mut().chars().next()? == '\'' && inp.borrow_mut().chars().last()? == '\'' {
	return Some(Literal::character_literal(inp.borrow_mut().clone()));
    }

    return None;
    
}


fn is_string_literal(inp : Rc<RefCell<String>>) -> Option<Literal>{

    if inp.borrow_mut().chars().next()? == '"' && inp.borrow_mut().chars().last()? == '"' {
	return Some(Literal::string_literal(inp.borrow_mut().clone()));
    }

    return None;
    
}


fn is_integer_literal(inp : Rc<RefCell<String>>) -> Option<Literal>{

    for i in inp.borrow_mut().chars(){
	if ! i.is_numeric(){
	    return None;
	}
    }

    return Some(Literal::integer_literal(inp.borrow_mut().clone()));
}


fn is_decimal_literal(inp : Rc<RefCell<String>>) -> Option<Literal>{

    
    for i in inp.borrow_mut().chars(){
	if ! i.is_numeric() || i != '.'{
	    return None;
	}
    }

    return Some(Literal::decimal_literal(inp.borrow_mut().clone()));
    
}
