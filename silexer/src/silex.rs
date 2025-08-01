#![allow(non_camel_case_types)]

use std::fs;
use std::process;
 
#[derive(Debug, Clone)]
pub enum Operator{
    ADD, 
    SUB,
    MUL,
    DIV,
    BWAND,
    BWOR,

    AND,
    OR,

    EQ,

    CGT,
    CLT,

    CNQ,
    CEQ,
    MOD,
}

#[derive(Debug, Clone)]
pub enum Special {
    // ( )
    ROUND_START,
    ROUND_END,

    // { }
    CURLY_START,
    CURLY_END,

    // [ ]
    SQUARE_START,
    SQUARE_END,

    // , . ; :
    COMMA,
    DOT,
    SEMICOLON,
    COLON,
}

#[derive(Debug, Clone)]
pub enum Keyword{
    // data types
    I32,
    F32,
    I64,
    F64,
    STRING,
    CHAR,
    STACK,
    POINTER,
    
    // statements
    FN,
    IF,
    ELIF,
    ELSE,
    FOR,
    STRUCT,
   
    // these are the prefix operators 
    REFRENCE,
    DREFRENCE,
    INCREMENT,
    DECREMENT,
}

#[derive(Debug, Clone)]
pub enum Token{
    INTEGER(i64),
    STRING(String),
    FLOAT(f64),
    CHAR(char),

    IDENTIFIER(String),

    OPERATOR(Operator),
    STC(Special),
    KEYWORD(Keyword),
    EOF,
}

pub struct SiLexer{

    pub inp : Vec<char>,
    pub index : usize ,
}

impl SiLexer{

    pub fn new(file_path : String ) -> Self {
        let mut file_contents : Vec<char> = match fs::read_to_string(file_path.clone()){
            Ok(val) => val.chars().collect() ,
            Err(e) => {
                println!("The input file : {} cannot be found or cannot be read \n {} \n" , file_path, e);
                process::exit(-1);
            }
        };

        file_contents.push('\0');

        return Self { inp: file_contents , index: 0 };
    }

    pub fn lex(&mut self ) -> Vec<(Token , usize)> {
        
        let mut buffer : Vec<(Token , usize )> = Vec::new();

        loop {
            if self.is_eof(){
                buffer.push((Token::EOF , 0));
                break;

            }else if self.is_space(self.current()){
                self.consume();

            }else if self.is_single_line_comment(){
                self.lex_commnets();
            
            }else if let Some(token)  = self.lex_stc(){
                buffer.push(token);
            }
            
            
        }

        return buffer;
    }

    pub fn lex_no_pos(&mut self ) -> Vec<Token> {
        let lex = self.lex();
        
        let mut buffer : Vec<Token> = Vec::new();

        for (token , _) in lex {
           buffer.push(token);
        }

        return buffer;
    }

    fn peek(&self ) -> Option<char> {
        if self.index != self.inp.len() - 1{
            return Some(self.inp[self.index + 1]);
        }
        None
    } 

    fn current(&self ) -> char {
        return self.inp[self.index];
    }

    fn consume(&mut self){
       if self.is_eof(){
           panic!("Tried to consume EOF token");
       }
       self.index += 1;
    } 

    fn vomit_and_die<T> (&mut self ,checkpoint : usize ) -> Option<T> {
        self.index = checkpoint;
        return None;
    }


    fn is_eof(&self ) -> bool {
        matches!(self.current() , '\0') 
    }

    fn is_space(&self, c: char) -> bool {
        matches!(c, '\t' | ' ' | '\n') 
    }

