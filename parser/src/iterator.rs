use crate::lexer::token_type::{Token , Statement , Keyword , STC};
use crate::parsingdata::{Block , ParsingData};
use crate::binaryexp::BinaryExpressionTree;
use crate::binaryexp_handle::break_binary_expression;

#[derive(Debug)]
pub enum IteratorType{

    IterateOverRange,
    IterateOverCondition,
}

#[derive(Debug)]
pub struct Iterator{

    pub iter_type : Option<IteratorType>,
    pub iter_condition : Option<BinaryExpressionTree>,
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
	    iter_condition : None,
	    iter_value : None ,
	    super_scope : scope ,
	    range_start : None,
	    range_end : None ,
	    iter_body : None ,
	}
	
    }

    pub fn clear(&mut self){

	self.iter_condition = None;
	self.iter_type  = None;
	self.iter_value = None;
	self.range_start= None;
	self.range_end  = None;
	self.iter_body  = None;

    }
}


pub fn find_iterator_in_scope(parsingvec : Vec<ParsingData> , scope : String , tmp_count : &mut usize  ) -> Vec<ParsingData> {

    println!("hello from the iterator finder");
    
    let mut retval : Vec<ParsingData> = Vec::new();

    let mut iter_count = 0;
    let len = parsingvec.len();
    let mut i = 0;
    

    while i < len {

	if let ParsingData::lexeme(lex) = parsingvec[i].clone() {

	    if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::for_statement(_)))){

		let mut iterator =  Iterator::new(scope.clone());


		if is_iterate_over_range(parsingvec.clone() ,
					 &mut i , &mut iterator ,
					 scope.clone() , iter_count) {
		    
		}else if  is_iterate_over_condition(parsingvec.clone() ,
					 &mut i , &mut iterator ,
					 scope.clone() ,tmp_count , iter_count) {
		    
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


pub fn is_iterate_over_range(parsingvec : Vec<ParsingData> ,
			     i : &mut usize ,
			     iterator : &mut Iterator ,
			     scope : String ,
			     iter_count : usize ) -> bool {

    let mut is_iter_over_range = false;
    
    
    if let ParsingData::lexeme(lex) = parsingvec[*i+1].clone(){

	// checks for i
	
	if matches!(lex.tokens , Token::t_identifier(_)){
	    
	    iterator.iter_value = Some(lex.tokens.clone());
	    
	    is_iter_over_range = true;
	    
	}else {
	    return false;
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[*i+2].clone(){
	if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::in_statement(_)))){
	    is_iter_over_range = true;
	}else {
	    return false;
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[*i+3].clone(){

	if matches!(lex.tokens , Token::t_identifier(_) | Token::t_literal(_)){

	    iterator.range_start = Some(lex.tokens.clone());
	    is_iter_over_range = true;
	    
	}else {
	    return false;
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[*i+4].clone(){
	if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::to_statement(_))) ){
	    is_iter_over_range = true;
	}else {
	    return false;
	    
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[*i+5].clone(){
	if matches!(lex.tokens , Token::t_identifier(_) | Token::t_literal(_) ){
	    
	    iterator.range_end = Some(lex.tokens.clone());
	    is_iter_over_range = true;
	    
	}else {
	    return false;
	    
	}
	
    }
    
    if is_iter_over_range == true {	

	*i += 6;

	iterator.iter_type = Some(IteratorType::IterateOverRange);
	
	let mut body_block : Vec<ParsingData> = Vec::new();
	
	if let ParsingData::lexeme(lex) = parsingvec[*i].clone(){
	    
	    if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_))){
		*i += 1;
		let mut found_inner = 0;
		loop {
		    
		    if let ParsingData::lexeme(lex) = parsingvec[*i].clone(){
			
			if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_))){
			    body_block.push(parsingvec[*i].clone());
			    
			    found_inner += 1;
			    
			}else if matches!(lex.tokens , Token::t_stc(STC::stc_scope_end(_))){
			    
			    if found_inner > 0{
				found_inner -= 1;
				body_block.push(parsingvec[*i].clone());
				
			    }else{
				
				break;
			    }
			    
			}else {
			    
			    body_block.push(parsingvec[*i].clone());
			}
		    }else {
			
			body_block.push(parsingvec[*i].clone());
		    }
		    
		    *i += 1;
		}
		
	    }
	}

	iterator.iter_body = Some (
	    Block {
		scope : format!("{}_iterator_{}", scope , iter_count ),
		block : body_block,
	    }
	);

	
	return is_iter_over_range;
    }else {
	return false;
    }
   
}

pub fn is_iterate_over_condition (parsingvec : Vec<ParsingData> ,
				  i : &mut usize ,
				  iterator : &mut Iterator ,
				  scope : String  ,
				  tmp_count : &mut usize ,
				  iter_count : usize) -> bool {

    let mut is_iter_over_condition = false;
    
    if let ParsingData::lexeme(lex) = parsingvec[*i+1].clone(){

	// checks for i
	
	if matches!(lex.tokens , Token::t_identifier(_)){
	    
	    iterator.iter_value = Some(lex.tokens.clone());
	    
	    is_iter_over_condition = true;
	    
	}else {
	    is_iter_over_condition = false;
	}
    }

    if let ParsingData::lexeme(lex) = parsingvec[*i+2].clone(){
	if matches!(lex.tokens , Token::t_operator(_)){
	    is_iter_over_condition = true;
	}else {
	    is_iter_over_condition = false;
	}
    } 

  

    if is_iter_over_condition == true {

	iterator.clear();
	*i += 1;
	let mut binexp : Vec<Token > = Vec::new();

	loop {

	    if let ParsingData::lexeme(lex) = parsingvec[*i].clone(){

		if matches!(lex.tokens  , Token::t_stc(STC::stc_scope_begin(_)) ){
		    break;
		}else {
		    binexp.push(lex.tokens);
		}
		    
	    }
	    *i += 1;
	}

	iterator.iter_condition = Some (break_binary_expression(&mut binexp ,
								 &format!("{}_iterator_{}" , scope ,iter_count ) ,
								 tmp_count )
	);
	*tmp_count += 1;
	
//	*i += 1;

	iterator.iter_type = Some(IteratorType::IterateOverCondition);
	
	let mut body_block : Vec<ParsingData> = Vec::new();
	
	if let ParsingData::lexeme(lex) = parsingvec[*i].clone(){
	    
	    if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_))){
		*i += 1;
		let mut found_inner = 0;
		loop {
		    
		    if let ParsingData::lexeme(lex) = parsingvec[*i].clone(){
			
			if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_))){
			    body_block.push(parsingvec[*i].clone());
			    
			    found_inner += 1;
			    
			}else if matches!(lex.tokens , Token::t_stc(STC::stc_scope_end(_))){
			    
			    if found_inner > 0{
				found_inner -= 1;
				body_block.push(parsingvec[*i].clone());
				
			    }else{
				
				break;
			    }
			    
			}else {
			    
			    body_block.push(parsingvec[*i].clone());
			}
		    }else {
			
			body_block.push(parsingvec[*i].clone());
		    }
		    
		    *i += 1;
		}
		
	    }
	}

	iterator.iter_body = Some (
	    Block {
		scope : format!("{}_iterator_{}", scope , iter_count ),
		block : body_block,
	    }
	);

	return true;
    }else{
	return true;
    }
    
}
