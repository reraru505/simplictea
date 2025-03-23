use crate::expressions::Expression;
use crate::parsing_symbols::{is_arg_begin, is_arg_end, is_exp_end, is_scope_begin, is_scope_end};
use std::cell::RefCell;
use std::process;
use std::rc::Rc;

pub fn collect_binary_expression(vecref: Rc<RefCell<Vec<Expression>>>,index: &mut usize)
 -> Vec<Expression> {

    let mut retval: Vec<Expression> = Vec::new();

    loop {
        if is_exp_end(vecref.borrow()[*index].clone()) {
            *index += 1;
            break;
        }

        retval.push(vecref.borrow()[*index].clone());
        *index += 1;

    } //loop end

    return retval;
}

pub fn collect_body(vecref: Rc<RefCell<Vec<Expression>>>, index: &mut usize)
 -> Vec<Expression> {

    let mut retval: Vec<Expression> = Vec::new();

    let mut inner = 0;

    // this case should not occur but checking here for the last time

    if is_scope_begin(vecref.borrow()[*index].clone()) {
        *index += 1;
    } else {
        println!(" in function collect_body , found bad input ");
        process::exit(-1);
    }

    loop {
        if is_scope_begin(vecref.borrow()[*index].clone()) {
            retval.push(vecref.borrow()[*index].clone());

            inner += 1;
        }

        if is_scope_end(vecref.borrow()[*index].clone()) {
            retval.push(vecref.borrow()[*index].clone());

            if inner == 0 {
                *index += 1;
                break;
            } else {
                inner -= 1;
            }
        }

        retval.push(vecref.borrow()[*index].clone());
        *index += 1;

    } //loop end

    return retval;
}

pub fn collect_args(vecref: Rc<RefCell<Vec<Expression>>>, index: &mut usize)
 -> Vec<Expression> {

    let mut retval: Vec<Expression> = Vec::new();
    let mut inner = 0;

    // this case should not happen but checking here for the last time

    if is_arg_begin(vecref.borrow()[*index].clone()) {
        *index += 1;
    } else {
        println!(" in function collect_args , found bad input ");
        process::exit(-1);
    }


    
    loop {
        if is_arg_begin(vecref.borrow()[*index].clone()) {
            retval.push(vecref.borrow()[*index].clone());

            inner += 1;
        }

        if is_arg_end(vecref.borrow()[*index].clone()) {
            retval.push(vecref.borrow()[*index].clone());

            if inner == 0 {
                *index += 1;
                break;
            } else {
                inner -= 1;
            }
        }

        retval.push(vecref.borrow()[*index].clone());
        *index += 1;

    } //loop end

    return retval;
}
