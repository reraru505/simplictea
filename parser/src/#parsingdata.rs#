//use crate::expressions::*;
//use crate::lexer::token_type::*;

use crate::function::FunctionDef;
use crate::lexer::lex::Lexeme;
use crate::binaryexp::{BinaryExpressionBlock , BinaryExpressionTree};
use crate::variable::Variable;

struct Parser;

#[derive(Debug)]
pub enum ParsingData{
    lexeme( Lexeme)  ,
    binexp( BinaryExpressionBlock),
    functiondef (FunctionDef),
    temp_arg_indicator(Vec<Lexeme>),
    variable(Variable),
    function_return(BinaryExpressionTree),
}


#[derive(Debug)]
pub struct Block{
    pub scope : String,
    pub block : Vec<ParsingData>,
}

impl Clone for Block{
    fn clone (&self ) -> Self {
	Self {
	    scope : self.scope.clone(),
	    block : self.block.clone(),
	}
    }
}


impl ParsingData{
    pub fn generate(in_lexemes : Vec<Lexeme>) -> Vec<ParsingData>{
	let mut retval : Vec<ParsingData> = Vec::new();

	for i in in_lexemes.iter(){
	    retval.push(ParsingData::lexeme(i.clone()));
      	}

	return retval;
    }
}


impl Clone for ParsingData{

    fn clone(&self ) -> Self{
	match self {
	    ParsingData::lexeme(s) => return ParsingData::lexeme(s.clone()),
	    ParsingData::binexp(s) => return ParsingData::binexp(s.clone()),
	    ParsingData::functiondef(s) => return ParsingData::functiondef(s.clone()),
	    ParsingData::temp_arg_indicator(s) => return ParsingData::temp_arg_indicator(s.clone()),
	    ParsingData::variable(s) => return ParsingData::variable(s.clone()),
	    ParsingData::function_return(s) => return ParsingData::function_return(s.clone()),
	}
	
    }
    
}

