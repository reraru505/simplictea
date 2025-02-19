#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use crate::token_type::*;

pub struct STC_Checker;

impl STC_Checker{

    pub fn find(&self , inp : Rc<RefCell<String>>) -> Option<Token>{
	match inp.borrow_mut() {
	    s if *s == "{" => return Some(Token::t_stc(STC::stc_scope_begin(s.clone()))) ,
	    s if *s == "}" => return Some(Token::t_stc(STC::stc_scope_end(s.clone()))) ,
	    s if *s == "," => return Some(Token::t_stc(STC::stc_comma_seperator(s.clone()))) ,
	    s if *s == ";" => return Some(Token::t_stc(STC::stc_end_expression(s.clone()))) ,
	    s if *s == "(" => return Some(Token::t_stc(STC::stc_arg_begin(s.clone()))) ,
	    s if *s == ")" => return Some(Token::t_stc(STC::stc_arg_end(s.clone()))) ,
	    s if *s == "." => return Some(Token::t_stc(STC::stc_dot(s.clone()))),
	    _ => return None,
	}
    }
    
}

