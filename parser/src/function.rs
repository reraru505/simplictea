use crate::parsingdata::{ParsingData , Block};
use crate::lexer::token_type::*;
use crate::symboltable::symbol::DataType;
use crate::lexer::lex::Lexeme;
use crate::variable::{Variable , get_variable_def_from_args , find_variable_declarations_in_scope};
use crate::function_return::find_returns_inside_scope;

#[derive(Debug)]
pub enum FunctionDefType{

    DEF_WITH_ARGS,
    DEF_NO_ARGS,

}

#[derive(Debug)]
pub struct FunctionDef{

    pub fn_id : Token ,
    pub fn_type : Option<FunctionDefType>,
    pub fn_return_type : DataType,
    pub fn_args : Option< Vec<Variable>>,
    pub fn_body : Option<Block>,
}

impl FunctionDef{
    pub fn new(fid : Token , frt : DataType ) -> Self{
	Self{
	    fn_id : fid ,
	    fn_type : None,
	    fn_return_type : frt,
	    fn_args : None,
	    fn_body : None,
	}
    }
}


pub fn find_all_function_def(parsingvec : Vec<ParsingData>) -> Vec<ParsingData>{

    let mut context : Vec<ParsingData> = Vec::new();
    let mut retval  : Vec<ParsingData> = Vec::new();
    
    for i in 0.. parsingvec.len(){

	if i > 3{
	        context = parsingvec[i-4 .. i].to_vec();
	}

	if is_function_def(context.clone()){

	    for _ in 0..4{
		retval.pop();
	    }
	    retval.push(ParsingData::functiondef(FunctionDef::new(get_function_id(context.clone()),
								  get_function_return_type(context.clone()))));
	    context.clear();
	}
	retval.push(parsingvec[i].clone());
	

	
    }
    
    return retval;
}

// this depends on find all function args to run first
// message for myself , please dont call this before and try to look for the fucking bugs

pub fn find_all_function_args(parsingvec : Vec<ParsingData>) -> Vec<ParsingData>{

    let mut retval  : Vec<ParsingData> = Vec::new();

    let mut i = 0;
    while i < parsingvec.len() {

	if let ParsingData::functiondef(mut s) = parsingvec[i].clone() {

	    if let ParsingData::lexeme(v) = parsingvec[i + 1].clone() {

		if matches!(v.tokens , Token::t_stc(STC::stc_scope_begin(_))){

		    s.fn_type = Some(FunctionDefType::DEF_NO_ARGS);
		    retval.push(ParsingData::functiondef(s));
		    i += 1;
		    
		}else if matches!(v.tokens , Token::t_stc(STC::stc_arg_begin(_))){

		    s.fn_type = Some(FunctionDefType::DEF_WITH_ARGS);

		    retval.push(ParsingData::functiondef(s));
		    
		    let mut pushval : Vec<Lexeme> = Vec::new();

		    i += 2;

		    loop {
			if let ParsingData::lexeme(l) = parsingvec[i].clone(){
			    if matches!(l.tokens , Token::t_stc(STC::stc_arg_end(_))){
				break;
			    }else{
				pushval.push(l);
			    }
			}
			i += 1;
		    }
		    
		    retval.push(ParsingData::temp_arg_indicator(pushval));
		    i += 1;
		}

		
		
	    }
	  
	}else {
	    retval.push(parsingvec[i].clone());
	    i += 1;
	}

	
    }

    return retval;
    
}


pub fn find_all_function_blocks(parsingvec : Vec<ParsingData>) -> Vec<ParsingData>{

    let mut retval  : Vec<ParsingData> = Vec::new();

    let mut i = 0;
    while i < parsingvec.len() {

	if let ParsingData::functiondef(mut tfndef) = parsingvec[i].clone(){
	    
	    if matches!(tfndef.clone().fn_type.unwrap() , FunctionDefType::DEF_NO_ARGS){

		if let ParsingData::lexeme(tlex) = parsingvec[i + 1].clone(){
		    if matches!(tlex.tokens , Token::t_stc(STC::stc_scope_begin(_))){

			i += 2;
			let mut inner = 0;
			let mut pushval : Vec<ParsingData> = Vec::new(); 
			
			loop {
			    if let ParsingData::lexeme(l) = parsingvec[i].clone(){

				if matches!(l.tokens , Token::t_stc(STC::stc_scope_begin(_))){
				    inner += 1;
				}else if matches!(l.tokens , Token::t_stc(STC::stc_scope_end(_))){
				    if inner == 0 {
					break;
				    }else {
					inner -= 1;
				    }
				    
				}else{
				    pushval.push(parsingvec[i].clone());
				}
			    }
			    i += 1;
			}

			if let Token::t_identifier(name) = tfndef.fn_id.clone() {
			    tfndef.fn_body = Some (Block{scope : name , block : pushval});
			}
			retval.push(ParsingData::functiondef(tfndef));
		    }
		
		} 

	    }else if matches!(tfndef.clone().fn_type.unwrap() , FunctionDefType::DEF_WITH_ARGS){

		if matches!(parsingvec[i + 1] , ParsingData::temp_arg_indicator(_)){
		    if let ParsingData::lexeme(tlex) = parsingvec[i + 2].clone(){
			if matches!(tlex.tokens , Token::t_stc(STC::stc_scope_begin(_))){

			    let preserved = i + 1;
			    i += 3;
			    let mut inner = 0;
			    let mut pushval : Vec<ParsingData> = Vec::new();
			    
			    loop {
				if let ParsingData::lexeme(l) = parsingvec[i].clone(){

				    if matches!(l.tokens , Token::t_stc(STC::stc_scope_begin(_))){
					inner += 1;
				    }else if matches!(l.tokens , Token::t_stc(STC::stc_scope_end(_))){
					if inner == 0 {
					    break;
					}else {
					    inner -= 1;
					}
					
				    }else{
					pushval.push(parsingvec[i].clone());
				    }
				}
				i += 1;
			    }
			    if let Token::t_identifier(name) = tfndef.fn_id.clone() {
				tfndef.fn_body = Some (Block{scope : name , block : pushval});
			    }
			    
			    retval.push(ParsingData::functiondef(tfndef));
			    retval.push(parsingvec[preserved].clone());
			    
			}
		    }
		
		} 
	    }
	    i+= 1;
	}else {
	    retval.push(parsingvec[i].clone());
	    i+= 1;
	}
    }
    return retval;
    
}


