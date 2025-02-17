#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use crate::token_type::*;

pub struct Operator_Checker;

impl Operator_Checker{

    pub fn find(&self , inp : Rc<RefCell<String>>) -> Option<Token>{
	match inp.borrow_mut() {
	    s if *s == "=" => return Some(Token::t_operator(Operator::assignment_op(s.clone()))) ,
	    s if *s == ":" => return Some(Token::t_operator(Operator::type_assignment_op(s.clone()))) ,
	    s if *s == "+" => return Some(Token::t_operator(Operator::addition_op(s.clone()))) ,
	    s if *s == "-" => return Some(Token::t_operator(Operator::subtraction_op(s.clone()))) ,
	    s if *s == "*" => return Some(Token::t_operator(Operator::multiplication_op(s.clone()))) ,
	    s if *s == "/" => return Some(Token::t_operator(Operator::division_op(s.clone()))) ,
	    _ => return None,
	}
    }
    
}
