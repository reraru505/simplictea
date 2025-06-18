use crate::lexer::token_type::Token;
use crate::lexer::token_type::STC;


// first turn them into blocks 
// then make them into expressions

#[derive(Debug)]
pub enum PreParsingData{
    token(Token),
    block(Vec<PreParsingData>),
    expression(Vec<Token>),
}

impl Clone for PreParsingData{
    fn clone(&self) -> Self {
        match self {

            PreParsingData::token(t) =>      return PreParsingData::token(t.clone()),
            PreParsingData::block(b) =>      return PreParsingData::block(b.clone()),
            PreParsingData::expression(e) => return PreParsingData::expression(e.clone()),
        }
    }
}

impl PreParsingData {

    pub fn is_block(&self ) -> bool {

        if matches!(self , PreParsingData::block(_)){

            return true;
        }

        return false;
    } 


    pub fn is_token(&self ) -> bool {
       
        if matches!(self , PreParsingData::token(_)){

            return true;
        }

        return false;  
    }

    pub fn is_expression(&self ) -> bool {
       
        if matches!(self , PreParsingData::expression(_)){

            return true;
        }

        return false;  
    }
 
    pub fn is_scope_begin(&self ) -> bool {

        if self.is_token(){

            if matches!(self.get_token() , Token::t_stc(STC::stc_scope_begin(_))){
                return true;
            }else {
                return false;
            }
        }
        return false;
    }

   pub fn is_scope_end(&self ) -> bool {

        if self.is_token(){

            if matches!(self.get_token() , Token::t_stc(STC::stc_scope_end(_))){
                return true;
            }else {
                return false;
            }
        }
        return false;
    }  

     pub fn is_exp_end(&self ) -> bool {

        if self.is_token(){

            if matches!(self.get_token() , Token::t_stc(STC::stc_end_expression(_))){
                return true;
            }else {
                return false;
            }
        }
        return false;
    }  
  
    pub fn get_token(&self ) -> Token {

        if let PreParsingData::token(tok ) = self {
            
            return tok.clone();

        }else {

            panic!("Tried to access token in < PreParsingData > without checking ");
        }

    }

    pub fn print(&self ) {
        match self {
            PreParsingData::block(b) => {
                println!("\nBlock => \n");
                for i in b.iter(){
                    if i.is_token(){
                        i.print();
                    }else if i.is_expression() {
                        i.print();
                    }else {
                        i.print();
                    }
                }
                println!("\nBlock end\n");
            },
            PreParsingData::token(t) => {
                println!("{}" , t);
            },
            PreParsingData::expression(t ) => {
                println!("\nExpression =>\n");
                for i in t.iter(){
                    print!(" {} " , i);
                }
                println!("\nExpression end\n");
            }
        }
    }
}




