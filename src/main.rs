extern crate lexer;
extern crate parser;
extern crate cwrite;

use crate::lexer::lex::Lexeme;
use crate::parser::parse;
use crate::cwrite::cwrite;

fn main() {

    let lexemes : Vec<Lexeme> = lexer::lexer("./Examples/example1.spt".to_string());
    let parsing_data = parse(lexemes);
    cwrite(parsing_data);
    
    
    
}
