
use crate::lexer::token_type::Position;
use crate::lexer::tokenizer::Tokenizer;
use std::process;
use std::fmt;

pub enum ErrorType {
    Syntax_error ,
    Type_error,
}

impl fmt::Display for ErrorType {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::Syntax_error  => write!(f, "Syntax Error"),
            ErrorType::Type_error => write!(f, "Type Checker Error"),
        }
    }
}

struct Error{
    pub message : String ,
    pub start_pos : Position ,
    pub end_pos : Position ,
    pub err_type : ErrorType,
}

pub struct ErrorQueue{

    errors : Vec<Error>,
    file_data_as_lines : Vec<String>,
}



impl Error{

    pub fn new(in_message : String ,in_start_pos  :  Position , in_end_pos : Position  , in_err_type : ErrorType ) -> Self {
	Self {
	    message : in_message ,
	    start_pos : in_start_pos ,
	    end_pos : in_end_pos ,
	    err_type : in_err_type,
	    
	}
    }
    
}

impl ErrorQueue {

    pub fn new(path : String) -> Self {

	let tokenizer = Tokenizer ;
	let file_cont = tokenizer.load_source_file(path);
	let line_vec : Vec<String> = file_cont.split("\n").map(String::from).collect();

	
	Self{
	    errors : Vec::new(),
	    file_data_as_lines : line_vec, 
	    
	}
    }
    pub fn push(&mut self , in_message : String ,in_start_pos  :  Position , in_end_pos : Position , in_err_type : ErrorType){
	self.errors.push (
	    Error::new(in_message  ,in_start_pos , in_end_pos, in_err_type  )
	);
    }

    pub fn display_if_error(&self ){
	if self.errors.len() > 0{

	    for i in self.errors.iter(){
		
		println!("\n\x1b[31mError : \x1b[34m{} at [{} : {}]\x1b[0m\n",i.err_type , i.start_pos.y + 1 , i.start_pos.x );
		println!("--> \x1b[92m{}\x1b[0m\n",i.message );

		
		println!("--> \x1b[31mError at => \x1b[0m{}", self.file_data_as_lines[i.start_pos.y]);

		
		for _ in 0 .. i.start_pos.x + 16{
		    print!(" ");
		}
		print!("\x1b[31m");
    
		for _ in i.start_pos.x .. i.end_pos.x{
		    print!("^")
		}
		print!("\x1b[0m\n");
		
		
	    }
	    
	    

	    println!("\n\x1b[31mCompilation Terminated\x1b[0m\n ");
	    process::exit(1);
	}
    }
    
}
