extern crate lexer;
extern crate lex_validator;
extern crate parser;
extern crate cwrite;




use lexer::lex;
use lexer::lexer;

use crate::lexer::lex::Lexeme;
use crate::lex_validator::validate;

use crate::parser::preparser::exprules::lexemes_to_preparsingdata;
use crate::parser::preparser::exprules::get_all_blocks;
use crate::parser::preparser::exprules::get_all_expressions;

fn main() {

    let lexemes : Vec<Lexeme> = lexer::lexer("./Examples/testing.spt".to_string());
    let lexemes = validate("./Examples/testing.spt".to_string() , lexemes);

    let prepar = lexemes_to_preparsingdata(lexemes);
    let prepar = get_all_blocks(prepar);
    let prepar = get_all_expressions(prepar); 

    for i in prepar.iter(){
        i.print();
    }
}



