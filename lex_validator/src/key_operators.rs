use crate::lexer::token_type::{Token , Operator , Keyword , Statement };
use crate::lexer::lex::Lexeme;
use crate::error_handler::{ ErrorQueue , ErrorType };

pub fn find_operators_as_keywords(inlexemes : Vec<Lexeme>) -> Vec<Lexeme>{

    let mut retval : Vec<Lexeme> = Vec::new();

    for i in inlexemes.iter(){
	
	if let Some(operator ) = key_to_operator(i.tokens.clone()){
	    retval.push(
		Lexeme{
		    tokens : Token::t_operator(operator),
		    position : i.position.clone(),
		}
	    );
	}else {
	    retval.push(i.clone());
	}
    }
    return retval;
}

fn key_to_operator(token : Token) -> Option<Operator>{

    match token {
	Token::t_keyword(Keyword::statement(Statement::and_operator(op))) => return Some(Operator::and_op(op)),
	Token::t_keyword(Keyword::statement(Statement::or_operator(op))) => return Some(Operator::or_op(op)),
	Token::t_keyword(Keyword::statement(Statement::xor_operator(op))) => return Some(Operator::xor_op(op)),
	_ => return None,
    }
    
}
