use crate::lexer::token_type::Token;


// step one 

pub struct Expression{
   pub body  : Vec<Token>,
   pub scope : String ,
}

pub struct Block{
   pub body : Vec<Expression>,
   pub scope : String ,
   pub id : String ,
}


pub enum ParsingData{
    expression(Expression),
    block(Block),
}


impl Clone for Expression {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            scope: self.scope.clone(),
        }
    }
}


impl Clone for Block {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            scope: self.scope.clone(),
            id: self.id.clone(),
        }
    }
}

impl Clone for ParsingData {
    fn clone(&self) -> Self {
        match self {
            ParsingData::expression(expr) => ParsingData::expression(expr.clone()),
            ParsingData::block(block) => ParsingData::block(block.clone()),
        }
    }
}
