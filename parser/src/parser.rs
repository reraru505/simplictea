use crate::expressions::*;
use crate::lexer::lex::Lexeme;
use crate::lexer::token_type::*;
use crate::parsingdata::*;
use crate::function::*;

//struct Parser;
//
//impl Parser {
//    
//    pub fn parse(in_lexemes : Vec<Lexeme>) -> Vec<Expression> {
//
//	let mut parsingvec = ParsingData::generate(in_lexemes.clone());
//	let mut context : Vec<ParsingData> = Vec::new();
//
//	let mut parsing = true;
//
//	while parsing {
//
//	    for (index , i) in parsingvec.iter().enumerate(){
//
//		if is_function_def(context.clone()){
//		    
//		}
//		
//		//if let ParsingData::lexeme(_) = i {
//		context.push(i.clone());
//		//}
//	    }
//	    
//	}
//		
//	
//    }
//}
//
//fn is_binary_expression(in_context : & Vec<Lexeme>) -> bool {
//
//    let mut context = in_context.clone();
//    if matches!(context.pop().unwrap().tokens ,
//		Token::t_stc(STC::stc_end_expression(_)) |
//		Token::t_stc(STC::stc_comma_seperator(_))) == false{
//
//	return false;
//	
//    } 
//    for i in context.iter(){
//	
//	if matches!(i.tokens , Token::t_identifier(_) | Token::t_literal(_)| Token::t_operator(_)) == false{
//	    return false;
//	}
//    }
//
//    return true;
//}
//
//
//fn is_fn_definition(in_context : & Vec<Lexeme>) -> bool {
//
//    let mut retval = false;
//
//    if in_context.len() < 4 {
//	return false;
//    }
//
//    
//    if matches!(in_context[0].tokens.clone() , Token::t_identifier(_)) &&
//	matches!(in_context[1].tokens.clone() , Token::t_operator(Operator::type_assignment_op(_))) 
//    
//    
//    return retval;
//}
