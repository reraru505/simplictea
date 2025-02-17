
use std::fs;
use std::process;


pub struct Tokenizer;

impl Tokenizer {

    pub fn is_space(&self , c : char ) -> bool{

	match c {
	    ' '  =>  true,
	    '\t' =>  true,
	    '\n' =>  true,
	    _ =>     false,
	}
    }


    pub fn is_operator(&self , c : char) -> bool{
	let operators = "=+-*/";

	for i in operators.chars(){
	    if i == c {
		return true;
	    }
	}
	return false;
	
    }

    pub fn is_stc(&self , c : char ) -> bool {

	let stc = "{}();.";

	for i in stc.chars(){
	    if i == c {
		return true;
	    }
	}
	return false;
    }

    pub fn load_source_file(&self , source_file : String ) -> String {

	let file_contents =  match fs::read_to_string(source_file){
	    Ok(v) => v,
	    Err(e) => {
		println!("{} " , e);
		process::exit(1);
	    },
	};
	
	return file_contents;
    }
    
}

