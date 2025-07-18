#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use qbe::*;
use std::collections::HashMap;

use crate::helpers::get_name_of_identifier;
use crate::parser::ast::AST_Node;
use crate::parser::ast::AST_Statement;
use crate::parser::ast::AST_TypeAnnotation;


pub struct Custom_Types_Handler <'a> {

    pub custom_types : Vec<TypeDef <'a>> ,
    pub counter : usize ,
    pub type_lookup : HashMap<String , usize>,
    
}

pub struct Globals_Handler{
    pub global_data : Vec<DataItem> ,
    pub counter : usize ,
    pub data_lookup : HashMap<String , usize>,
}

pub struct Functions_Handler <'a>{
    pub functions : Vec<Function <'a>>,
    pub function_lookup : HashMap<String , usize >,
}


impl <'a> Custom_Types_Handler <'a>{
    
    pub fn new() -> Self {
        Self{
            custom_types : Vec::new(),
            counter : 0,
            type_lookup : HashMap::new(),
        }
    }

    pub fn find_all_types_in_ast(&mut self , ast : Vec<AST_Node> ) {
        
        for i in ast.iter(){
            if let AST_Node::statement(AST_Statement::STRUCTURE { name, body }) = i.clone(){
               self.type_lookup.insert(get_name_of_identifier(name), self.counter);
               self.counter += 1;
               
               self.custom_types.push(
                   TypeDef { name: get_name_of_identifier(name.clone()), align: None, items: self.unwrap_structure_body(body) }
               );
            }
        }
    }


    pub fn unwrap_structure_body(&self ,body : Vec<AST_Node>) -> Vec<(Type  , usize)> {

        let mut retval : Vec<(Type , usize)> = Vec::new();

        for i in body {

            if let AST_Node::statement(AST_Statement::LET { name, type_annotation, initializer }) = i{
                retval.push(self.get_types_from_AST(type_annotation));
            }    
        }

        return retval;
    }

    pub fn display_types(&self ) {
        
        let mut module = Module::new();
    
        for i in self.custom_types.iter() {
            module.add_type(i.clone());
        }

        println!("{}" , module);
    }


    pub fn get_custom_type_from_AST (&self , type_ : AST_TypeAnnotation ) -> (Type , usize){

        if let AST_TypeAnnotation::CUSTOM(name ) = type_ {

            match self.type_lookup.get(&name) {

                Some(v) => {
                    return (
                        Type::Aggregate(& self.custom_types[*v]) ,
                        1
                    )
                },
                None => {
                    panic!(" The custom type is not in the type cache and is possibly not defined");
                }
            }
        }
        panic!(" The provide type annoation was not a custom one");
    }
    pub fn get_types_from_AST (&self, type_ : AST_TypeAnnotation ) -> (Type , usize) {

        match type_ {

            AST_TypeAnnotation::I32 => return (Type::Word , 1),      
            AST_TypeAnnotation::U32 => return (Type::Word , 1),      
            AST_TypeAnnotation::I64 => return (Type::Long , 1),     
            AST_TypeAnnotation::U64 => return (Type::Long , 1),     

            AST_TypeAnnotation::F32 => return (Type::Single , 1),   
            AST_TypeAnnotation::F64 => return (Type::Double , 1),   

            AST_TypeAnnotation::STRING => return (Type::Word , 1),  
            AST_TypeAnnotation::VOID => return   (Type::Zero , 1),  
            AST_TypeAnnotation::CHAR => return   (Type::Byte , 1),  
            AST_TypeAnnotation::CUSTOM(val) => {
                match  self.type_lookup.get(&val){

                    Some(v) => {
                        return (
                            Type::Aggregate(& self.custom_types[*v]),
                            1
                        )
                    },
                    _ => panic!("The type is nor basic nor custom "),

                }
            }
        }
    }


}