    fn is_operator(&self , c : char) -> bool {
        matches!(c , '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '%' | '=' )
    }

    fn is_stc(&self , c : char) -> bool {
        matches!(c , '{' | '}' | '(' | ')' | '[' | ']' | ',' | '.' | ':' | ';' )
    }

    fn is_symbol(&self , c : char ) -> bool {
        !c.is_alphabetic()
    }

    fn is_number(&self ,c: char) -> bool {
        c.is_ascii_digit() // Fast path for ASCII
            || c.is_digit(10)  
    }

    pub fn is_single_line_comment(&mut self) -> bool {
        
        let condition_one = if  self.current() == '/'{
            true
        }else{
            false
        }; 

        let condition_two = if let Some(c) = self.peek(){
            if c == '/'{
                true
            }else{
                false
            }
        }else{
            false
        };

        if condition_one == true && condition_two == true {
            return true;
        }else{
            return false;
        }

    }

    pub fn lex_commnets(&mut self ){

        loop {
            if self.current() == '\n'{
                self.consume();
                break;
            }
            self.consume();
        }
    }

    // priority 5
    pub fn lex_char_literal(&mut self ) -> Option<(Token , usize)> {
        let index = self.index;
        if self.current() == '\''{
            self.consume();
        }else {
            return None;
        }

        let mut buffer = String::new();

        if matches!(self.current() , '\\'){
            self.consume();
            match self.current() {
               'n' => buffer.push('\n'), 
               't' => buffer.push('\t'),
               '\\' => buffer.push('\\'),
               _ => panic!("Unknown escape"),
            }
            self.consume();
        }else{
           buffer.push(self.current());
           self.consume();
        }

        if self.current() == '\'' {
            self.consume();
            let retval : char = match buffer.parse(){
                 Ok(val) => val,
                 Err(e) => todo!("Error handler should report this , not a char \n{}", e),
            };
            return Some((Token::CHAR(retval),index));
        }else{
            panic!("character literal contains too many characters");
        }
        
    }


    // priority 4
    pub fn lex_string_literal(&mut self) -> Option<(Token , usize)> {

        if let Some((token , index )) = self.lex_char_literal(){
            return Some((token , index));
        }
        let index = self.index;

        if self.current() == '"' {
            self.consume(); 
        }else{
            return None;
        }
        
        let mut buffer = String::new();

        loop {
            
            let ch = self.current();
            match ch {
                // Terminate on unescaped quote
                '"' => {
                    self.consume(); // Skip closing quote
                    return Some((Token::STRING(buffer), index));
                }
                // Handle escapes (e.g., \", \\, \n)
                '\\' => {
                    self.consume();

                    let escaped = self.current();

                    buffer.push(match escaped {
                        '"' => '"',  // \"
                        '\\' => '\\', // \\
                        'n' => '\n',  // \n
                        _ => panic!("Invalid escape: \\{} at position {}", escaped, self.index),
                    });
                    self.consume(); 
                }

                _ => {
                    buffer.push(ch);
                    self.consume();
                }
            }
        }
    }

    // priority 3
    pub fn lex_float_literals(&mut self ) -> Option<(Token , usize)> {

        if let Some((token , index)) = self.lex_string_literal(){
            return Some((token , index));
        }

        let mut buffer = String::new();
        let checkpoint = self.index;
        let mut dot_counter = false;

        let mut current = self.current();

        if self.is_number(current){
            buffer.push(current);
            self.consume();

            loop {
                current = self.current();
                if self.is_number(current){
                    buffer.push(current);
                    self.consume();

                }else if current == '.'{
                    
                    if dot_counter == true {
                        todo!(" Error handler should report this but the float literal found has too many dots");
                    }

                    dot_counter = true;

                    buffer.push(current);
                    self.consume();

                }else{
                    if self.is_space(self.current()){
                        break;
                    }else if self.is_symbol(self.current()){
                        break;
                    }else {
                        panic!("Syntax error , float literal cannot be lexed");
                    }
                    
                }
            }
        }else{
            return None;
        }

        if dot_counter == false {
            return self.vomit_and_die(checkpoint);
        }

        let retval : f64 = match buffer.parse(){
            Ok(val) => val,
            Err(e) => {
                println!(" Error handler should report this but the float lieral lexed is not a float literal \n{}", e);
                return self.vomit_and_die(checkpoint);
            }
        };
        
        return Some((Token::FLOAT(retval), checkpoint ));
    }


    // priority 2
    pub fn lex_integer_literal(&mut self ) -> Option<(Token , usize)> {

        if let Some((token , index)) = self.lex_float_literals(){
            return Some((token , index));
        }
    
        let mut buffer = String::new();
        let checkpoint = self.index;

        let mut current = self.current();
        if self.is_number(current){
            buffer.push(current);
            self.consume();

            loop {
                current = self.current();

                if self.is_number(current) {
                    buffer.push(current);
                    self.consume();
                }else{
                    if self.is_space(self.current()){
                        break;
                    }else if self.is_symbol(self.current()){
                        break;
                    }else{
                        panic!("Syntax error , the integer literal cannot be parsed");
                    }
                }
            }
        }else{
            return None;
        }


        let retval : i64 = match buffer.parse(){
            Ok(val) => val ,
            Err(e) =>{
                println!(" Error handler should report this , the error in integer lexer\n{}", e);
                return self.vomit_and_die(checkpoint);
            }
        };

        return Some((Token::INTEGER(retval), checkpoint));
    }

    // priority 1
    pub fn is_keyword(&self , buffer : &str) -> Option<Token> {

        match buffer {
            "i32"  => return Some(Token::KEYWORD(Keyword::I32)),
            "f32"  => return Some(Token::KEYWORD(Keyword::F32)),
            "i64"  => return Some(Token::KEYWORD(Keyword::I64)),
            "f64"  => return Some(Token::KEYWORD(Keyword::F64)),

            "string"  => return Some(Token::KEYWORD(Keyword::STRING)),
            "char"  => return Some(Token::KEYWORD(Keyword::CHAR)),
            "stack"  => return Some(Token::KEYWORD(Keyword::STACK)),
            "ptr"  => return Some(Token::KEYWORD(Keyword::POINTER)),

            "fn"  => return Some(Token::KEYWORD(Keyword::FN)),
            "if"  => return Some(Token::KEYWORD(Keyword::IF)),
            "elif"  => return Some(Token::KEYWORD(Keyword::ELIF)),
            "else"  => return Some(Token::KEYWORD(Keyword::ELSE)),
            "for"  => return Some(Token::KEYWORD(Keyword::FOR)),

            "struct"  => return Some(Token::KEYWORD(Keyword::STRUCT)),

            "ref"  => return Some(Token::KEYWORD(Keyword::REFRENCE)),
            "dref"  => return Some(Token::KEYWORD(Keyword::DREFRENCE)),
            "inc" => return Some(Token::KEYWORD(Keyword::INCREMENT)),
            "dec" => return Some(Token::KEYWORD(Keyword::DECREMENT)),

            _ => return None,
        }
    }

    // priority 0
    pub fn lex_identifier(&mut self ) -> Option<(Token , usize )> {
        if let Some((token , index)) = self.lex_integer_literal(){
            return Some((token , index));
        }

        let index = self.index;
        let mut buffer = String::new();
        let mut current = self.current();

        if !self.is_number(current) && !self.is_symbol(current){
            buffer.push(current);
            self.consume();
            current = self.current();

            loop {
                if self.is_stc(current) || self.is_space(current) || self.is_operator(current) {
                    break;
                }else{
                    buffer.push(current);
                    self.consume();
                    current = self.current();  
                }
            }
        }else{
            return None;
        } 

        if let Some(token) = self.is_keyword(&buffer) {
            return Some((token , index));
        }else{
            return Some((Token::IDENTIFIER(buffer), index));
        }
    }

    pub fn is_valid_operator(&self , buffer : &str , index : usize )  -> Option<(Token , usize)> {
        match buffer {
            "+" => return Some((Token::OPERATOR(Operator::ADD),index)),
            "-" => return Some((Token::OPERATOR(Operator::SUB),index)),
            "*" => return Some((Token::OPERATOR(Operator::MUL),index)),
            "/" => return Some((Token::OPERATOR(Operator::DIV),index)),

            "&" => return  Some((Token::OPERATOR(Operator::BWAND), index)),
            "|" => return  Some((Token::OPERATOR(Operator::BWOR), index)),

            "&&" => return Some((Token::OPERATOR(Operator::AND), index)),
            "||" => return Some((Token::OPERATOR(Operator::OR), index)),
            "=" => return  Some((Token::OPERATOR(Operator::EQ),index)),

            ">" => return  Some((Token::OPERATOR(Operator::CGT) , index)),
            "<" => return  Some((Token::OPERATOR(Operator::CLT), index)),

            "==" => return Some((Token::OPERATOR(Operator::CEQ), index)),
            "!=" => return Some((Token::OPERATOR(Operator::CNQ) , index)),

            "%" => return  Some((Token::OPERATOR(Operator::MOD) , index)),
            _ => return None,
        }
    }

    // priority -1
    pub fn lex_operators(&mut self ) -> Option<(Token , usize)> {
        
        if let Some((token , index )) = self.lex_identifier(){
            return Some((token , index));
        }

        let index = self.index;
        let mut current = self.current();
        let mut buffer = String::new();
        
        if self.is_operator(current){
            buffer.push(current);
            self.consume();
            loop {
                current = self.current();
                if self.is_operator(current){
                    buffer.push(current);
                    self.consume();
                }else{
                    break;
                }
            }
        }else{
            return None;
        }

        return self.is_valid_operator(&buffer, index);
    }

    pub fn lex_stc(&mut self ) -> Option<(Token, usize)> {

        let index = self.index;
        if let Some((token , usize )) = self.lex_operators(){
            return Some((token , usize));
        }
        
        let current = self.current();

        match current {
            '(' => {
                self.consume();
                return Some((Token::STC(Special::ROUND_START), index));
            },
            ')' => {
                self.consume();
                return Some((Token::STC(Special::ROUND_END), index));
            },
            '{' => {
                self.consume();
                return Some((Token::STC(Special::CURLY_START), index));
            },
            '}' => {
                self.consume();
                return Some((Token::STC(Special::CURLY_END), index));
            },
            '[' => {
                self.consume();
                return Some((Token::STC(Special::SQUARE_START), index));
            },
            ']' => {
                self.consume();
                return Some((Token::STC(Special::SQUARE_END), index));
            },
            ',' => {
                self.consume();
                return Some((Token::STC(Special::COMMA), index));
            },
            '.' => {
                self.consume();
                return Some((Token::STC(Special::DOT), index));
            },
            ';' => {
                self.consume();
                return Some((Token::STC(Special::SEMICOLON), index));
            },
            ':' => {
                self.consume();
                return Some((Token::STC(Special::COLON), index));
            } ,
            _ => {
                return None;
            },
        }
    }
}
