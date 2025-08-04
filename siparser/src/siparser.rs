use crate::ast::{Node , Data_Type};
use crate::silexer::silex::{Token , Keyword , Operator , Special};
use std::collections::HashMap;



pub struct SiParser{
    pub tokens : Vec<Token>,
    pub index : usize,
}

impl SiParser {

    pub fn new(intokens : Vec<Token>) -> Self {
        Self { tokens: intokens, index: 0  }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        
        let mut retvec : Vec<Node> = Vec::new();

        loop {
            if self.is_eof(){
                break;
            }else if let Some(val) = self.function_declaration_parser(){
                retvec.push(val);
            }else if let Some(val) = self.structure_declaration_parser(){
                retvec.push(val);
            }else if let Some(val) = self.variable_declaration_parser(){
                retvec.push(val);
            }else if let Some(_) = self.assignment_parser(){
                panic!("Assignment cannot happen in global scope");
            }else if let Some(_) = self.iterator_parser(){
                panic!("Iterator cannnot be global");
            }else if let Some(_) = self.conditional_chain_parser(){
                panic!("Conditional cannot be global");
            }else if let Some(_) = self.return_statement_parser(){
                panic!("Cannot return from global scope");
            }else if let Some(_) = self.break_statement_parser(){
                panic!("WTF do you want me to break, your bones ?");
            }else if let Some(_) = self.function_call_parser(){
                panic!("The called function will not be called , this is not the entry point");
            }else{
                self.consume();
                println!("Consuming unconditionally , nothing found to push");
            }
        }

        return retvec;
    }
    
