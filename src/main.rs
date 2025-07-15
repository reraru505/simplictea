extern crate lexer;
extern crate lex_validator;
extern crate parser;
extern crate cwrite;




use lexer::lex;
use lexer::lexer;

use crate::lexer::lex::Lexeme;
use crate::lex_validator::validate;

use crate::parser::parser::Parser;
use crate::parser::statement_parser;


fn main() {

    let lexemes : Vec<Lexeme> = lexer::lexer("./Examples/testing.spt".to_string());
    let lexemes = validate("./Examples/testing.spt".to_string() , lexemes);

   for i in lexemes.iter() {
       println!("{:#?}" , i);
   }

   println!(" len of lexeme = {}" , lexemes.len());
    let mut parser = Parser::new(lexemes);

    let retval = parser.parse();

    println!("{:#?}" , retval);
    
}



