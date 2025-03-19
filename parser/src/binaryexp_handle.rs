use std::rc::Rc;
use std::cell::RefCell;

use crate::binaryexp::{

    BinaryExpressionTree ,
    BinaryExpression,

};

use crate::lexer::{

    token_type::Token,
    token_type::STC ,
}

use crate::binaryexp_helpers::{

    check_highest_precedence,
    get_operator_from_token ,
    expvec_preprocessor ,
}

pub fn make_binary_expression_tree (tokvec : Vec<Token> , scope : String , tmp_count : &mut usize )
 -> BinaryExpressionTree {

    let mut precedence = check_highest_precedence(&mut tokvec );
    
        
}

pub fn break_binary_expression(precedence : &mut usize , tokvec : Vec<Token> , scope : String , tmp_count : &mut usize ) -> {

    if precedence == 7 {

        
    }
}


//helper function to find the bracket
fn find_bracket_from_index (refvec : Rc<RefCell< Vec<Token> >>  , ex_index : &mut usize ) -> Option<(usize , usize)> {

    let mut retval : (usize , usize ) = (0 , 0);


    let mut inner = 0;

    for (index , i ) in refvec.borrow().skip(*ex_index ).iter().enumerate() {

        if matches!(i , Token::t_stc(STC::stc_arg_begin(_))){
            if inner == 0 {
                retval = (index , 0);
                inner += 1;                
            }else {
                inner += 1;
            }
            
        }else if matches!(i , Token::t_stc(STC::stc_arg_end(_))){
            if inner == 0 {
                retval = (retval.0 , index + 1 );
                *ex_index = index + 1;
                break;
            }else {
                inner -= 0;
            }
        }
        
    }


    return None ;
}

pub fn create_sub_expression(refvec : Rc<RefCell< Vec<Token> >> , start : usize , end : usize ) -> Vec<Token> {

    refvec.borrow()[start .. end ].to_vec()
}
