extern crate lexer;
extern crate lex_validator;
extern crate parser;
extern crate cwrite;
extern crate target_qbe;



use lexer::lex;
use lexer::lexer;
use lexer::token_type;

use crate::lexer::lex::Lexeme;
use crate::lex_validator::validate;

use crate::parser::parser::Parser;
use crate::parser::statement_parser;
use crate::target_qbe::gen_ir::STATEMENT;


fn main() {

    let a_lexemes : Vec<Lexeme> = lexer::lexer("./Examples/testing.spt".to_string());

    let mut lexemes : Vec<Lexeme> = Vec::new(); 

    for mut i in a_lexemes.iter() {
        if matches!(i.tokens , lexer::token_type::Token::t_keyword(lexer::token_type::Keyword::statement(lexer::token_type::Statement::or_operator(_)))){
            lexemes.push(Lexeme { tokens: token_type::Token::t_operator(token_type::Operator::or_op("or".to_string())), position: i.position.clone() });
        }else if matches!(i.tokens , lexer::token_type::Token::t_keyword(lexer::token_type::Keyword::statement(lexer::token_type::Statement::and_operator(_)))){
            lexemes.push(Lexeme { tokens: token_type::Token::t_operator(token_type::Operator::and_op("and".to_string())), position: i.position.clone() });
        }else{
            lexemes.push(i.clone());
        }  
    }
   // let lexemes = validate("./Examples/testing.spt".to_string() , lexemes);

   for i in lexemes.iter() {
       println!("{:#?}" , i);
   }

   println!(" len of lexeme = {}" , lexemes.len());
    let mut parser = Parser::new(lexemes);

    let retval = parser.parse();

    println!("{:#?}" , retval);

    let mut statement = STATEMENT::new(retval.clone());
    statement.th.find_all_types_in_ast(retval);
    statement.try_and_find();

    println!("{}\n{}\n{}", statement.th.write_types() , statement.gh.write_globals(), statement.writer);

   
}



