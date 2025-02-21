use crate::parsingdata::{ParsingData , Block };

pub fn create_global_scope(parsingvec : Vec <ParsingData>) -> Block {

    Block{
	scope : "GLOBAL_SCOPE".to_string(),
	block : parsingvec,
    }
    
}


