use crate::function::FunctionDef;
use crate::lexer::token_type::{Data_Type, Keyword, Operator, Statement, Token, STC};
use crate::{
    expressions::Expression,
    function::{FunctionCall, FunctionDefType, FunctionRet},
    symboltable::symbol::{get_data_type_from_token, DataType},
};

pub fn is_function_def(expvec: &mut Vec<Expression>, index: &mut usize) -> Option<FunctionDef> {
    let last_index = *index;

    let mut retval: bool = false;

    let mut id: String = String::new();
    let scope: String = String::new();
    let mut data_type: DataType = DataType::VOID;

    if let Expression::token(tok) = expvec[*index].clone() {
        if matches!(tok, Token::t_identifier(_)) {
            if let Token::t_identifier(val) = tok.clone() {
                id = val;
            }
            *index += 1;
            retval = true;
        } else {
            *index = last_index;
            return None;
        }
    }

    if let Expression::token(tok) = expvec[*index].clone() {
        if matches!(tok, Token::t_operator(Operator::type_assignment_op(_))) {
            *index += 1;
            retval = true;
        } else {
            *index = last_index;
            return None;
        }
    }

    if let Expression::token(tok) = expvec[*index].clone() {
        if matches!(tok, Token::t_keyword(Keyword::data_type(_))) {
            *index += 1;
            data_type = get_data_type_from_token(tok.clone());
            retval = true;
        } else {
            *index = last_index;
            return None;
        }
    }

    if let Expression::token(tok) = expvec[*index].clone() {
        if matches!(
            tok,
            Token::t_keyword(Keyword::statement(Statement::function_marker(_)))
        ) {
            *index += 1;
            retval = true;
        } else {
            *index = last_index;
            return None;
        }
    }

    if retval == true {
        if let Expression::token(tok) = expvec[*index].clone() {
            if matches!(tok, Token::t_stc(STC::stc_scope_begin(_))) {
                return Some(FunctionDef {
                    fn_id: id,
                    super_scope: scope,
                    fn_type: FunctionDefType::DEF_NO_ARGS,
                    fn_return_type: data_type,
                    fn_body: None,
                });
            } else if matches!(tok, Token::t_stc(STC::stc_arg_begin(_))) {
                return Some(FunctionDef {
                    fn_id: id,
                    super_scope: scope,
                    fn_type: FunctionDefType::DEF_WITH_ARGS,
                    fn_return_type: data_type,
                    fn_body: None,
                });
            }
        }
    }
    return None;
}

pub fn is_function_return(expvec: &mut Vec<Expression>, index: &mut usize) -> Option<FunctionRet> {
    let last_index = *index;
    let mut is_return = false;

    if let Expression::token(tok) = expvec[*index].clone() {
        if matches!(
            tok,
            Token::t_keyword(Keyword::statement(Statement::return_statement(_)))
        ) {
            is_return = true;
        }
    }

    //@ note to myself @ validity of the binary expression can be checked here

    if is_return == true {
        *index += 1;

        let mut buffer: Vec<Token> = Vec::new();

        loop {
            if let Expression::token(tok) = expvec[*index].clone() {
                if matches!(tok, Token::t_stc(STC::stc_end_expression(_))) {
                    break;
                } else {
                    buffer.push(tok);
                }
            }
        }
    }

    *index = 
    return None;
}

pub fn is_function_call(expvec: &mut Vec<Expression>, index: &mut usize) -> Option<FunctionCall> {
    if let Expression::token(tok) = expvec[*index].clone() {
        if matches!(
            tok,
            Token::t_keyword(Keyword::statement(Statement::return_statement(_)))
        ) {}
    }

    return None;
}
