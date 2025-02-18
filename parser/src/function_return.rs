

use crate::parsingdata::ParsingData;
use crate::lexer::token_type::{Keyword , Statement , Token , STC};
use crate::binaryexp_handle::break_binary_expression;

pub fn find_returns_inside_scope(parsingvec : Vec<ParsingData> , scope : String , tmp_count : &mut usize) ->Vec<ParsingData>{

    let mut retval : Vec<ParsingData> = Vec::new();
    
    let mut i = 0;
    while i < parsingvec.len(){

	if let ParsingData::lexeme(lex) = parsingvec[i].clone() {
	    
	    if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::return_statement(_)))){

		i += 1;
		let mut binexp : Vec<Token > = Vec::new();
		loop {
		    if let ParsingData::lexeme(newlex) = parsingvec[i].clone(){
			if matches!(newlex.tokens , Token::t_stc(STC::stc_end_expression(_))){
			    binexp.push(newlex.tokens.clone());
			    break
			}else{
			    binexp.push(newlex.tokens.clone());
			}
		    }
		    i+=1;
		}

		i += 1;
		
		retval.push(ParsingData::function_return(break_binary_expression(&mut binexp , &scope , tmp_count)));
		*tmp_count += 1;
		
		
	    }else{
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
