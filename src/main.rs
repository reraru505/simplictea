extern crate lexer;
extern crate lex_validator;
extern crate parser;
extern crate cwrite;
extern crate target_qbe;



use lexer::lex;
use lexer::lexer;

use crate::lexer::lex::Lexeme;
use crate::lex_validator::validate;

use crate::parser::parser::Parser;
use crate::parser::statement_parser;
use crate::target_qbe::sections::Custom_Types_Handler;


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

    let mut type_handler = Custom_Types_Handler::new();

    type_handler.find_all_types_in_ast(retval);
    type_handler.display_types();
    
}



