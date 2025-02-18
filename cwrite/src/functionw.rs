use crate::parser::parsingdata::ParsingData;
use crate::parser::function::{FunctionDef , FunctionDefType};
use crate::symboltable::symbol::DataType;
use crate::parser::variable::Variable;
use crate::lexer::token_type::Token;
use crate::variablew::write_variable;
use crate::returnw::write_return;

pub fn write_data_type(d_type : DataType ) -> String{
    match d_type{
	DataType::I32 => return "i32".to_string(),
	DataType::I64 => return "i64".to_string(),
	DataType::F32 => return "f32".to_string(),
	DataType::F64 => return "f64".to_string(),
	DataType::CHAR => return "char".to_string(),
	DataType::STRING => return "string".to_string(),
	DataType::VOID => return "void".to_string(),
    }
}

pub fn write_function_args(fn_args : Vec<Variable>) -> String{

    let mut retval : Vec<String> = Vec::new();
    retval.push("(".to_string());

    for i in 0 .. fn_args.len() - 1{
	retval.push(format!("{} {} , " , write_data_type(fn_args[i].clone().var_type) ,fn_args[i].var_id ));
    }
    
    retval.push(format!("{} {} " , write_data_type(fn_args[fn_args.len() - 1].clone().var_type),fn_args[fn_args.len() - 1].var_id ));

    retval.push(")".to_string());
    
    return retval.join("");
    
    
}

pub fn write_function_id(fid : Token) -> String{
    if let Token::t_identifier(s) = fid {
	return s;
    }else {
	// this case should not occur 
	return "error".to_string();
    }
}
    
pub fn write_function_def(fdef : FunctionDef ) -> String {
    
    if matches!(fdef.fn_type , Some(FunctionDefType::DEF_WITH_ARGS) ){
	return format!("{} {} {}",
		       write_data_type(fdef.fn_return_type ) ,
		       write_function_id(fdef.fn_id) ,
		       write_function_args(fdef.fn_args.unwrap()) );
    }else {
	return format!("{} {} {}",
		       write_data_type(fdef.fn_return_type ) ,
		       write_function_id(fdef.fn_id) ,
		       "()".to_string());
    }
    
} 

pub fn write_function_body(fdef : FunctionDef ) -> String{

    let mut retval : Vec<String> = Vec::new();
    retval.push("{\n".to_string());

    if let Some(block ) = fdef.fn_body {

//	println!("{:#?}", block);
	
	for i in block.block.iter(){

	    if matches!(i.clone() , ParsingData::function_return(_)){
		
		if let ParsingData::function_return(ret) = i.clone(){
		    retval.push(write_return(ret , fdef.fn_return_type.clone()));
		}
	    }
	    
	    if matches!(i.clone() , ParsingData::variable(_)){
		
		if let ParsingData::variable(var ) = i.clone(){
		    retval.push(write_variable(var));
		}
		
	    }

	   
	}
	
    }

    retval.push("}\n".to_string());

    return retval.join("");
    
} 
