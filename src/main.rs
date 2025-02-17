extern crate lexer;
extern crate parser;

use crate::lexer::lex::Lexeme;
use crate::parser::parse;

fn main() {

    let lexemes : Vec<Lexeme> = lexer::lexer("/home/fnln/dev/dummy/dummy.crs".to_string());

   for i in lexemes.clone() {
   	println!("{:?}",i.tokens);
    }
    
    parse(lexemes);
    
}
