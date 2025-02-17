#[allow(non_camel_case_types)]

pub mod token_type;
pub mod token_type_impl;

pub mod keywords;
pub mod literal;
pub mod stc;
pub mod operator;
pub mod identifier;

pub mod tokenizer;
pub mod lex;


use crate::lex::Lexeme;
use crate::lex::Lexer;

pub fn lexer(file_path : String) -> Vec<Lexeme> {

    let mut lexer_process = Lexer::new();
    lexer_process.lexx(file_path);

    return lexer_process.get_lexemes();
}

