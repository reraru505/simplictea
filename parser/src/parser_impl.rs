use std::panic;

use crate::parser::Parser;
use crate::ast::AST_Node;

impl Parser {

    pub fn parse_all(&mut self ) -> Vec<AST_Node>{

        let mut retval : Vec<AST_Node> = Vec::new();

        while !self.is_at_end() {

            match self.Expression_Parser() {
                Some(val)  => retval.push(val),
                None => {},
            }

        }

        return retval;
    }

    

}
