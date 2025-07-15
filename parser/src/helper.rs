#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use lexer::identifier;

use crate::ast::AST_Expression;
use crate::parser::Parser;
use crate::ast::AST_Node;
use crate::ast::AST_Statement;
use crate::ast::AST_TypeAnnotation;
use crate::ast::AST_ConditionalType;
use crate::symboltable::symbol::PrimaryType;
use crate::lexer::token_type::Token;
use crate::lexer::token_type::Data_Type;
use crate::lexer::token_type::Keyword;
use crate::lexer::token_type::Operator;
use crate::lexer::token_type::Statement;
use crate::symboltable::symbol::Type;


impl Parser {

    pub fn conditional_type_finder(&mut self) -> Option<AST_ConditionalType>{
        let current_token = self.current_token();

        if let Token::t_keyword(Keyword::statement(stat)) = current_token{
            match stat {

                Statement::if_statement(_) => {
                    if !self.consume() { 
                        return None;
                    }
                    return Some(AST_ConditionalType::IF); 
                },

                Statement::else_if_statement(_) => {
                    if !self.consume() { 
                        return None;
                    }
                    return Some(AST_ConditionalType::ELIF);
                },

                Statement::else_statement(_) => {
                    if !self.consume() { 
                        return None;
                    }

                    return Some(AST_ConditionalType::ELSE);
                }, 

                _ => return None,
            }
        }else{
            return None;
        }

    }

    pub fn extract_identifier_name(&self , id : AST_Expression) -> Option<String> {

        if let AST_Expression::Identifier(name ) = id {
            return Some(name.clone());
        }

        return None;
        
    }

    pub fn annotation_to_symbol_primary(& self , annot : AST_TypeAnnotation) -> Type {

        match annot {
            AST_TypeAnnotation::I32     =>     Type::PRIMARY(PrimaryType::I32) ,
            AST_TypeAnnotation::I64     =>     Type::PRIMARY(PrimaryType::I64),  
            AST_TypeAnnotation::U32     =>     Type::PRIMARY(PrimaryType::U32 ), 
            AST_TypeAnnotation::U64     =>     Type::PRIMARY(PrimaryType::U64 ), 
            AST_TypeAnnotation::F32     =>     Type::PRIMARY(PrimaryType::F32 ), 
            AST_TypeAnnotation::F64     =>     Type::PRIMARY(PrimaryType::F64 ), 
            AST_TypeAnnotation::CHAR    =>     Type::PRIMARY(PrimaryType::CHAR), 
            AST_TypeAnnotation::VOID    =>     Type::PRIMARY(PrimaryType::VOID), 
            AST_TypeAnnotation::STRING  =>     Type::PRIMARY(PrimaryType::STRING), 
            AST_TypeAnnotation::CUSTOM(val) => Type::CUSTOM(val),
            
        }

    }

    pub fn is_custom_type(&self , name : String ) -> bool {
        
        if self.symtable.lookup_struct(&name).is_some() {
            return true;
        }
        println!("Symbol lookup failed");
        return false;
    }
 

    pub fn Type_Annotation_Parser(&mut self) -> Option<AST_TypeAnnotation> {

        let current_token = self.current_token();


        if let Token::t_keyword(Keyword::data_type( data_type )) = current_token {
            match data_type {

                Data_Type::I32(_) => {
                    if !self.consume() { return None;}
                    return Some(
                        AST_TypeAnnotation::I32
                    ); 
                },

                Data_Type::I64(_) => {
                    if !self.consume() { return None;}
                    return Some(
                        AST_TypeAnnotation::I64
                    );
                },
                Data_Type::F32(_) => {
                    if !self.consume() { return None;}
                    return Some(
                        AST_TypeAnnotation::F32
                    );
                } ,
                Data_Type::F64(_) => {
                    if !self.consume() { return None;}
                    return Some(
                        AST_TypeAnnotation::F64
                    );
                } ,
                Data_Type::CHAR(_) => {
                    if !self.consume() { return None;}
                    return Some(
                        AST_TypeAnnotation::CHAR
                    );                      
                },
                Data_Type::STRING(_) => {
                    if !self.consume() { return None;}
                    return Some(
                        AST_TypeAnnotation::STRING
                    )
                } ,
                Data_Type::VOID(_) => {
                    if !self.consume() { return None;}
                    return Some(
                        AST_TypeAnnotation::VOID
                    )
                } ,

                _ => panic!("This case can never happen : \
                    please check code for fu Type_Annotation_Parser "),


            }
        }else if let Token::t_identifier(identifier) = current_token {

            println!("Type annotation parser trying to check identifier");

            if self.is_custom_type(identifier.clone()) {
                if !self.consume() { return None}
                return Some(
                    AST_TypeAnnotation::CUSTOM(identifier)
                );
            }else{
                None
            }
            

        }else{
            None
        }



    }



