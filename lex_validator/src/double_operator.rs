use crate::lexer::token_type::{Position , Token , STC , Literal , Operator  };
use crate::lexer::lex::Lexeme;
use crate::error_handler::{ ErrorQueue , ErrorType };


pub fn merge_operators(in_lexemes : Vec<Lexeme> ,  error_handler : &mut ErrorQueue) -> Vec<Lexeme>{

    let mut index = 0;
    let len = in_lexemes.len();
    let mut retval : Vec<Lexeme> = Vec::new();
    
    
    
    while index < len {

	
	if index + 1 < len && matches!(in_lexemes[index].tokens.clone() , Token::t_operator(_)){

	    if matches!(in_lexemes[index + 1].tokens.clone() , Token::t_operator(_)){
		
		if let Some(token ) = check_operator_validity(in_lexemes[index].tokens.clone() ,
							      in_lexemes[index + 1].tokens.clone()){
		    retval.push(Lexeme{
			tokens : token ,
			position : in_lexemes[index].position.clone(),
		    });
		    index += 1;
		}else {
		    retval.push(in_lexemes[index].clone());
		    error_handler.push(
			format!("The [{}{}] is not valid operator",
				get_operator_val_from_lexeme(in_lexemes[index].clone()),
				get_operator_val_from_lexeme(in_lexemes[index].clone())),
			in_lexemes[index].position.clone(),
			in_lexemes[index + 2].position.clone(),
			ErrorType::Syntax_error
		    );
		}
	    }else {
		retval.push(in_lexemes[index].clone());
	    }
	    
	    
	}else {
	    retval.push(in_lexemes[index].clone());

	}
	
	index += 1;
    }

    return retval;
}




fn get_operator_val_from_lexeme(lex : Lexeme ) -> String{
    if let Token::t_operator(op ) = lex.tokens {
	return get_operator_val(op);
    }
    return "function is guarded and will not happen".to_string();
}
			

fn get_operator_val(operator : Operator ) -> String{

    match operator{
	
	Operator::assignment_op(s) => return s,      // =
	Operator::type_assignment_op(s) => return s, // :
	Operator::addition_op(s) => return s,        // +
	Operator::subtraction_op(s) => return s,     // -
	Operator::multiplication_op(s) => return s,  // *
	Operator::division_op(s) => return s,        // /

	//comparision operatos
	Operator::not_op(s) => return s,             // !
	Operator::check_equal_op(s) => return s,     // ==
	Operator::not_equal_op(s) => return s,       // !=
	Operator::greater_than_op(s) => return s,    // >
	Operator::lesser_than_op(s) => return s,     // <

	//logical operators  
	Operator::and_op(s) => return s,      // and
	Operator::or_op(s) => return s,  	// or
	Operator::xor_op(s) => return s,	// xor
	
    }
    
}


pub fn check_operator_validity(first : Token , second : Token ) -> Option<Token>{

    let mut context : Vec<String> = Vec::new();
    
    match (first , second ){
	(Token::t_operator(Operator::assignment_op(f)) , Token::t_operator(Operator::assignment_op(s))) => {
	    context.push(f);
	    context.push(s);
	    return Some(
		Token::t_operator(
		    Operator::check_equal_op(context.join(""))
		)
	    );
	},
	
	(Token::t_operator(Operator::not_op(f)) , Token::t_operator(Operator::assignment_op(s))) => {
	    context.push(f);
	    context.push(s);
	    return Some(
		Token::t_operator(
		    Operator::not_equal_op(context.join(""))
		)
	    );
	},
	_ => return None,
    }
    
}
