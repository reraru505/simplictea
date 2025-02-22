use crate::parsingdata::{ParsingData , Block};
use crate::lexer::token_type::*;
use crate::symboltable::symbol::DataType;
use crate::lexer::lex::Lexeme;
use crate::function_helpers::{is_def_with_args ,
			      is_def_no_args ,
			      get_function_id,
			      get_function_return_type};

use crate::variable::{Variable,
		      get_variable_def_from_args};


#[derive(Debug)]
pub enum FunctionDefType{

    DEF_WITH_ARGS,
    DEF_NO_ARGS,

}

#[derive(Debug)]
pub struct FunctionDef{

    pub super_scope : String,
    pub fn_id : String ,
    pub fn_type : Option<FunctionDefType>,
    pub fn_return_type : DataType,
    pub fn_args : Option< Vec<Variable>>,
    pub fn_body : Option<Block>,
}

impl FunctionDef{
    pub fn new(fid : String , frt : DataType , scope : String ) -> Self{
	Self{
	    super_scope : scope,
	    fn_id : fid ,
	    fn_type : None,
	    fn_return_type : frt,
	    fn_args : None,
	    fn_body : None,
	}
    }
}


pub fn find_functions_in_scope(parsingvec : Vec<ParsingData> , scope : String )-> Vec<ParsingData>{

    let mut index = 0;
    let len = parsingvec.len();
    let mut retval   : Vec<ParsingData> = Vec::new();
    
    while index < len{

	let mut i = 0;
	let mut j = 0;
	if index + 5 < len {

	    i = is_def_with_args(parsingvec.clone() , index);
	    j = is_def_no_args(parsingvec.clone() , index);
	    
	}
	
	if i > 0{

	    let mut function_val = FunctionDef::new(

		get_function_id(parsingvec[index].clone()),
		get_function_return_type(parsingvec[index + 2].clone()),
		scope.clone()
		    
	    );
	    index = i;
	    let mut args : Vec<Lexeme> = Vec::new();
	    index = get_function_args(
		parsingvec.clone(),
		&mut args,
		index ,
	    );
	    let mut body : Vec<ParsingData> = Vec::new();
	    index = get_function_block_body(
		parsingvec.clone(),
		&mut body,
		index ,
	    );

	    function_val.fn_type = Some(FunctionDefType::DEF_WITH_ARGS);
	    function_val.fn_args = Some(get_variable_def_from_args(args));
	    function_val.fn_body = Some(
		Block{
		    scope : format!("{}_{}", function_val.super_scope , function_val.fn_id),
		    block : body,
		}
	    );
	    retval.push(ParsingData::functiondef(function_val));
	}else if j > 0{
	    
	    let mut function_val = FunctionDef::new(
		
		get_function_id(parsingvec[index].clone()),
		get_function_return_type(parsingvec[index + 2].clone()),
		scope.clone()
		    
	    );
	    index = j ;
	    let mut body : Vec<ParsingData> = Vec::new();
	    index = get_function_block_body(
		parsingvec.clone(),
		&mut body,
		index ,
	    );
	    
	    function_val.fn_type = Some(FunctionDefType::DEF_NO_ARGS);
	    function_val.fn_body = Some(
		Block{
		    scope : format!("{}_{}", function_val.super_scope , function_val.fn_id),
		    block : body,
		}
	    );
	    retval.push(ParsingData::functiondef(function_val));
	}else {
	    retval.push(parsingvec[index].clone());
	    index += 1;
	}
	
	
    }

    return retval;
}


pub fn get_function_block_body(parsingvec : Vec<ParsingData> , body : &mut Vec<ParsingData> , current_index : usize) -> usize{

    let mut index = current_index;
    println!("\n\n\nsjdasjdkaskldaskj {:#?}",parsingvec[index].clone());
    println!("sjdasjdkaskldaskj {:#?}\n\n\n\n",parsingvec[index + 1].clone());
    let mut inner = 0;
    
    loop {
	if let ParsingData::lexeme(lex ) = parsingvec[index].clone(){
	    
	    if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_))){
		body.push(parsingvec[index].clone());
		inner += 1;
	    }
	    else if matches!(lex.tokens , Token::t_stc(STC::stc_scope_end(_))){
		if inner == 0 {
		    break;
		}else {
		    body.push(parsingvec[index].clone());
		    inner -= 1;
		}
	    }else {
		body.push(parsingvec[index].clone());
	    }
	}else {
	    body.push(parsingvec[index].clone());
	}
	index += 1;
    }

    return index + 1;
    
}

pub fn get_function_args(parsingvec : Vec<ParsingData> , args : &mut Vec<Lexeme> , current_index : usize) -> usize{
    
    let mut index = current_index ;
    let mut inner = 0;
    
    loop {
	if let ParsingData::lexeme(lex ) = parsingvec[index].clone(){
	    
	    if matches!(lex.tokens , Token::t_stc(STC::stc_arg_begin(_))){
		args.push(lex);
		inner += 1;
	    }
	    else if matches!(lex.tokens , Token::t_stc(STC::stc_arg_end(_))){
		if inner == 0 {
		    break;
		}else {
		    args.push(lex);
		    inner -= 1;
		}
	    }else {
		args.push(lex);
	    }
	}
	index += 1;
    }

    return index + 2 ;
}