pub fn hanble_function_args(parsingvec : Vec<ParsingData>) -> Vec<ParsingData>{

    let len = parsingvec.len();
    let mut retval : Vec<ParsingData> = Vec::new();
    for i in 0 .. len {
	if let ParsingData::functiondef(mut s) = parsingvec[i].clone(){
	    if matches!(s.fn_type , Some(FunctionDefType::DEF_WITH_ARGS)){

		if let ParsingData::temp_arg_indicator(v) = parsingvec[i + 1].clone(){
		    s.fn_args = Some(get_variable_def_from_args(v));
		    retval.push(ParsingData::functiondef(s));
		} 
		
	    }else{
		retval.push(parsingvec[i].clone());
	    }
	}
    }

    return retval;
    
}


pub fn handle_variable_defs_in_functions(parsingvec : Vec<ParsingData>) -> Vec<ParsingData>{

    let mut retval : Vec<ParsingData> = Vec::new();
    
    for i in parsingvec.iter(){

	if matches!(i , ParsingData::functiondef(_)){
	    if let ParsingData::functiondef(mut fdef) = i.clone() {
		if let Some(mut block ) = fdef.fn_body{
		    
		    block.block = find_variable_declarations_in_scope(block.block.clone() , block.scope.clone());
		    //println!("{:#?}", block);
		    fdef.fn_body = Some(block.clone());
		}

		retval.push(ParsingData::functiondef(fdef));
	    }
	}else{
	    retval.push(i.clone());
	}
	
    }
    return retval;
    
}


pub fn handle_return_statements_in_functions(parsingvec : Vec<ParsingData>) -> Vec<ParsingData>{

    let mut retval : Vec<ParsingData> = Vec::new();
    
    for i in parsingvec.iter(){

	if matches!(i , ParsingData::functiondef(_)){
	    if let ParsingData::functiondef(mut fdef) = i.clone() {
		if let Some(mut block ) = fdef.fn_body{
		    
		    block.block = find_returns_inside_scope(block.block.clone() , block.scope.clone());
		    //println!("{:#?}", block);
		    fdef.fn_body = Some(block.clone());
		}

		retval.push(ParsingData::functiondef(fdef));
	    }
	}else{
	    retval.push(i.clone());
	}
	
    }
    return retval;
    
}


pub fn get_function_return_type(in_context : Vec<ParsingData>) -> DataType{

    if let ParsingData::lexeme(s) = in_context[2].clone() {
	match s.tokens {
	    
	    Token::t_keyword(Keyword::data_type(Data_Type::CHAR(_))) => return DataType::CHAR,
	    Token::t_keyword(Keyword::data_type(Data_Type::I32(_))) => return DataType::I32,
	    Token::t_keyword(Keyword::data_type(Data_Type::I64(_))) => return DataType::I64,
	    Token::t_keyword(Keyword::data_type(Data_Type::F32(_))) => return DataType::F32,
	    Token::t_keyword(Keyword::data_type(Data_Type::F64(_))) => return DataType::F64,
	    Token::t_keyword(Keyword::data_type(Data_Type::STRING(_)))  => return DataType::STRING,
	    Token::t_keyword(Keyword::data_type(Data_Type::VOID(_))) => return DataType::VOID,
	    _ => return DataType::VOID, //this case will not be possible but the rust compiler is a bitch
	}
    }
    return DataType::VOID;//again rust compiler is a bitch
}

pub fn get_function_id(in_context : Vec<ParsingData>) -> Token{

    if let ParsingData::lexeme(s) = in_context[0].clone(){
	return s.tokens;
    }
    return Token::t_identifier("THIS_CASE_CAN_NOT_OCCUR".to_string());
}

pub fn is_function_def(in_context : Vec<ParsingData>) -> bool{

    if in_context.len() < 4 {
	return false;
    }

    let mut retval = false;

    for i in 0..4 {
	if let ParsingData::lexeme(s) = in_context[i].clone(){
	    match i{
		0 => if matches!(s.tokens , Token::t_identifier(_)){retval = true;}
		else{return false; },
		1 => if matches!(s.tokens , Token::t_operator(Operator::type_assignment_op(_))){retval = true;}
		else{return false; },
		2 => if matches!(s.tokens , Token::t_keyword(Keyword::data_type(_))){retval = true;}
		else{return false; },
		3 => if matches!(s.tokens , Token::t_keyword(Keyword::statement(Statement::function_marker(_)))){retval = true;}
		else{return false; },
		_ => return false,
	    }
	    
	} 
    }
    

    return retval;
    
}

