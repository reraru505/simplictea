extern crate lexer;
extern crate lex_validator;
extern crate parser;
extern crate cwrite;


use crate::lexer::lex::Lexeme;
use crate::parser::parse;
use crate::cwrite::cwrite;
use crate::lex_validator::validate;


fn main() {

    let lexemes : Vec<Lexeme> = lexer::lexer("./Examples/example1.spt".to_string());
    //let lexemes = validate("./Examples/example1.spt".to_string() , lexemes);
    
    for i in lexemes.iter(){
 	println!("{:#?}", i);
    }
    let parsing_data = parse(lexemes);
  //  cwrite(parsing_data);
    
    
    
}
