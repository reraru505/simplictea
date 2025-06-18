use lexer::token_type::Token;

use crate::preparser::preparsingdata::PreParsingData;

pub enum DataType{
    I32,
    I64,
    U32,
    U64,
}


pub struct Variable_Definition{
    pub scope : String ,
    pub id : String ,
    pub datatype : DataType,
    pub value : Option<Vec<Expression>>,
}


pub struct Variable_Assignment{
    pub scope : String ,
    pub id : String ,
    pub value : Option<Vec<Expression>>,
}
 

pub struct Function_Definition{
    pub scope : String,
    pub id : String ,
    pub datatype : DataType,
    pub body : Vec<Expression>,
    pub args : Option<Vec<Variable_Definition>>,
}

pub struct Function_Call{
    pub scope : String ,
    pub id : String ,
    pub args : Vec<Variable_Definition> ,
}

pub struct Return_Statement{
    pub scope : String , 
    pub args : Vec<Variable_Definition> ,
}

pub struct Binary_Expression{
    pub scope : String ,
    pub args : Vec<Variable_Definition>,
}


pub enum Expression{

     FUNCTION_DEFINITION (Function_Definition),
     FUNCTION_CALL (Function_Call),

     VARIABLE_DEFINITION (Variable_Definition),
     VARIABLE_ASSIGNMENT (Variable_Assignment),

     RETURN_STATEMENT (Return_Statement),
//     CONDITIONAL_STATEMENT ,

//     ITERATOR,

     BINARY_EXPRESSION (Binary_Expression),
     LITERAL(Token),
 }


pub fn classify_functions(ppd : Vec<PreParsingData> ) -> Vec<Expression> {



}

pub fn is_function_definition(e : PreParsingData) -> Option<Expression>{

    let mut id : String = String::new();
    let mut DataType = DataType::I32;




    if e.is_expression(){
        if let PreParsingData::expression(exp ) = e {

            if exp.len() > 3 {
                if matches!(exp[0] , Token::t_identifier(_)){

                }
            }
        }
    }

    return None;
}
