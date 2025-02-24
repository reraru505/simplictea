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
    let lexemes = validate("./Examples/example1.spt".to_string() , lexemes);

    //for i in lexemes.iter(){
    //	println!("{:#?}", i);
    //}
    let parsing_data = parse(lexemes);
    //    cwrite(parsing_data);
    
    
    
}



//use crate::lexer::token_type::Token;
//use crate::parser::binaryexp_handle::break_binary_expression;
//use crate::parser::binaryexp::BinaryExpressionTree;
//use crate::parser::binaryexp_helpers::print_binary_expression_tree_debug;


//testig
//    let mut tokvec : Vec<Token> = Vec::new();
//    for i in lexemes.iter(){
//	tokvec.push(i.tokens.clone());
//    }
//    let binaryexptree = break_binary_expression(&mut tokvec , "mero_scope" , &mut 0);
//    print_binary_expression_tree_debug(binaryexptree);
    
