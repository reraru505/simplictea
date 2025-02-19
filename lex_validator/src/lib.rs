extern crate lexer;

pub mod decimal;

use std::process;
use crate::lexer::lex::Lexeme;
use crate::decimal::merge_decimal;
use crate::lexer::tokenizer::Tokenizer;

pub fn validate(file_path : String , lexvec : Vec<Lexeme>) -> Vec<Lexeme> {

    let tokenizer = Tokenizer;
    let file_cont = tokenizer.load_source_file(file_path);
    let line_vec : Vec<String> = file_cont.split("\n").map(String::from).collect();
    return merge_decimal(lexvec ,  line_vec);

    
}

pub fn print_error_line(line_vec : Vec<String> , line_index : usize , x_end : usize , x : usize ) {

    println!("--> \x1b[31mError at => \x1b[0m{}", line_vec[line_index]);

    
    for _ in 0 .. x + 16{
	print!(" ");
    }
    print!("\x1b[31m");
    
    for _ in x .. x_end{
	print!("^")
    }
    print!("\x1b[0m\n");
    println!("\n\x1b[31mCompilation Terminated\x1b[0m\n ");
    process::exit(1);
}
