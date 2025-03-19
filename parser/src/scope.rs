use crate::expressions::Expression;
use crate::lexer::lex::Lexeme;

#[derive(Debug)]
pub struct Block {
    pub scope: String,
    pub body: Vec<Expression>,
}

impl Block {
    pub fn new(in_scope: String) -> Self {
        Self {
            scope: in_scope,
            body: Vec::new(),
        }
    }
}

pub fn create_exp_vec(lexvec: Vec<Lexeme>) -> Vec<Expression> {
    let mut retvec: Vec<Expression> = Vec::new();

    for i in lexvec.iter() {
        retvec.push(Expression::token(i.tokens.clone()));
    }

    return retvec;
}

//call this to create global scope for the program
pub fn create_glocbal_scope(expvec: Vec<Expression>) -> Block {
    Block {
        scope: "Global".to_string(),
        body: expvec,
    }
}
