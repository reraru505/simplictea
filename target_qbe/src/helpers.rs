use parser::ast::*;
use crate::sections::Custom_Types_Handler;
use qbe::*;
use crate::parser::ast::AST_TypeAnnotation;
use qbe::Type;

use std::rc::Rc;
use std::cell::RefCell;

pub fn find_all_types_in_ast(custom_type_handler: Rc<RefCell<Custom_Types_Handler>>, ast: Vec<AST_Node>) {
    let handler = custom_type_handler;
    
    for node in ast {
        if let AST_Node::statement(AST_Statement::STRUCTURE { name, body }) = node {
            // Operation 1
            {
                let mut h = handler.borrow_mut();
                h.type_lookup.insert(get_name_of_identifier(name.clone()), h.counter);
                h.counter += 1;
            }
            
            // Operation 2
            let items = {
                let mut h = handler.borrow_mut();
                h.unwrap_structure_body(body)
            };
            
            // Operation 3
            {
                let mut h = handler.borrow_mut();
                h.custom_types.push(
                    TypeDef { 
                        name: get_name_of_identifier(name.clone()), 
                        align: None,
                        items 
                    }
                );
            }
        }
    }
}


pub fn get_name_of_identifier(name : AST_Expression) -> String {
    if let AST_Expression::Identifier(iname) = name {
        return iname;
    }

    String::new()
}


