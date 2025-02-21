use crate::expressions::*;
use crate::lexer::lex::Lexeme;
use crate::lexer::token_type::*;
use crate::parsingdata::*;
use crate::function::*;
use crate::global::create_global_scope;
use crate::iterator::find_iterator_in_scope;
use crate::variable::find_variable_declarations_in_scope;
use crate::function_return::find_returns_inside_scope;

pub fn parser(parsingvec : Vec<ParsingData> ) -> Vec<ParsingData> {

    let mut program = create_global_scope(parsingvec);

    
    program.block = find_iterator_in_scope(program.block , program.scope.clone() );
    program.block = find_functions_in_scope(program.block , program.scope.clone() );
    let mut tmp_count = 0;
    program.block = find_variable_declarations_in_scope(program.block , program.scope.clone() ,&mut tmp_count );

    
    	
//    parse_recurse(&mut program);

    return program.block;
}

fn parse_recurse(block : &mut Block ){

    if has_scope(block.clone().block){

	for i in 0 .. block.block.len(){
	    
	    if matches!(block.block[i] , ParsingData::functiondef(_)){
		
		if let ParsingData::functiondef(mut fdef ) = block.block[i].clone(){
		    
		    if let Some(mut body) = fdef.fn_body{
		    
			body.block = find_iterator_in_scope(body.block , body.scope.clone() );
			body.block = find_functions_in_scope(body.block , body.scope.clone() );
			let mut tmp_count = 0;
			body.block = find_variable_declarations_in_scope(body.block , body.scope.clone() ,&mut tmp_count );
			body.block = find_returns_inside_scope(body.block , body.scope.clone() ,&mut tmp_count );
			
			
			parse_recurse(&mut body);
			
			fdef.fn_body = Some(body);
		    }
		    block.block[i] =  ParsingData::functiondef(fdef);
		    
		}
	    }else if matches!(block.block[i] , ParsingData::iterator(_)){

		
		if let ParsingData::iterator(mut iterator ) = block.block[i].clone(){
		    
		    if let Some(mut body) = iterator.iter_body{
		    
			body.block = find_iterator_in_scope(body.block , body.scope.clone() );
			body.block = find_functions_in_scope(body.block , body.scope.clone() );
			let mut tmp_count = 0;
			body.block = find_variable_declarations_in_scope(body.block , body.scope.clone() ,&mut tmp_count );
			body.block = find_returns_inside_scope(body.block , body.scope.clone() ,&mut tmp_count );
			
			
			parse_recurse(&mut body);
			
			iterator.iter_body = Some(body);
		    }
		    block.block[i] =  ParsingData::iterator(iterator);
		    
		}
		
	    }
	}
    }else {
	
	return;
    }
}



pub fn has_scope(parsingvec : Vec<ParsingData>) -> bool {

    for i in parsingvec.iter(){
	if matches!(i , ParsingData::functiondef(_) | ParsingData::iterator(_)){
	    return true;
	}
    }

    return false;
}
