use crate::lexer::token_type::{Position , Token , STC , Literal };
use crate::lexer::lex::Lexeme;
use crate::error_handler::{ ErrorQueue , ErrorType };



pub fn merge_decimal(lexemevec : Vec<Lexeme> , error_handler : &mut ErrorQueue) -> Vec<Lexeme>{

    let mut retval : Vec<Lexeme> = Vec::new();
    let mut buffer : Vec<String> =  Vec::new();

    let len = lexemevec.len();
    let mut i = 0;
    
    while i < len {
	
	if let Token::t_literal(Literal::integer_literal(in_lit)) = lexemevec[i].clone().tokens {
	    buffer.push(in_lit);
	    if i < len - 2{
		if let Token::t_stc(STC::stc_dot(in_dot)) = lexemevec[i + 1].clone().tokens {
		    let last_position = lexemevec[i].clone().position; 
		    buffer.push(in_dot);
		    i += 2;
		    loop{
			
			if let Token::t_literal(Literal::integer_literal(in_else)) =  lexemevec[i].clone().tokens{
			    buffer.push(in_else);
			}else if let Token::t_stc(STC::stc_dot(in_else))  = lexemevec[i].clone().tokens {
			    buffer.push(in_else);
			}else{
			    break;
			}

			i += 1;
		    }

		    i -= 1;
		    let next_position = lexemevec[i + 1].clone().position;

		    check_decimal_validity(buffer.clone().join("") , last_position.clone() , next_position , error_handler);
		    
		    retval.push(Lexeme{tokens : Token::t_literal(Literal::decimal_literal(buffer.join(""))) ,
				       position : last_position});
		    buffer.clear();
		}else {
		    retval.push(lexemevec[i].clone());
		    buffer.clear();
		}
	    }

	    
	}else {
	    retval.push(lexemevec[i].clone());
	    buffer.clear();
	}

	i += 1;
	
    }

    return retval;
}

pub fn check_decimal_validity( value : String  , pos : Position , next_pos : Position ,  error_handler : &mut ErrorQueue) {

    let valvec : Vec<char> = value.chars().collect();

    let mut num_dot = 0;
    for i in valvec.iter(){
	if *i == '.'{
	    num_dot += 1;
	}
    }

    if num_dot > 1 {
	
	error_handler.push(
	    format!("The value [{}] is not valid decimal expression", value),
	    pos ,
	    next_pos,
	    ErrorType::Syntax_error
	);
	
    }

}

