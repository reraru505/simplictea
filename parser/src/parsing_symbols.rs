#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::lexer::token_type::{

    Token ,

    Keyword ,
    Statement ,
    Literal ,
     
    STC ,
    
};

use crate::expressions::Expression;

pub fn is_scope_begin(tok : Expression ) -> bool {

    if matches!(tok , Expression::token(Token::t_stc(STC::stc_scope_begin(_))) ){
        return true;
    }
    return false;
}

pub fn is_scope_end(tok : Expression ) -> bool {
    if matches!(tok , Expression::token(Token::t_stc(STC::stc_scope_end(_)))){
        return true;
    }
    return false;
}

pub fn is_arg_begin(tok : Expression ) -> bool {
    if matches!(tok , Expression::token(Token::t_stc(STC::stc_arg_begin(_)))){
        return true;
    }
    return false;
}

pub fn is_arg_end(tok : Expression ) -> bool {
    if matches!(tok ,Expression::token( Token::t_stc(STC::stc_arg_end(_)))){
        return true;
    }
    return false;
}

pub fn is_exp_end(tok : Expression ) -> bool {
    if matches!(tok ,Expression::token( Token::t_stc(STC::stc_end_expression(_)))){
        return true;
    }

    return false;
}
