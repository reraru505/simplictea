#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::panic;

use crate::parser::Parser;
use crate::ast::{AST_Expression, AST_Node, AST_Operator};
use crate::lexer::token_type::{Token, Literal, Operator, STC};
use crate::symboltable::symbol::Type;


impl Parser {

    pub fn Expression_Parser(&mut self) -> Option<AST_Node> {
        // Try parsing full expression first
        if let Some(expr) = self.logical_or_parser() {
            return Some(AST_Node::expression(expr));
        }
        
        // Fall back to single value if full expression fails
        if let Some(value) = self.value_parser() {
            return Some(AST_Node::expression(value));
        }

        // If nothing could be parsed, return None to allow other parsers to try
        None
    }

    pub fn Literal_Parser(&mut self) -> Option<AST_Expression> {
        let current_token = self.tokens[self.current].clone();

        if let Token::t_literal(literal) = current_token {
            match literal {
                Literal::string_literal(value) => {
                    if !self.consume() {return None;}
                    Some(AST_Expression::StringLiteral(value))
                },
                Literal::integer_literal(value) => {
                    if !self.consume() {return None;}
                    value.parse().ok().map(AST_Expression::IntegerLiteral)
                },
                Literal::decimal_literal(value) => {
                    if !self.consume() {return None;}
                    value.parse().ok().map(AST_Expression::DecimalLiteral)
                },
                Literal::character_literal(value) => {
                    if !self.consume() {return None;}
                    value.parse().ok().map(AST_Expression::CharacterLiteral)
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn Identifier_Parser(&mut self) -> Option<AST_Expression> {
        let current_token = self.tokens[self.current].clone();
        match current_token {
            Token::t_identifier(value) => {
                
                if self.symtable.lookup_variable(
                    &value.clone()
                ).is_none() {
                    panic!("The error handler is not implemented \
                    \n The identifer < {} > is not in the active scope" , value);
                }



                if !self.consume() { 
                    return None; 
                }

                Some(AST_Expression::Identifier(value))

            },
            _ => None,
        }
    }
    pub fn Identifier_Parser_old(&mut self) -> Option<AST_Expression> {
        let current_token = self.tokens[self.current].clone();
        match current_token {
            Token::t_identifier(value) => {
                // Remove the panic - leave semantic checking for later
                if !self.consume() { 
                    return None; 
                }
                Some(AST_Expression::Identifier(value))
            },
            _ => None,
        }
    }
    pub fn Declaration_Identifier_Parser(&mut self) -> Option<AST_Expression> {
        let current_token = self.tokens[self.current].clone();
        match current_token {
            Token::t_identifier(value) => {
                if !self.consume() { 
                    return None;
                }
                Some(AST_Expression::Identifier(value))
            },
            _ => None,
        }
    }

    pub fn lhs_parser(&mut self) -> Option<AST_Expression> {
        // Try to parse object items first
       // if let Some(obj_item) = self.object_item_parser() {
        //    return Some(obj_item);
        //}

        // Then try simple identifiers
        if let Some(iden) = self.Declaration_Identifier_Parser() {
            return Some(iden);
        }

        None
    }

    pub fn argument_parser(&mut self  ) -> Option<Vec<AST_Expression>> {

        let checkpoint = self.current;
        let mut retval : Vec<AST_Expression> = Vec::new();

        if self.is_param_end(){
            return None;
        }

        loop {
            let arg : AST_Expression;

            if let Some(AST_Node::expression(expr)) = self.Expression_Parser(){
                arg = expr;
            }else{
                return self.vomit_and_die(checkpoint); 
            }

            if self.is_comma_seperator(){
                retval.push(arg.clone());
            }
            
            if self.is_param_end(){
                retval.push(arg);

                return Some(retval);
            }

        } // loop end

    }

    pub fn Function_Call_Parser(&mut self) -> Option<AST_Expression> {

        //panic!("Function call parser calledl");

        let checkpoint = self.current;

        let name : AST_Expression;

        if let Some(identifer) = self.Identifier_Parser(){
            name = identifer;
        }else{
            return self.vomit_and_die(checkpoint);
        }

        if self.is_param_start(){
            return Some (AST_Expression::Call { 
                calee: Box::new(name) ,
                argumments: self.argument_parser(),
            });

        }else{
            return self.vomit_and_die(checkpoint);
        }
    }
 

    pub fn value_parser_old(&mut self) -> Option<AST_Expression> {
        self.Literal_Parser()
            .or_else(|| self.Function_Call_Parser()
                .or_else(|| self.Identifier_Parser()
                    .or_else(|| self.object_item_parser())))
    }


    pub fn value_parser(&mut self) -> Option<AST_Expression> {
        // Try object items first since they're common in return statements
        if let Some(obj) = self.object_item_parser() {
            return Some(obj);
        }

        // Then try other value types
        self.Literal_Parser()
            .or_else(|| self.Function_Call_Parser())
            .or_else(|| self.Identifier_Parser())
    }

    pub fn Primary_Parser(&mut self) -> Option<AST_Expression>{

        if let Some(left ) = self.value_parser(){
            return Some(left);
        }

        let checkpoint = self.current;

        if self.is_param_start(){
            if let Some(AST_Node::expression(expr)) = self.Expression_Parser(){
                if self.is_param_end(){
                    return Some(expr);

                }else{
                    return self.vomit_and_die(checkpoint);
                }

            }
        }

        None
    }

    fn is_next_token_dot(&self ) -> bool {

        if let Some(token) = self.peek(1){
            if matches!(token , Token::t_stc(STC::stc_dot(_))){
                return true;
            }else{
                return false;
            }
        } 

        return false;
    }
    pub fn object_item_parser(&mut self) -> Option<AST_Expression> {
        let checkpoint = self.current;

        // Must start with an identifier
        let first_ident = match self.Identifier_Parser() {
            Some(id) => id,
            None => return None,
        };

        let mut chain = vec![first_ident];

        // Parse dot-separated items
        while self.is_dot_operator() {
            if let Some(item) = self.Declaration_Identifier_Parser() {
                chain.push(item);
            } else {
                return self.vomit_and_die(checkpoint);
            }
        }

        // Need at least two components for a valid object item
        if chain.len() < 2 {
            return self.vomit_and_die(checkpoint);
        }

        Some(AST_Expression::OBJECT_ITEM { object_item: chain })
    }

    pub fn object_item_parser_old_2(&mut self) -> Option<AST_Expression> {
        let checkpoint = self.current;

        // Check if the next token is a dot
        if !self.is_next_token_dot() {
            return None;
        }

        let mut chain = Vec::new();

        // Parse the first identifier
        let first_ident = self.Identifier_Parser()?;
        chain.push(first_ident.clone());

        // Parse dot-separated items
        while self.is_dot_operator() {
            let item = self.Declaration_Identifier_Parser()?;
            chain.push(item.clone());
        }

        // We must have at least two identifiers (e.g., `a.b`)
        if chain.len() < 2 {
            return self.vomit_and_die(checkpoint);
        }

        Some(AST_Expression::OBJECT_ITEM { object_item: chain })
    }


    pub fn object_item_parser_old(&mut self) -> Option<AST_Expression>{

        let checkpoint = self.current;

        if ! self.is_next_token_dot(){
            return None;
        }

        println!("\n\nObject Item Parser called\n\n\n");

        let mut retval : Vec<AST_Expression> = Vec::new();

        // getting the first item if the next operator is dot
        let mut current_object : AST_Expression;
        if let Some(object) = self.Identifier_Parser(){

            current_object = object.clone();

        }else{
            return self.vomit_and_die(checkpoint);
        }

        loop {
            
            // getting the name of the current object
            let current_object_name : String;
            if let AST_Expression::Identifier(name) = current_object.clone() {
                current_object_name = name;
            }else{
                return self.vomit_and_die(checkpoint);
            }

            // checking if the dot operator is followed
            if !self.is_dot_operator(){
                return self.vomit_and_die(checkpoint);
            }

            let current_item_name : String;
            let current_item : AST_Expression;

            if let Some(item) = self.Declaration_Identifier_Parser(){
                current_item = item.clone();
                if let AST_Expression::Identifier(name) = item{
                    current_item_name = name;
                }else{
                    return self.vomit_and_die(checkpoint);
                }
            }else{
                return self.vomit_and_die(checkpoint);
            }

            // if the dot operator is present continue the loop , if not break
            if self.is_dot_operator_dont_consume(){
                if self.symtable.lookup_struct_field(&current_object_name , &current_item_name).is_some(){
                    retval.push(current_object);
                }
                current_object = current_item.clone();
            }else{
                if self.symtable.lookup_struct_field(&current_object_name, &current_item_name).is_some(){
                    retval.push(current_object);
                    retval.push(current_item);
                }
                break;
            }
        }

        println!("\n\nObject Item Parser Returned the value {:#?}\n\n" , retval);

        return Some(AST_Expression::OBJECT_ITEM { object_item: retval });
        
       
    }
    pub fn object_item_parser_(&mut self) -> Option<AST_Expression> {
        if !self.is_next_token_dot() { return None; }

        let checkpoint = self.current;
        let mut chain = Vec::new();

        // Parse initial identifier
        let first_ident = self.Identifier_Parser()?;
        chain.push(first_ident.clone());

        // Parse dot-separated items
        while self.is_dot_operator() {
            let item = self.Declaration_Identifier_Parser()?;
            chain.push(item.clone());
        }

        // We must have at least two identifiers (e.g., `a.b`)
        if chain.len() < 2 {
            return self.vomit_and_die(checkpoint);
        }

        // Validate the chain by traversing struct fields
        let mut current_struct_name = None;

        // Look up the first identifier as a variable to get its type
        if let AST_Expression::Identifier(first_id) = &chain[0] {
            if let Some(symbol) = self.symtable.lookup_variable(first_id) {
                if let Type::CUSTOM(name) = &symbol.data_type {
                    current_struct_name = Some(name.clone());
                } else {
                    // First identifier is not a struct
                    return self.vomit_and_die(checkpoint);
                }
            } else {
                // First identifier not found
                return self.vomit_and_die(checkpoint);
            }
        } else {
            return self.vomit_and_die(checkpoint);
        }

        // Traverse the rest of the fields
        for i in 1..chain.len() {
            if let AST_Expression::Identifier(field) = &chain[i] {
                if let Some(ref struct_name) = current_struct_name {
                    if let Some(field_symbol) = self.symtable
                        .lookup_struct_field_from_struct_name(struct_name, field) 
                    {
                        // Update current struct name for next field if it's a struct
                        match &field_symbol.data_type {
                            Type::CUSTOM(next_struct) => {
                                current_struct_name = Some(next_struct.clone());
                            }
                            _ => {
                                // This field is not a struct
                                current_struct_name = None;
                            }
                        }
                    } else {
                        // Field not found in the struct
                        return self.vomit_and_die(checkpoint);
                    }
                } else {
                    // Previous part was not a struct, so we can't have a field
                    return self.vomit_and_die(checkpoint);
                }
            } else {
                return self.vomit_and_die(checkpoint);
            }
        }

        Some(AST_Expression::OBJECT_ITEM { object_item: chain })
    }

    pub fn logical_or_parser(&mut self) -> Option<AST_Expression>{

        let mut left = self.logical_and_parser()?;

        loop {
            match self.current_token(){

                Token::t_operator(Operator::or_op(_)) => {
                    if !self.consume() { break;}
                    let right = self.logical_and_parser()?;
                    left = AST_Expression::BinaryExpression { 
                        operator: AST_Operator::OR,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                },

                _ => break,
            }
        }

        Some(left)
    }


    pub fn logical_and_parser(&mut self) -> Option<AST_Expression>{

        let mut left = self.equality_parser()?;

        loop {
            match self.current_token(){

                Token::t_operator(Operator::and_op(_)) => {
                    if !self.consume() { break;}
                    let right = self.equality_parser()?;
                    left = AST_Expression::BinaryExpression { 
                        operator: AST_Operator::AND,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                },

                _ => break,
            }
        }

        Some(left)
    }
  

    pub fn equality_parser(&mut self) -> Option<AST_Expression>{
        
        let mut left = self.relational_parser()?;

        loop {
            match self.current_token(){

                Token::t_operator(Operator::check_equal_op(_)) => {
                    if !self.consume() { break;}
                    let right = self.relational_parser()?;
                    left = AST_Expression::BinaryExpression { 
                        operator: AST_Operator::CHECK_EQUAL,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                },

                Token::t_operator(Operator::not_equal_op(_)) => {
                    if !self.consume() { break;}
                    let right = self.relational_parser()?;
                    left = AST_Expression::BinaryExpression { 
                        operator: AST_Operator::CHECK_NOT_EQUAL,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                },
                  
                _ => break,
            }
        }

        Some(left)
    }
     
    pub fn relational_parser(&mut self) -> Option<AST_Expression>{
        let mut left = self.additive_parser()?;

        loop {
            match self.current_token(){

                Token::t_operator(Operator::lesser_than_op(_)) => {
                    if !self.consume() { break;}
                    let right = self.additive_parser()?;
                    left = AST_Expression::BinaryExpression { 
                        operator: AST_Operator::CHECK_LESSER,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                },

                Token::t_operator(Operator::greater_than_op(_)) => {
                    if !self.consume() { break;}
                    let right = self.additive_parser()?;
                    left = AST_Expression::BinaryExpression { 
                        operator: AST_Operator::CHECK_GREATER,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                },
                  
                _ => break,
            }
        }

        Some(left)
    }

    pub fn additive_parser(&mut self) -> Option<AST_Expression> {
        let mut left = self.multiplicative_parser()?;

        println!("Additive parser started");


        loop {
            match self.current_token() {
                Token::t_operator(Operator::addition_op(_)) => {
                    println!("Found addition op");
                    if !self.consume() { break;}
                    let right = self.multiplicative_parser()?;
                    left = AST_Expression::BinaryExpression {
                        operator: AST_Operator::ADD,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::t_operator(Operator::subtraction_op(_)) => {
                    println!("Found subtraction op");
                    if !self.consume() { break;}
                    let right = self.multiplicative_parser()?;
                    left = AST_Expression::BinaryExpression {
                        operator: AST_Operator::SUB,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(left)
    }

    pub fn multiplicative_parser(&mut self) -> Option<AST_Expression> {
        let mut left = match self.Primary_Parser() {
            Some(val) => {
                println!("Literal found");
                val
            }
            None => return None,
        };

        loop {
            match self.current_token() {
                Token::t_operator(Operator::multiplication_op(_)) => {
                    if !self.consume() { break;}
                    let right = match self.value_parser() {
                        Some(val) => {
                            val
                        }
                        None => return None,
                    };
                    left = AST_Expression::BinaryExpression {
                        operator: AST_Operator::MUL,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::t_operator(Operator::division_op(_)) => {
                    if !self.consume() { break;}
                    let right = match self.value_parser() {
                        Some(val) => {
                            val
                        }
                        None => return None,
                    };
                    left = AST_Expression::BinaryExpression {
                        operator: AST_Operator::DIV,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }                          
        Some(left)
    }
}
