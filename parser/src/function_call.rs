use crate::parsingdata::ParsingData;
use crate::lexer::token_type::{Token , STC};
use crate::lexer::lex::Lexeme;
use crate::function_helpers::get_function_id;
use crate::function::get_function_args;
use crate::binaryexp::BinaryExpressionTree;
use crate::binaryexp_handle::break_binary_expression;

#[derive(Debug)]
pub struct FunctionCall{
    pub super_scope : String ,
    pub fn_id : String ,
    pub fn_args : Option<Vec<BinaryExpressionTree>>,
}

impl Clone for FunctionCall {
    fn clone (&self ) -> Self {
	Self {
	    super_scope : self.super_scope.clone(),
	    fn_id : self.fn_id.clone(),
	    fn_args : self.fn_args.clone(),
	}
    }
}

impl FunctionCall{
    pub fn new(id : String , scope : String) -> Self {
	Self {
	    super_scope : scope  ,
	    fn_id : id ,
	    fn_args : None ,
	    
	}
    }
}

pub fn find_function_calls_in_scope(parsingvec : Vec<ParsingData> , scope : String , tmp_count : &mut usize )-> Vec<ParsingData>{

    let mut index = 0;
    let len = parsingvec.len();
    let mut retval : Vec<ParsingData> = Vec::new();

    while index < len {

	if index + 1 < len && matches!(parsingvec[index].clone() , ParsingData::lexeme(_)  ){

	    if let ParsingData::lexeme(lex ) = parsingvec[index].clone() {

		if let Token::t_identifier(id ) = lex.tokens {

		    if let ParsingData::lexeme(innerlex ) = parsingvec[index + 1].clone(){
			if matches!(innerlex.tokens , Token::t_stc(STC::stc_arg_begin(_) )){

			    let mut funcall = FunctionCall::new(
				get_function_id(parsingvec[index].clone()),
				scope.clone()
			    );
			    
			    index += 2;
			    let mut fargs : Vec<Lexeme> = Vec::new();

			    index = get_function_args(parsingvec.clone() , &mut fargs , index);
			    index -= 1;

			    funcall.fn_args = Some(
				enumerate_args(fargs , scope.clone() , tmp_count )
			    );
			    retval.push(
				ParsingData::functioncall(funcall)
			    );
			    index += 1;

			}else {
			    retval.push(parsingvec[index].clone());
			    index += 1;
			}
		    }

		    
		}else {
		    retval.push(parsingvec[index].clone());
		    index += 1;
		}
		
	    }
	    
	}else {
	    retval.push(parsingvec[index].clone());
	    index += 1;
	}
	
    }

    return retval;
}

pub fn enumerate_args(inargs : Vec<Lexeme> , scope : String , tmp_count : &mut usize ) -> Vec<BinaryExpressionTree>{

    let mut args : Vec<Vec<Token>> = Vec::new();
    let mut tmp_arg : Vec<Token> = Vec::new();
    
    for i in inargs.iter(){

	if matches!(i.tokens , Token::t_stc(STC::stc_comma_seperator(_))){
	    args.push(tmp_arg.clone());
	    tmp_arg.clear();
	}else{
	    tmp_arg.push(i.tokens.clone());
	}
	
    }
    args.push(tmp_arg.clone());
    
    let mut retval : Vec<BinaryExpressionTree> = Vec::new();

    for i in args.iter(){
	
	retval.push(break_binary_expression(&mut i.clone() , &scope , tmp_count));
	*tmp_count += 1;
    }

    return retval;
}
