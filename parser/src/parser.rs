use crate::expressions::*;
use crate::lexer::lex::Lexeme;
use crate::lexer::token_type::*;
use crate::parsingdata::*;
use crate::function::*;
use crate::global::create_global_scope;
use crate::iterator::find_iterator_in_scope;
use crate::variable::find_variable_declarations_in_scope;
use crate::function_return::find_returns_inside_scope;
use crate::function_call::find_function_calls_in_scope;

pub fn parser(parsingvec : Vec<ParsingData> ) -> Vec<ParsingData> {

    let mut program = create_global_scope(parsingvec);

    program.block = find_functions_in_scope(program.block , program.scope.clone() );
    let mut tmp_count = 0;
    program.block = find_iterator_in_scope(program.block , program.scope.clone() , &mut tmp_count );
    program.block = find_function_calls_in_scope(program.block , program.scope.clone() , &mut tmp_count );
    program.block = find_variable_declarations_in_scope(program.block , program.scope.clone() ,&mut tmp_count );
    program.block = find_returns_inside_scope(program.block , program.scope.clone() ,&mut tmp_count );
    
    parse_recurse(&mut program);

    return program.block;
}

fn parse_recurse(block : &mut Block ){

    

    for i in 0 .. block.block.len(){
	
	if matches!(block.block[i] , ParsingData::functiondef(_)){
	    println!("Parse_recurse condition 1\n");
	    if let ParsingData::functiondef(mut fdef ) = block.block[i].clone(){
		
		if let Some(mut body) = fdef.fn_body{

		    
		    body.block = find_functions_in_scope(body.clone().block , body.scope.clone() );
		    let mut tmp_count = 0;
		    body.block = find_iterator_in_scope(body.clone().block , body.scope.clone() , &mut tmp_count);
		    body.block = find_function_calls_in_scope(body.clone().block , body.scope.clone() ,  &mut tmp_count);
		    body.block = find_variable_declarations_in_scope(body.clone().block , body.scope.clone() ,&mut tmp_count );
		    body.block = find_returns_inside_scope(body.clone().block , body.scope.clone() ,&mut tmp_count );
		    if has_scope(body.clone().block){
			parse_recurse(&mut body);
		    }
		    
		    
		    
		    
		    fdef.fn_body = Some(body);
		}
		block.block[i] =  ParsingData::functiondef(fdef);
		
	    }
	}
	if matches!(block.block[i] , ParsingData::iterator(_)){

	    println!("Parse_recurse condition 2\n");
	    if let ParsingData::iterator(mut iterator ) = block.block[i].clone(){
		
		if let Some(mut body) = iterator.iter_body{

		    
		    body.block = find_functions_in_scope(body.clone().block , body.scope.clone() );
		    let mut tmp_count = 0;
		    body.block = find_iterator_in_scope(body.clone().block , body.scope.clone() , &mut tmp_count );
		    body.block = find_function_calls_in_scope(body.clone().block , body.scope.clone(), &mut tmp_count );
		    body.block = find_variable_declarations_in_scope(body.clone().block , body.scope.clone() ,&mut tmp_count );
		    body.block = find_returns_inside_scope(body.clone().block , body.scope.clone() ,&mut tmp_count );
		    if has_scope(body.clone().block){
			parse_recurse(&mut body);
		    }
		    
		    
		    
		    iterator.iter_body = Some(body);
		}
		block.block[i] =  ParsingData::iterator(iterator);
		
	    }
	    
	}
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
