#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use crate::token_type::*;

pub struct Identifier_Checker;

impl Identifier_Checker{

    pub fn find(&self , inp : Rc<RefCell<String>>) -> Option<Token>{

	if inp.borrow_mut().chars().next()?.is_numeric() {
	    return None;
	}

	return Some(Token::t_identifier(inp.borrow_mut().clone()))
    }
    
}
