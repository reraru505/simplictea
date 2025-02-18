extern crate lexer;
extern crate parser;
extern crate symboltable;

pub mod functionw;
pub mod variablew;
pub mod returnw;

use crate::parser::parsingdata::ParsingData;
use crate::functionw::{write_function_def ,write_function_body};

pub fn cwrite(parsingvec : Vec<ParsingData >){

    println!("#include <stdint.h>\n\
	      typedef int32_t i32;\n\
	      typedef int64_t i64;\n\
	      typedef float f32;\n\
	      typedef doube f64;\n");
    
    for i in parsingvec.iter(){

	if matches!(i , ParsingData::functiondef(_)){
	    if let ParsingData::functiondef(fndef) = i{
		println!("{}" , write_function_def(fndef.clone()));
		println!("{}" , write_function_body(fndef.clone()));

	    }
	}
	
    }
    
}
