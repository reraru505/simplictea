use crate::lexer::token_type::{Token , Statement , Keyword , STC};
use crate::parsingdata::{Block , ParsingData};

#[derive(Debug)]
pub enum IteratorType{

    IterateOverRange,
}

#[derive(Debug)]
pub struct Iterator{

    pub iter_type : Option<IteratorType>,
    pub iter_value : Option<Token>,
    pub super_scope : String ,
    pub range_start : Option<Token>,
    pub range_end : Option<Token>,
    pub iter_body : Option<Block>,
}

impl Iterator {
    pub fn new(scope : String ) -> Self {

	Self {
	    iter_type : None ,
	    iter_value : None ,
	    super_scope : scope ,
	    range_start : None,
	    range_end : None ,
	    iter_body : None ,
	}
	
    }

    pub fn clear(&mut self){

	self.iter_type  = None;
	self.iter_value = None;
	self.range_start= None;
	self.range_end  = None;
	self.iter_body  = None;

    }
}


pub fn find_iterator_in_scope(parsingvec : Vec<ParsingData> , scope : String  ) -> Vec<ParsingData> {

    println!("hello from the iterator finder");
    
    let mut retval : Vec<ParsingData> = Vec::new();

    let mut iter_count = 0;
    let len = parsingvec.len();
    let mut i = 0;
    

    while i < len {

	if let ParsingData::lexeme(lex) = parsingvec[i].clone() {

	    if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::for_statement(_)))){

		let mut iterator =  Iterator::new(scope.clone());

		let mut is_iter_over_range = false;

		
		if let ParsingData::lexeme(lex) = parsingvec[i+1].clone(){

		    // checks for i
		    
		    if matches!(lex.tokens , Token::t_identifier(_)){
				
			iterator.iter_value = Some(lex.tokens.clone());
			
			is_iter_over_range = true;
			
		    }else {
			is_iter_over_range = false;
		    }
		}
		if let ParsingData::lexeme(lex) = parsingvec[i+2].clone(){
		    if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::in_statement(_)))){
			is_iter_over_range = true;
		    }else {
			is_iter_over_range = false;
		    }
		}
		if let ParsingData::lexeme(lex) = parsingvec[i+3].clone(){

		    if matches!(lex.tokens , Token::t_identifier(_) | Token::t_literal(_)){

			iterator.range_start = Some(lex.tokens.clone());
			is_iter_over_range = true;
			
		    }else {
			is_iter_over_range = false;
		    }
		}
		if let ParsingData::lexeme(lex) = parsingvec[i+4].clone(){
		    if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::to_statement(_))) ){
			is_iter_over_range = true;
		    }else {
			is_iter_over_range = false;
			
		    }
		}
		if let ParsingData::lexeme(lex) = parsingvec[i+5].clone(){
		    if matches!(lex.tokens , Token::t_identifier(_) | Token::t_literal(_) ){
			
			iterator.range_end = Some(lex.tokens.clone());
			is_iter_over_range = true;
			
		    }else {
			is_iter_over_range = false;
			
		    }
		    
		}
		
		i += 6;
		
		if is_iter_over_range == true {
		    
		    iterator.iter_type = Some(IteratorType::IterateOverRange);
		    
		    let mut body_block : Vec<ParsingData> = Vec::new();
		    
		    if let ParsingData::lexeme(lex) = parsingvec[i].clone(){
			
			if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_))){
			    i += 1;
			    let mut found_inner = 0;
			    loop {
				
				if let ParsingData::lexeme(lex) = parsingvec[i].clone(){
				    
				    if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_))){
					
					found_inner += 1;
					
				    }else if matches!(lex.tokens , Token::t_stc(STC::stc_scope_end(_))){
					
					if found_inner > 0{
					    found_inner -= 1;
					    
					}else{
					    
					    break;
					}
					
				    }else {
					
					body_block.push(parsingvec[i].clone());
				    }
				}else {
				    
				    body_block.push(parsingvec[i].clone());
				}
				
				i += 1;
			    }
			    
			}
		    }

		    iterator.iter_body = Some (
			Block {
			    scope : format!("{}_iterator_{}", scope , iter_count ),
			    block : body_block,
			}
		    );
		    
		}

		iter_count += 1;
		retval.push(ParsingData::iterator(iterator ));
		
		i += 1;


	    }else {
		retval.push(parsingvec[i].clone());
		i += 1;
	    
	    }
	}else {

	    retval.push(parsingvec[i].clone());
	    i += 1;
	    
	}
    	
    } 
    
    return retval;
    
}
