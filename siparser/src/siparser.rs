use crate::ast::{Node , Data_Type};
use crate::silexer::silex::{Token , Keyword , Operator , Special};

pub struct SiParser{
    pub tokens : Vec<Token>,
    pub index : usize,
}

impl SiParser {

    pub fn new(intokens : Vec<Token>) -> Self {
        Self { tokens: intokens, index: 0 }
    }
    
    pub fn function_declaration_parser(&mut self) -> Option<Node> {

        None
    }

    pub fn structure_declaration_parser(&mut self) -> Option<Node> {
        None
    }

    pub fn variable_declaration_parser(&mut self) -> Option<Node> {
        None
    }

    pub fn conditional_parser(&mut self ) -> Option<Node> {
        None
    }

    pub fn iterator_parser(&mut self) -> Option<Node> {
        None
    }

    pub fn assignment_parser(&mut self) -> Option<Node> {
        None
    }

}
