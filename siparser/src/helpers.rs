use crate::silexer::silex::{Token, Keyword, Special, Operator };
use crate::siparser::SiParser;
use crate::ast::{Data_Type , Node};


impl SiParser {

    pub fn current (&self) -> Token {
        return self.tokens[self.index].clone();
    }

    pub fn consume (&mut self ) -> bool  {

        if !self.is_eof(){
            self.index += 1;
            return true;
        }

        panic!("Tried to consume eof");
        return false;
    }

    pub fn is_eof (&self ) -> bool {
        matches!(self.current() , Token::EOF)
    }

    pub fn vomit_and_die<T>(&mut self , checkpoint : usize ) -> Option<T> {
        self.index = checkpoint;
        return None;
    }

    pub fn is_param_start(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::ROUND_START)){
            self.consume();
            return true;
        }
        false
    }
    pub fn is_param_end(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::ROUND_END)){
            self.consume();
            return true;
        }
        false
    }
    pub fn is_scope_start(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::CURLY_START)){
            self.consume();
            return true;
        }
        false
    }
    pub fn is_scope_end(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::CURLY_END)){
            self.consume();
            return true;
        }
        false
    } 
    pub fn is_comma(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::COMMA)){
            self.consume();
            return true;
        }
        false
    } 
    pub fn is_dot(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::DOT)){
            self.consume();
            return true;
        }
        false
    }
    pub fn is_colon(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::COLON)){
            self.consume();
            return true;
        }
        false
    }
    pub fn is_semicolon(&mut self ) -> bool {
        if matches!(self.current() , Token::STC(Special::SEMICOLON)){
            self.consume();
            return true;
        }
        false
    }  

    pub fn identifer_parser(&mut self) -> Option<Node> {
       if let Token::IDENTIFIER(val) = self.current() {
           self.consume();
           return Some(Node::IDENTIFER(val));
       }
       None
    }
    
    pub fn identifer_name_parser(&mut self) -> Option<String> {
       if let Token::IDENTIFIER(val) = self.current() {
           self.consume();
           return Some(val);
       }
       None
    }
 
    pub fn integer_parser(&mut self) -> Option<Node> {
       if let Token::INTEGER(val) = self.current() {
           self.consume();
           return Some(Node::INTEGER(val));
       }
       None
    }
   
    pub fn float_parser(&mut self) -> Option<Node> {
       if let Token::FLOAT(val) = self.current() {
           self.consume();
           return Some(Node::FLOAT(val));
       }
       None
    } 
    pub fn string_parser(&mut self) -> Option<Node> {
       if let Token::STRING(val) = self.current() {
           self.consume();
           return Some(Node::STRING(val));
       }
       None
    }
    pub fn char_parser(&mut self) -> Option<Node> {
       if let Token::CHAR(val) = self.current() {
           self.consume();
           return Some(Node::CHAR(val));
       }
       None
    } 

    pub fn function_call_parser(&mut self ) -> Option<Node> {

        let checkpoint = self.index;
        let name = if let Some(Node::IDENTIFER(id)) = self.identifer_parser(){
           id
        }else{
            return None;
        };

        if self.is_param_start(){
            let args = self.get_function_call_args();
            return Some(
                Node::FUNCTION_CALL { name: name, args: args }
            )
        }else{
           return self.vomit_and_die(checkpoint) ;
        }
        
    }

    pub fn get_function_call_args(&mut self) -> Vec<Node> {
        let mut retval : Vec<Node> = Vec::new();

        if let Some(exp) = self.binary_expression_parser(){
            retval.push(exp);
        }

        loop {
            if self.is_param_end(){
                break;
            }else if self.is_comma(){
                
                if let Some(exp) = self.binary_expression_parser(){
                    retval.push(exp);
                }

            }
        }

        return retval;
    }

    pub fn value_parser(&mut self) -> Option<Node> {
        self.function_call_parser()
            .or_else(|| self.char_parser()
                .or_else(|| self.string_parser() 
                    .or_else(|| self.float_parser()
                        .or_else(|| self.integer_parser()
                            .or_else(|| self.identifer_parser())))))

    }

    pub fn primary_parser(&mut self ) -> Option<Node> {
        let checkpoint = self.index;

        if let Some(val) = self.value_parser(){
            return Some(val);

        }else if self.is_param_start(){
             if let Some(exp) = self.binary_expression_parser(){
                 return Some(exp);
             }else{
                 return self.vomit_and_die(checkpoint);
             }
        }else{
            return None;
        }
    }

    pub fn multiplicative_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.primary_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::MUL) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::MUL,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else if matches!(self.current() , Token::OPERATOR(Operator::DIV) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::DIV,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }
  
        }// loop end
    }

    pub fn additive_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.multiplicative_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::ADD) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::ADD,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else if matches!(self.current() , Token::OPERATOR(Operator::SUB) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::SUB,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }

        }// loop end
    }

    pub fn relational_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.additive_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::CGT) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::CGT,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else if matches!(self.current() , Token::OPERATOR(Operator::CLT) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::CLT,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }

        }// loop end
    }
 

    pub fn equality_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.relational_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::CEQ) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::CEQ,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else if matches!(self.current() , Token::OPERATOR(Operator::CNQ) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::CNQ,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }

        }// loop end
    }

    pub fn bitwise_and_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.equality_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::BWAND) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::BWAND,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }

        }// loop end
    }

    pub fn bitwise_or_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.bitwise_and_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::BWOR) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::BWOR,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }

        }// loop end
    }
    pub fn  logical_and_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.bitwise_or_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::AND) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::AND,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }

        }// loop end
    }
    pub fn  logical_or_parser(&mut self ) -> Option<Node>{
        let checkpoint = self.index;
        let mut left = if let Some(val) = self.logical_and_parser(){
            val
        }else{
            return None;
        };

        loop {
            if matches!(self.current() , Token::OPERATOR(Operator::OR) ){
                if !self.consume(){
                    return self.vomit_and_die(checkpoint);
                }

                let right = if let Some(val) = self.primary_parser(){
                    val
                }else{
                    return self.vomit_and_die(checkpoint);
                };

                left = Node::BINARY_EXPRESSION { 
                    operator: Operator::OR,
                    left: Box::new(left),
                    right: Box::new(right)
                }; 

            }else{
                return Some(left);
            }

        }// loop end
    }

    pub fn binary_expression_parser(&mut self ) -> Option<Node> {
        self.logical_or_parser()
            .or_else(|| self.primary_parser())
    }

    pub fn function_param_parser(&mut self ) -> Vec<Node> {

        let mut retvec : Vec<Node> = Vec::new();

        if self.is_param_end(){
            return retvec;
        }

        loop {
            let name = if let Some(id) = self.identifer_name_parser(){
                id
            }else{
                panic!("syntax error");
            };

            if !self.is_colon(){
               panic!("syntax error");
            }

            let d_type = self.data_type_parser();

            if self.is_comma(){
                retvec.push(Node::PARAM { name: name, d_type: d_type });
            }else if self.is_param_end(){
                retvec.push(Node::PARAM { name: name, d_type: d_type });
                break;
            }
            
        } // loop end

        return retvec;
    }

    pub fn data_type_parser(&mut self ) -> Data_Type {

        match self.current() {
            Token::KEYWORD(Keyword::I32) => return Data_Type::I32,
            Token::KEYWORD(Keyword::F32) => return Data_Type::F32,
            
            Token::KEYWORD(Keyword::I64) => return Data_Type::I64,
            Token::KEYWORD(Keyword::F64) => return Data_Type::F64,
            
            Token::KEYWORD(Keyword::STRING) => return Data_Type::STRING,
            Token::KEYWORD(Keyword::CHAR) => return Data_Type::CHAR,
            
            Token::KEYWORD(Keyword::STACK) => return Data_Type::STACK,
            Token::KEYWORD(Keyword::POINTER) => return Data_Type::POINTER,

            _ => panic!("Not a Data_Type"),
        }

    }

}



