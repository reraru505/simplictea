use crate::lexer::token_type::{Token , Keyword , Statement};
use crate::lexer::lex::Lexeme;

pub fn find_else_if_statement(in_lexemes : Vec<Lexeme>) -> Vec<Lexeme>{

    let mut index = 0;
    let len = in_lexemes.len();
    let mut retval : Vec<Lexeme> = Vec::new();

    while index < len {

	if index + 1 < len &&
	    matches!(in_lexemes[index].tokens , Token::t_keyword(Keyword::statement(Statement::else_statement(_)))) &&
	    matches!(in_lexemes[index + 1].tokens , Token::t_keyword(Keyword::statement(Statement::if_statement(_)))){
		retval.push(
		    Lexeme{
			tokens : Token::t_keyword(Keyword::statement(Statement::else_if_statement("else if".to_string()))).clone(),
			position : in_lexemes[index].position.clone(),
		    }
		);
		index += 1;
	    }else {
		retval.push(in_lexemes[index].clone())
	    }
	index += 1;
    }

    return retval;
}
