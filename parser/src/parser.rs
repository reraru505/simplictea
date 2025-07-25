use crate::ast::{AST_Expression, AST_Node};
use crate::lexer::lex::Lexeme;
use crate::lexer::token_type::Token;

use crate::symboltable::symbol::SymbolTable;

pub struct Parser{
    pub tokens : Vec<Token>,
    pub current : usize ,
    pub symtable : SymbolTable,
}


impl Parser {

    pub fn parse(&mut self) -> Vec<AST_Node> {

        let mut ast : Vec<AST_Node> = Vec::new();

        while ! self.parser_should_end(){

            println!("\n\ncurrent = {} , len = {}\n ", self.current , self.tokens.len());

            if let Some(var_dec) = self.Variable_Declaration_Parser(){

                println!("variable declaration parser called");
                ast.push(
                    AST_Node::statement(var_dec)
                );
            }

            else if let Some(fn_dec) = self.Function_Declaration_Parser(){
                ast.push(
                    AST_Node::statement(fn_dec)
                );
            }

            else if let Some(var_assign) = self.Assignment_Parser(){
                println!("Assignment parser called");

                ast.push(
                    AST_Node::statement(var_assign)
                );
            }

            else if let Some(conditional ) = self.Conditional_Parser(){
                ast.push(
                    AST_Node::statement(conditional)
                );
            }

            else if let Some(ret) = self.Return_Statement_Parser(){
                ast.push(
                    AST_Node::statement(ret)
                );
            }

            else if let Some(itera) = self.Iterator_Parser(){
                ast.push(
                    AST_Node::statement(itera)
                );
            }

            else if let Some(structure) = self.Struct_Parser(){
                
                println!("Structure parser called");
                
                ast.push(
                    AST_Node::statement(structure)
                );
            }else{
                println!("\n\n\nParseed unconditionally , check for token {:#?}\n\n\n" , self.current_token());
                if !self.consume(){
                    panic!("some statement is unknown");
                }

            }
            
 
        }

        return ast;

    }

    // create a new instance for the parser
    pub fn new(lex : Vec<Lexeme>) -> Self {
        let mut tokens : Vec<Token> = Vec::new();

        for i in lex.iter(){
            tokens.push(i.tokens.clone());
        }

        tokens.push(Token::t_eof);

        return Parser{
            tokens : tokens , 
            current : 0 ,
            symtable : SymbolTable::new(),
        }
    }

    pub fn parser_should_end(&self ) -> bool {
        return self.is_at_end();
    }

    // checking if the current pointer is at the 
    // end of the tokens array
    pub fn is_at_end(&self ) -> bool {
        if matches!(self.current_token() , Token::t_eof){
            return  true;
        }
        return false;
    }

    // checking if the current pointer is at the 
    // start of the tokens array
    pub fn is_at_start(&self ) -> bool {
        if self.current == 0 {
            return true;
        }
        return false;
    }

    // peeking the next token and seeing if it matches
    pub fn peek(&self , steps : usize) -> Option<Token>{

        if self.is_at_end(){
            return None;
        }

        let index = self.current + steps;
        if index >= self.tokens.len() {
            return None;
        }
        return Some(self.tokens[index].clone());
 
    }

    // checking if the previous token is the expected token
    pub fn prev(&self , expected : Token) -> bool{

        if self.is_at_start() {
            return false;
        }

        let index = self.current.clone() - 1;

        if matches!(self.tokens[index].clone() , expected){
            return true;
        }
        return false;  
    }


    // chek if the token is as exprected
    pub fn check(&self , expected : Token) -> bool {
        let index = self.current.clone();
        if matches!(self.tokens[index].clone() , expected){
            return true;
        }
        return false;
    }

    pub fn current_token(&self) -> Token {
        
        return self.tokens[self.current].clone();
    }

    // advancing the current pointer 
    pub fn consume(&mut self) -> bool {
        
        if self.is_at_end(){
            return false;
        }
        self.current += 1;
        return true;
    }

    // gives back the consumed token
    pub fn vomit(&mut self , checkpoint : usize ){
        self.current = checkpoint ;
    }

    pub fn  vomit_and_die<T>(&mut self , checkpoint : usize) -> Option<T>{
        self.current = checkpoint;
        return None;
    }

    // check if the the token matches and advance the current pointer
    pub fn match_consume(&mut self , expected : Token ) -> bool {
        if self.check(expected) {
            self.current += 1;
            return true;
        }else{
            return false;
        }
    }

}