    pub fn is_expression_end_marker(&mut self) -> bool {
        let current_token = self.current_token();

        if matches!(current_token ,
            Token::t_stc(lexer::token_type::STC::stc_end_expression(_))){
            if !self.consume() {
                return false;
            }
            return true;
        }

        return false;

    }

    pub fn is_type_assignment_marker(&mut self ) -> bool {
        let current_token = self.current_token();

        if matches!(current_token , 
            Token::t_operator(Operator::type_assignment_op(_))){
            if !self.consume() { 
                return false;
            }
            return true;
        }

        return false; 
    }

    pub fn is_assignment_marker(&mut self) -> bool {
        let current_token = self.tokens[self.current].clone();

        if matches!(current_token , 
            Token::t_operator(Operator::assignment_op(_))){
            if !self.consume() { 
                return false;
            }
            return true;
        }

        return false; 

    }

    pub fn is_function_marker(&mut self ) -> bool {
        let current_token = self.current_token();

        if matches!(current_token ,
            Token::t_keyword(Keyword::statement(
                    lexer::token_type::Statement::function_marker(_)
                    ))){
            if !self.consume(){
                return false;
            }

            return true;
        }

        return false;
    }


    pub fn is_body_start(&mut self) -> bool {
        let current_token = self.current_token();

        if matches!(current_token , 
            Token::t_stc(lexer::token_type::STC::stc_scope_begin(_))){
            if !self.consume(){
                return false;
            }

            return true;
        }

        return false;
    }

    pub fn is_body_end(&mut self) -> bool {
        let current_token = self.current_token();

        if matches!(current_token , 
            Token::t_stc(lexer::token_type::STC::stc_scope_end(_))){
            if !self.consume() { 
                return false;
            }

            return true;
        }

        return false;
    }

    pub fn is_param_start(&mut self ) -> bool {
        let current_token = self.current_token();

        if matches!(current_token ,
            Token::t_stc(lexer::token_type::STC::stc_arg_begin(_))){
            if !self.consume() {
                return false;
            }

            return true;
        }

        return false;
    }

    
    pub fn is_param_end(&mut self ) -> bool {
        let current_token = self.current_token();

        if matches!(current_token ,
            Token::t_stc(lexer::token_type::STC::stc_arg_end(_))){
            if !self.consume() { 
                return false;
            }
            return true;
        }

        return false;
    }

    pub fn is_comma_seperator(&mut self) -> bool {
        let current_token = self.current_token();

        if matches!(current_token,
            Token::t_stc(lexer::token_type::STC::stc_comma_seperator(_))){
            if !self.consume() { 
                return false;
            }
            return true;
        }
        return false;
    }

    pub fn is_dot_operator(&mut self) -> bool {
        let current_token = self.current_token();

        if matches!(current_token,
            Token::t_stc(lexer::token_type::STC::stc_dot(_))){
            if !self.consume() { 
                return false;
            }
            return true;
        }
        return false;
    }
       
    pub fn is_dot_operator_dont_consume(&mut self) -> bool {
        let current_token = self.current_token();

        if matches!(current_token,
            Token::t_stc(lexer::token_type::STC::stc_dot(_))){
            return true;
        }
        return false;
    }


    
}
