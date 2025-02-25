use std::rc::Rc;
use std::cell::RefCell;

use crate::token_type::*;
use crate::tokenizer::Tokenizer;

use crate::keywords::Keyword_Checker;
use crate::identifier::Identifier_Checker;
use crate::stc::STC_Checker;
use crate::operator::Operator_Checker;
use crate::literal::Literal_Checker;


#[derive(Debug)]
pub struct Lexeme{

    pub tokens : Token ,
    pub position : Position,
    
}

impl Lexeme{
    pub fn new(i_token : Token , i_pos : Position) -> Self {
	Self{
	    tokens : i_token ,
	    position : i_pos ,
	}
    } 
}

impl Clone for Lexeme{
    fn clone(&self) -> Self {
	Self{
	    tokens : self.tokens.clone(),
	    position : self.position.clone(),
	}
    }
}

pub struct Lexer{
    
    lexemes : Vec<Lexeme>
}

impl Lexer {

    pub fn new() -> Self {
	Self{
	    lexemes : Vec::new(),
	}
    }

    pub fn lexx(&mut self , file_path : String ) {

	let tokenizer = Tokenizer;
	let id_checker = Identifier_Checker;
	let kw_checker = Keyword_Checker;
	let stc_checker = STC_Checker;
	let op_checker = Operator_Checker;
	let lit_checker = Literal_Checker;

	
	let file_contents = tokenizer.load_source_file(file_path);
	let contents_vec : Vec<char> = file_contents.chars().collect();
	let contents_len  = file_contents.len();

	
	let context = Rc::new(RefCell::new(String::new()));
	let mut context_len = 0;
	let mut index : usize = 0;
	let mut  line : usize = 0;
	let mut   bol : usize = 0;

	while index < contents_len {
	    
	    // skips the single line comments
	    if index < (contents_len - 1) && contents_vec[index] == '/' && contents_vec[index + 1] == '/'{
		while contents_vec[index] != '\n'{
		    if index != (contents_len - 1){
			index += 1;
		    }else{
			break;
		    }
		}
		bol = 0;
		line += 1;
		index += 1;

	    }else if tokenizer.is_space(contents_vec[index]) {

		context_len = context.borrow().len();

		if context_len > 0 {

		    if let Some(a) = kw_checker.find(Rc::clone(&context)) {
			self.push_lexeme(a , Position{x : bol - context_len , y : line})
		    }
		    else if let Some(a) = stc_checker.find(Rc::clone(&context)) {
			self.push_lexeme(a , Position{x : bol - context_len , y : line})
		    }
		    else if let Some(a) = op_checker.find(Rc::clone(&context)) {
			self.push_lexeme(a , Position{x : bol - context_len , y : line})
		    }
		    else if let Some(a) = lit_checker.find(Rc::clone(&context)) {
			self.push_lexeme(a , Position{x : bol - context_len , y : line})
		    }
		    else if let Some(a) = id_checker.find(Rc::clone(&context)) {
			self.push_lexeme(a , Position{x : bol - context_len , y : line})
		    }

		    context.borrow_mut().clear();

		}

		if contents_vec[index] == '\n' { bol = 0; line += 1 ;} else { bol += 1};
		index += 1;

		
	    }else{
		if  contents_vec[index] == '"' {

		    context.borrow_mut().push(contents_vec[index]);
		    bol += 1;
		    index += 1;

		    while contents_vec[index] != '"'{

			context.borrow_mut().push(contents_vec[index]);
			bol += 1;
			index += 1;
			
		    }

		    
		    context.borrow_mut().push(contents_vec[index]);
		    bol += 1;
		    index += 1;

		    println!("3 Line was set to {}", line);
		}else if contents_vec[index] == '\'' {
		    
		    context.borrow_mut().push(contents_vec[index]);
		    bol += 1;
		    index += 1;

		    while contents_vec[index] != '\''{

			context.borrow_mut().push(contents_vec[index]);
			bol += 1;
			index += 1;
			
		    }

		    
		    context.borrow_mut().push(contents_vec[index]);
		    bol += 1;
		    index += 1;

		    


		}else if tokenizer.is_operator(contents_vec[index]){
		    
		    let context_len = context.borrow().len();

		    if context_len > 0 {

			

			if let Some(a) = kw_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = stc_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = op_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = lit_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = id_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			
			context.borrow_mut().clear();
			
		    }
		    
		    context.borrow_mut().push(contents_vec[index]);
		    bol += 1;
		    index += 1;
		    

		}else if tokenizer.is_stc(contents_vec[index]){
		    
		    let context_len = context.borrow().len();

		    if context_len > 0{
			
			if let Some(a) = kw_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = stc_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = op_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = lit_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}
			else if let Some(a) = id_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line})
			}

			
			context.borrow_mut().clear();
		    }
		    
		    context.borrow_mut().push(contents_vec[index]);
		    bol += 1;
		    index += 1;

		    
		}else {

		    let context_len = context.borrow().len();

		    if context_len > 0{

			if let Some(a) = stc_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line}) ;
			    context.borrow_mut().clear();
			}else if let Some(a) = op_checker.find(Rc::clone(&context)) {
			    self.push_lexeme(a , Position{x : bol - context_len , y : line}) ;
			    context.borrow_mut().clear();
			}
			
		    }
		    
		    context.borrow_mut().push(contents_vec[index]);
		    bol += 1;
		    index += 1;
		    
		}
		
	    }

	    
	}

	if context_len > 0 && bol > 0{
			
	    if let Some(a) = kw_checker.find(Rc::clone(&context)) {
		self.push_lexeme(a , Position{x : bol - context_len , y : line})
	    }
	    else if let Some(a) = stc_checker.find(Rc::clone(&context)) {
		self.push_lexeme(a , Position{x : bol - context_len , y : line})
	    }
	    else if let Some(a) = op_checker.find(Rc::clone(&context)) {
		self.push_lexeme(a , Position{x : bol - context_len , y : line})
	    }
	    else if let Some(a) = lit_checker.find(Rc::clone(&context)) {
		self.push_lexeme(a , Position{x : bol - context_len , y : line})
	    }
	    else if let Some(a) = id_checker.find(Rc::clone(&context)) {
		self.push_lexeme(a , Position{x : bol - context_len , y : line})
	    }
	}
	
    }

    pub fn get_lexemes(&self) -> Vec<Lexeme> {
	return self.lexemes.clone();
    }

    
    fn push_lexeme(&mut self , t : Token , p : Position ) {
	self.lexemes.push(Lexeme::new(t , p));
    }

}
