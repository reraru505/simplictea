use crate::parsingdata::ParsingData ;
use crate::lexer::token_type::*;
use crate::symboltable::symbol::DataType;


pub fn is_def_with_args(parsingvec : Vec<ParsingData> , index : usize ) -> usize {

    let mut retval = 0;
    
    if let ParsingData::lexeme(lex) = parsingvec[index].clone(){
	if matches!(lex.tokens , Token::t_identifier(_)){
	    retval = index;
	}else {
	    return 0;
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[index + 1].clone(){
	
	if matches!(lex.tokens , Token::t_operator(Operator::type_assignment_op(_))){

	    retval += 1;
			
	}else {

	    return 0;
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[index + 2].clone(){
	if matches!(lex.tokens , Token::t_keyword(Keyword::data_type(_)) ){
		retval += 1;
	}else {
	    return 0;
	    
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[index + 3].clone(){
	if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::function_marker(_))) ){

	    retval += 1;
	    
	}else {
	    return 0;
	    
	}
	
    }
    
    if let ParsingData::lexeme(lex) = parsingvec[index + 4].clone(){
	if matches!(lex.tokens , Token::t_stc(STC::stc_arg_begin(_)) ){

	    retval += 1;
	    
	}else {
	    return 0;
	    
	}
	
    }

    return retval + 1;
}



pub fn is_def_no_args(parsingvec : Vec<ParsingData> , index : usize ) -> usize {

    let mut retval = 0;
    
    if let ParsingData::lexeme(lex) = parsingvec[index].clone(){
	if matches!(lex.tokens , Token::t_identifier(_)){
	    retval = index;
	}else {
	    return 0;
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[index + 1].clone(){
	
	if matches!(lex.tokens , Token::t_operator(Operator::type_assignment_op(_))){

	    retval += 1;
			
	}else {

	    return 0;
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[index + 2].clone(){
	if matches!(lex.tokens , Token::t_keyword(Keyword::data_type(_)) ){
		retval += 1;
	}else {
	    return 0;
	    
	}
    }
    if let ParsingData::lexeme(lex) = parsingvec[index + 3].clone(){
	if matches!(lex.tokens , Token::t_keyword(Keyword::statement(Statement::function_marker(_))) ){

	    retval += 1;
	    
	}else {
	    return 0;
	    
	}
	
    }
    
    if let ParsingData::lexeme(lex) = parsingvec[index + 4].clone(){
	if matches!(lex.tokens , Token::t_stc(STC::stc_scope_begin(_)) ){

	    retval += 1;
	    
	}else {
	    return 0;
	    
	}
	
    }

    return retval + 1;
}


pub fn get_function_return_type(in_context : ParsingData) -> DataType{

    if let ParsingData::lexeme(s) = in_context.clone() {
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

pub fn get_function_id(in_token : ParsingData) -> String{

    if let ParsingData::lexeme(lexeme) = in_token{

	if let Token::t_identifier(name) = lexeme.tokens{
	    return name;
	}
	
    }
    return "THIS_CASE_CAN_NOT_OCCUR".to_string();
}


