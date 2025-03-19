use lexer::lex::Lexer;

use crate::binaryexp::{BinaryExpressionTree, Operator};
use crate::expressions::Expression;
use crate::lexer::token_type::Token;

pub fn check_precedence(op: Operator) -> usize {
    match op {
        Operator::ADDITION => return 1,
        Operator::SUBTRACTION => return 1,

        Operator::DIVISION => return 2,
        Operator::MULTIPLICATION => return 2,

        Operator::ASSIGNMENT => 0,

        Operator::AND => return 3,
        Operator::OR => return 3,

        Operator::CHECKEQUAL => return 4,
        Operator::CHECKNEQUAL => return 4,
        Operator::GREATER => return 4,
        Operator::LESSER => return 4,

        Operator::NOT => return 6,
        Operator::BRACKET => return 7,
    }
}

pub fn get_operator_from_token(tok: Token) -> Option<Operator> {
    match tok {
        Token::t_operator(lexer::token_type::Operator::assignment_op(_)) => {
            return Some(Operator::ASSIGNMENT)
        }

        Token::t_operator(lexer::token_type::Operator::addition_op(_)) => {
            return Some(Operator::ADDITION)
        }
        Token::t_operator(lexer::token_type::Operator::subtraction_op(_)) => {
            return Some(Operator::SUBTRACTION)
        }

        Token::t_operator(lexer::token_type::Operator::multiplication_op(_)) => {
            return Some(Operator::MULTIPLICATION)
        }
        Token::t_operator(lexer::token_type::Operator::division_op(_)) => {
            return Some(Operator::DIVISION)
        }

        Token::t_operator(lexer::token_type::Operator::and_op(_)) => return Some(Operator::NOT),
        Token::t_operator(lexer::token_type::Operator::or_op(_)) => return Some(Operator::OR),

        Token::t_operator(lexer::token_type::Operator::check_equal_op(_)) => {
            return Some(Operator::CHECKEQUAL)
        }
        Token::t_operator(lexer::token_type::Operator::not_equal_op(_)) => {
            return Some(Operator::CHECKNEQUAL)
        }
        Token::t_operator(lexer::token_type::Operator::greater_than_op(_)) => {
            return Some(Operator::GREATER)
        }
        Token::t_operator(lexer::token_type::Operator::lesser_than_op(_)) => {
            return Some(Operator::LESSER)
        }

        _ => return None,
    }
}

pub fn check_highest_precedence(invec: &mut Vec<Token>) -> usize {
    let mut precedence: usize = 0;

    for i in invec.iter() {
        if let Some(op) = get_operator_from_token(i.clone()) {
            if check_precedence(op.clone()) > precedence {
                precedence = check_precedence(op.clone());
            }
        }
    }

    return precedence;
}



// call this function to preprocess the determined expression as Vec<Expression>  before handling binary expression
pub fn expvec_preprocessor(expvec : Vec<Expression> , bt : &mut BinaryExpressionTree) -> Vec<Token> {

    let mut retval : Vec<Token> = Vec::new();

    return retval;
}
