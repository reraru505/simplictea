use lexer::token_type::Data_Type;
use lexer::token_type::Token;

use lexer::token_type::Operator;
use lexer::token_type::Keyword;
use lexer::token_type::Statement;
use lexer::tokenizer::Tokenizer;


use crate::preparser::preparsingdata::PreParsingData;

pub enum DataType{
    I32,
    I64,
    F32,
    F64,
    CHAR,
    STRING,
    VOID,
}

impl DataType{
    pub fn get_datatype(inp : Data_Type) -> Self {
        match inp {
            Data_Type::I32(_) => return DataType::I32,
            Data_Type::I64(_) => return DataType::I64,

            Data_Type::F32(_) => return DataType::F32,
            Data_Type::F64(_) => return DataType::F64,

            Data_Type::CHAR(_) => return DataType::CHAR,
            Data_Type::VOID(_) => return DataType::VOID,
            Data_Type::STRING(_) => return DataType::STRING,
        }
    }
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


// function definition rules 
// <identifier> : <type> fn ()
// <identifier> : <type> fn (<variable definitio>)
// <identifier> : type fn {


pub fn is_function_definition(e : PreParsingData) -> Option<Expression>{

    let mut id : String = String::new();
    let mut datatype = DataType::I32;
    let mut is_function = false;



    if e.is_expression(){
        if let PreParsingData::expression(exp ) = e {

            if exp.len() > 4 {

                // checking if the first token is identifier 
                if matches!(exp[0] , Token::t_identifier(_)){
                    if let Token::t_identifier(nid ) = exp[0].clone(){
                        id = nid ;
                    }
                    is_function = true;
                }else{
                    return None;
                }

                // checking if the second token in a type assignment operator { : }
                if matches!(exp[1] , Token::t_operator(Operator::type_assignment_op(_))){

                    is_function = true;    

                }else{
                    return None;
                }

                // checking if the third token is a type
                if matches!(exp[2], Token::t_keyword(_)){
                    if let Token::t_keyword(Keyword::data_type(data )) = exp[2].clone(){
                        datatype = DataType::get_datatype(data);
                        is_function = true;
                    }else{
                        return None;
                    }
                }else {
                    return None;
                }

                // checking if the token in a function marker 
                if matches!(exp[3] , Token::t_keyword(Keyword::statement(Statement::function_marker(_)))){
                    is_function = true;
                }else{
                    return None;
                }

            }
        }


    }

    return Some(
        Expression::FUNCTION_DEFINITION(Function_Definition { 
            scope: String::new(), 
            id: id , 
            datatype: datatype, 
            body: Vec::new(),
            args: None,
        })
    );
}
