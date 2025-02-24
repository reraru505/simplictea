use crate::parsingdata::ParsingData;
use crate::lexer::token_type::{Token , STC};
use crate::lexer::lex::Lexeme;
use crate::function_helpers::get_function_id;
use crate::function::get_function_args;

#[derive(Debug)]
pub struct FunctionCall{
    pub super_scope : String ,
    pub fn_id : String ,
    pub fn_args : Option<Vec<Lexeme>>,
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

pub fn find_function_calls_in_scope(parsingvec : Vec<ParsingData> , scope : String )-> Vec<ParsingData>{

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

			    funcall.fn_args = Some(fargs);
			    retval.push(
				ParsingData::functioncall(funcall)
			    );
			    

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

pub fn enumerate_args(args : Vec<Lexeme>) -> usize{

    if args.len() == 0 {
	return 0;
    }
    
    let mut num = 1;
    for i in args.iter(){
	if matches!(i.tokens , Token::t_stc(STC::stc_comma_seperator(_)))  {
	    num += 1;
	}
    }
    return num;
    
}
