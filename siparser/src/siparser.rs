use crate::ast::{Node , Data_Type};
use crate::silexer::silex::{Token , Keyword , Operator , Special};
use std::collections::HashMap;

pub struct SiTable{
    pub table : HashMap<String , Vec<String>>
}

impl SiTable {
    pub fn new() -> Self {
        Self { table: HashMap::new() }
    }

    pub fn register_struct(&mut self , name : String , items : Vec<String> ) {
        self.table.insert(name, items);
    } 

    pub fn export(&self) -> HashMap<String , Vec<String>> {
        return self.table.clone();
    }
}


pub struct SiParser{
    pub tokens : Vec<Token>,
    pub index : usize,
    pub map : SiTable,
}

impl SiParser {

    pub fn new(intokens : Vec<Token>) -> Self {
        Self { tokens: intokens, index: 0 , map : SiTable::new() }
    }
    
    pub fn function_declaration_parser(&mut self) -> Option<Node> {

        let checkpoint = self.index;

        let name = if let Some(n) = self.identifer_name_parser(){
            n
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if !self.is_colon(){
            return self.vomit_and_die(checkpoint);
        }

        let d_type = if let Some(d) = self.data_type_parser(){
            d
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if matches!(self.current() , Token::KEYWORD(Keyword::FN)){
            self.consume();
        }else{
            return self.vomit_and_die(checkpoint);
        }

        let mut params : Vec<Node> = Vec::new();

        if self.is_param_start(){
             params = self.function_param_parser();
        };

        let body : Vec<Node> ;

        if self.is_scope_start(){
            body = self.function_body_parser();
        }else{
            panic!("Function declaration does not have a body");
        }


        return Some(
            Node::FUNCTION_DEC { name: name, r_type: d_type , params: params , body: body }
        );
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

    pub fn function_body_parser(&mut self) -> Vec<Node> {

        let mut retvec : Vec<Node> = Vec::new();

        loop {
            if let Some(val )  = self.variable_declaration_parser(){
                retvec.push(val);
            }else if self.is_scope_end(){
                break;
            }
        } // loop end
        
        return retvec;
    }

}
