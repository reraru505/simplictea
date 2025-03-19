use crate::{
    expressions::Expression, function_grammer::is_function_def, lexer::lex::Lexeme, scope::Block,
};

pub fn parser(inp_lexemes: Vec<Lexeme>) -> Vec<Expression> {}

fn parse_recurse() {}

fn once(expvec: Vec<Expression>) -> Vec<Expression> {
    let mut context: Vec<Expression> = Vec::new();
    let mut retvec: Vec<Expression> = Vec::new();

    let mut index: usize = 0;
    let veclen = expvec.len();

    while index < veclen {
        check_in_once(context.clone(), expvec[index].clone());

        index += 1;
    }

    return retvec;
}

fn check_in_once(context: Vec<Expression>, lookahed: Expression , ) -> Option<usize> {
    let retval = is_function_def(context, lookahed);
    if let Some(fdef) = retval {}
}



fn extract_body_as_scope(
    expvec: Vec<Expression>,
    scope: String,
    index: &mut usize,
) -> Option<Block> {

    
    let mut retval = Block::new(scope);
    let mut retvec : Vec<Expression> = Vec::new();

    if matches!(
        expvec[*index],
        Expression::token(lexer::token_type::Token::t_stc(
            lexer::token_type::STC::stc_scope_begin(_)
        ))
    ) {
        *index += 1;
    } else {
        return None;
    }



    //this loop extracts a body of any scoped expression 

    let mut inner = 0;

    loop {
        if matches!(
            expvec[*index],
            Expression::token(lexer::token_type::Token::t_stc(
                lexer::token_type::STC::stc_scope_begin(_)
            ))
        ) {

            retvec.push(expvec[*index].clone());
            inner += 1;

        } else if matches!(
            expvec[*index],
            Expression::token(lexer::token_type::Token::t_stc(lexer::token_type::STC::stc_scope_end(_)))) {

            if inner == 0 {
                break;

            } else {

                retvec.push(expvec[*index].clone());
                inner -= 1;

            }

        }else {

            retvec.push(expvec[*index].clone());
            *index += 1;
                        
        }
    }





    retval.body = retvec;

    return Some(retval);
}
