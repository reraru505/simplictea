pub struct SyntaxErr{
    pub messages : Vec<(String , usize)>,
}

pub struct SemanticErr{
    pub messages : Vec<String> ,
}

impl SyntaxErr {

    pub fn add(&mut self , msg : String , index : usize ) {
        self.messages.push((msg , index));
    }
    
}
