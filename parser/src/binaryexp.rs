use crate::lexer::token_type::Token;

#[derive(Debug)]
pub enum Operator {
    ADDITION,
    SUBTRACTION,
    DIVISION,
    MULTIPLICATION,

    ASSIGNMENT,

    AND,
    OR,
    NOT,

    CHECKEQUAL,
    CHECKNEQUAL,

    GREATER,
    LESSER,

    BRACKET,
}

#[derive(Debug)]
pub struct BinaryExpressionTree {
    pub super_scope: String,
    pub tree: Vec<BinaryExpression>,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub id: Token,
    pub operator: Operator,
    pub left: Token,
    pub right: Token,
}

pub fn new_binary_expression_identifier (name: String) -> Token {
    return Token::t_identifier(name);
}

impl BinaryExpression {
    pub fn new(name : String ) -> Self {
         Slef {
             super_scope : name ,
             tree : Vec::new(),
         }
    }
}
