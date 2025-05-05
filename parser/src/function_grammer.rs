#![allow(non_camel_case_types)]
#![allow(unused_imports)]


use crate::{
    collector::{collect_args, collect_binary_expression, collect_body},
    expressions::Expression,
    function::{FunctionCall, FunctionDef, FunctionDefType, FunctionRet},
    lexer::token_type::{Data_Type, Keyword, Operator, Statement, Token, STC},
    scope::Block,
    symboltable::symbol::{get_data_type_from_token, DataType}
};

use std::rc::Rc;
use std::cell::RefCell;

pub fn get_name_from_identifier(exp : Expression)
-> String {

    if let Expression::token(tok ) = exp {
        if let Token::t_identifier(retval ) = tok {
            return retval;
        }else {

            println!("The token passed was not an identifier");
            return String::from("");
            
        }
    }

    println!("The Expression passed was not a token  ");
    return String::from("");

}

fn get_datatype_from_expression(exp : Expression )
-> DataType {
    if let Expression::token(tok) = exp {
        return get_data_type_from_token(tok);
    }else {

        println!("The Expression passed was not a token");
        return DataType::VOID;
    }
}

pub fn is_function_def (index : &mut usize  , expvec : Rc<RefCell< Vec<Expression> >> , scope : String )
-> Option< FunctionDef> {

    let last_index = *index;

    let mut  retval =  FunctionDef::new();
    retval.super_scope = scope.clone();

    if matches!(expvec.borrow()[*index].clone(), Expression::token(
        Token::t_identifier(_)
    )){
        retval.fn_id = get_name_from_identifier(expvec.borrow()[*index].clone());
        *index += 1;
    }else{
        *index = last_index;
        return None;
    }
    
    if matches!(expvec.borrow()[*index].clone(), Expression::token(
        Token::t_operator(Operator::type_assignment_op(_))
    )){
        *index += 1;
    }else{
        *index = last_index;
        return None;
    }


    if matches!(expvec.borrow()[*index].clone(), Expression::token(
        Token::t_keyword(Keyword::data_type(_))
    )){
        retval.fn_return_type = get_datatype_from_expression(expvec.borrow()[*index].clone());
        *index += 1;
    }else{
        *index = last_index;
        return None;
    }


    if matches!(expvec.borrow()[*index].clone(), Expression::token(
        Token::t_keyword(Keyword::statement(Statement::function_marker(_)))
    )){
        *index += 1;
    }else{
        *index = last_index;
        return None;
    }
    

    if matches!(expvec.borrow()[*index].clone(), Expression::token(
        Token::t_stc(STC::stc_scope_begin(_)) | Token::t_stc(STC::stc_arg_begin(_))
    )){

        if matches!(expvec.borrow()[*index].clone(), Expression::token(
           Token::t_stc(STC::stc_arg_begin(_))
        )){
            retval.fn_args = Some(collect_args(Rc::clone(&expvec), index));
            let mut body = Block::new(scope.clone());
            body.body = collect_body(Rc::clone(&expvec), index);
            retval.fn_body = Some(body); 

            retval.fn_type = FunctionDefType::DEF_WITH_ARGS;    
        }


        if matches!(expvec.borrow()[*index].clone(), Expression::token(
            Token::t_stc(STC::stc_scope_begin(_))
        )){
            let mut body = Block::new(scope.clone());
            body.body = collect_body(Rc::clone(&expvec), index);
            retval.fn_body = Some(body);
             
            retval.fn_type = FunctionDefType::DEF_NO_ARGS;
        }
      
       
    }else{
        *index = last_index;
        return None;
    }
    
    return Some(retval);
}


pub fn is_function_call(index : &mut usize  , expvec : Rc<RefCell< Vec<Expression> >> , scope : String )
-> Option<FunctionCall> {

    let last_index = *index;
    let mut retval = FunctionCall::new();
    retval.super_scope = scope.clone();

    if matches!(expvec.borrow()[*index].clone() , Expression::token(
        Token::t_identifier(_)
    )){
        retval.fn_id = get_name_from_identifier(expvec.borrow()[*index].clone() );
        *index += 1;
    }else{
        *index = last_index;
        return None;
    }

    if matches!(expvec.borrow()[*index].clone() , Expression::token(
        Token::t_stc(STC::stc_arg_begin(_))
    )){
        retval.fn_args = Some(collect_args(Rc::clone(&expvec), index));
        *index +=  1;
        
    }else{

        *index = last_index;
        return None;
    }

    
    

    return None;
}

pub fn is_function_ret(index : &mut usize , expvec : Rc<RefCell< Vec<Expression> >> , scope : String )
-> Option<FunctionRet>{

    let last_index = *index;
    let mut retval = FunctionRet::new();

    if matches!(expvec.borrow()[*index].clone() , Expression::token(
        Token::t_keyword(Keyword::statement(Statement::return_statement(_)))
    )){

        *index += 1;

        retval.super_scope = scope.clone();
        retval.fn_ret = Some(collect_args(Rc::clone(&expvec), index));        
        return Some(retval);

    }

    *index = last_index;
    return None;
}