    pub fn function_declaration_parser(&mut self) -> Option<Node> {

        let checkpoint = self.index;

        let name = if let Some(n) = self.identifer_name_parser(){
            n
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if !self.is_colon(){
            return self.vomit_and_die(checkpoint);
        }

        let d_type = if let Some(d) = self.data_type_parser(){
            d
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if matches!(self.current() , Token::KEYWORD(Keyword::FN)){
            self.consume();
        }else{
            return self.vomit_and_die(checkpoint);
        }

        let mut params : Vec<Node> = Vec::new();

        if self.is_param_start(){
             params = self.function_param_parser();
        };

        let body : Vec<Node> ;

        if self.is_scope_start(){
            body = self.function_body_parser();
        }else{
            panic!("Function declaration does not have a body");
        }


        return Some(
            Node::FUNCTION_DEC { name: name, r_type: d_type , params: params , body: body }
        );
    }

    pub fn structure_declaration_parser(&mut self) -> Option<Node> {
        let checkpoint = self.index;

        if matches!(self.current() , Token::KEYWORD(Keyword::STRUCT)){
           self.consume();
        }else{
            return self.vomit_and_die(checkpoint);
        }

        let name = if let Some(n) = self.identifer_name_parser(){
            n
        }else{
            return self.vomit_and_die(checkpoint);
        };

        let body = if self.is_scope_start(){
            self.struct_body_parser()
        }else{
            panic!("There is no body for the struct defined");
        }; 

        return Some(
            Node::STRUCTURE { name: name , body: body }
        );
    }

    pub fn variable_declaration_parser(&mut self) -> Option<Node> {
        let checkpoint = self.index;

        let name = if let Some(nam) = self.identifer_name_parser(){
            nam
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if !self.is_colon(){
            return self.vomit_and_die(checkpoint);
        }

        let d_type = if let Some(d) = self.data_type_parser(){
            d
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if self.is_semicolon(){
            return Some(
                Node::VAR_DEC { name: name, d_type: d_type, initializer: None }
            );
        }else if self.is_assignmet_operator(){
            let initializer = if let Some(exp) = self.binary_expression_parser(){
                exp
            }else{
                panic!("The initializer is not an value or an expression");
            };

            if self.is_semicolon(){
                return Some(
                    Node::VAR_DEC { name: name, d_type: d_type , initializer: Some(Box::new(initializer)) }
                );
            }else{
                panic!("binary expression does not end the current token is {:#?}", self.current());
            }
        }else{
            // lets keep it here but might need to panic 
            return self.vomit_and_die(checkpoint);
        } 

    }

    pub fn conditional_chain_parser(&mut self) -> Option<Node>{
        let checkpoint = self.index;

        let mut retvec : Vec<Node> = Vec::new();
        if self.is_if_statement(){
            let exp = if let Some(exp) = self.binary_expression_parser(){
                exp
            }else{
                panic!("Conditional if without a condition");
            };

            let body = if self.is_scope_start(){
                self.conditional_body_parser()
            }else{
                panic!("Conditional if without body");
            };
            retvec.push(Node::CONDITIONAL { condition: Some(Box::new(exp)), body: body });
            loop {
     if self.is_elif_statement(){
                    let exp = if let Some(exp) = self.binary_expression_parser(){
                        exp
                    }else{
                        panic!("Conditional elif without a condition");
                    };

                    let body = if self.is_scope_start(){
                        self.conditional_body_parser()
                    }else{
                        panic!("Conditional elif without body");
                    };
                    retvec.push(Node::CONDITIONAL { condition: Some(Box::new(exp)), body: body });

                }else if self.is_else_statement() {

                    let body = if self.is_scope_start(){
                        self.conditional_body_parser()
                    }else{
                        panic!("Conditional else without body");
                    };
                    retvec.push(Node::CONDITIONAL { condition: None, body: body });
                    break;
                }else{
                    break;
                }
            }// loop end

            return Some(
                Node::CONDTIONAL_CHAIN(retvec)
            );
        }else if self.is_elif_statement(){
            panic!(" ELIF wihtout if");
        }else if self.is_else_statement(){
            panic!(" ELSE withot if");
        }else{
            return None;
        }
    }

    pub fn iterator_parser(&mut self) -> Option<Node> {

        if self.is_for_statement(){
            
            let exp = if let Some(exp) = self.binary_expression_parser(){
                Some(Box::new(exp))
            }else{
                None
            };

            let body = if self.is_scope_start(){
                self.iterator_body_parser()
            }else{
                panic!(" Iterator without body");
            };

            return Some(Node::ITERATOR { condition: exp, body: body });
        }
        
        None
    }

    pub fn assignment_parser(&mut self) -> Option<Node> {

        let checkpoint = self.index;

        let lhs = if let Some(val) =  self.identifer_parser(){
            val
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if !self.is_assignmet_operator(){
            return self.vomit_and_die(checkpoint);
        }

        let rhs = if let Some(val) = self.binary_expression_parser(){
            val
        }else{
            return self.vomit_and_die(checkpoint);
        };

        if self.is_semicolon(){
            return Some(
                Node::ASSIGNMENT { lhs: Box::new(lhs), rhs: Box::new(rhs) }
            );
        }else{
            panic!("Assignment statement does not end");
        }
    }

    pub fn return_statement_parser(&mut self) -> Option<Node> {
        if matches!(self.current() , Token::KEYWORD(Keyword::RETURN)){
            self.consume();

            let exp = if let Some(val) = self.binary_expression_parser(){
                Some(Node::RETURN { val: Some(Box::new(val)) })
            }else{
                None
            };

            if self.is_semicolon(){
                return exp;
            }else{
                panic!("Return statement does not end and the expression was {:#?}", exp);
            }
            
        }else{
            return None;
        }
    }

    pub fn break_statement_parser(&mut self) -> Option<Node> {
        if matches!(self.current() , Token::KEYWORD(Keyword::BREAK)){
            self.consume();

            return Some(Node::BREAK);
        }
        None
    }

    pub fn conditional_body_parser(&mut self) -> Vec<Node> {
        let mut retvec : Vec<Node> = Vec::new();

        loop {

            if let Some(val) = self.variable_declaration_parser(){
                retvec.push(val);
            }else if let Some(val) = self.assignment_parser(){
                retvec.push(val);
            }else if let Some(val) = self.conditional_chain_parser(){
                retvec.push(val);
            }else if let Some(val) = self.return_statement_parser(){
                retvec.push(val);
            }else if let Some(val) = self.iterator_parser(){
                retvec.push(val);
            }else if let Some(val) = self.break_statement_parser(){
                retvec.push(val);
            }else if let Some(val) = self.function_call_parser(){
                retvec.push(val);
            }else if let Some(_) = self.function_declaration_parser(){
                panic!("Function declaration inside a conditional");
            }else if let Some(_) = self.structure_declaration_parser(){
                panic!("Structure declaration inside a conditional");
            }else if self.is_scope_end(){
                break;
            }
        } // loop endk
        return retvec;
    }

    pub fn iterator_body_parser(&mut self) -> Vec<Node> {
        let mut retvec : Vec<Node> = Vec::new();

        loop {

            if let Some(val) = self.variable_declaration_parser(){
                retvec.push(val);
            }else if let Some(val) = self.assignment_parser(){
                retvec.push(val);
            }else if let Some(val) = self.conditional_chain_parser(){
                retvec.push(val);
            }else if let Some(val) = self.return_statement_parser(){
                retvec.push(val);
            }else if let Some(val) = self.iterator_parser(){
                retvec.push(val);
            }else if let Some(val) = self.break_statement_parser(){
                retvec.push(val);
            }else if let Some(val) = self.function_call_parser(){
                retvec.push(val);
            }else if let Some(_) = self.function_declaration_parser(){
                panic!("Function declaration inside a iterator");
            }else if let Some(_) = self.structure_declaration_parser(){
                panic!("Structure declaration inside a iterator");
            }else if self.is_scope_end(){
                break;
            }
        } // loop endk
        return retvec;
    }
 

    pub fn function_body_parser(&mut self) -> Vec<Node> {

        let mut retvec : Vec<Node> = Vec::new();

        loop {
            if let Some(val )  = self.variable_declaration_parser(){
                retvec.push(val);
            }else if let Some(val) = self.assignment_parser(){
                retvec.push(val);
            }else if let Some(val) = self.iterator_parser(){
                retvec.push(val);
            }else if let Some(val) = self.conditional_chain_parser(){
                retvec.push(val);
            }else if let Some(val) = self.return_statement_parser(){
                retvec.push(val);
            }else if let Some(_) = self.function_declaration_parser(){
                panic!("Nested functions not supported");
            }else if let Some(_) = self.structure_declaration_parser(){
                panic!("Structure declaration inside function body not supported");
            }else if let Some(val) = self.function_call_parser(){
                retvec.push(val);
            }else if self.is_scope_end(){
                break;
            }
        } // loop end
        
        return retvec;
    }

}
