use std::collections::HashMap;
use parser::ast::*;
use parser::ast::AST_Node;
use indexmap::IndexMap;

pub enum QBE_TYPES{
    W,
    L,
    S,
    D,
    B,
    H,
    V,
    A(String),
}

pub enum GLOBAL{
    STRING{
        name : String,
        data : String,
    },

    FLOAT{
        name : String,
        data : String,
    }
}

pub struct GLOBAL_Handler{
    pub globals : Vec<GLOBAL>,
    pub counter : usize,
}


impl GLOBAL_Handler {

    pub fn new() -> Self {
        Self {
            globals : Vec::new(),
            counter : 0,
        }
    }

    pub fn register_string(&mut self , val : String ) -> String {

        let nam = format!("$global_{}" , self.counter);
        self.counter += 1;
        self.globals.push(
GLOBAL::STRING { name: nam.clone(), data: val }
        );
        return nam;
    }

    pub fn register_float(&mut self , val : f64) -> String {
        let nam = format!("$global_{}" , self.counter);
        self.counter += 1;
        self.globals.push(
            GLOBAL::FLOAT { name: nam.clone(), data: format!("{}", val) }
        );
        return nam; 
    }

    pub fn write_globals(&self ) -> String  {

        let mut retval = String::new();
        for i in self.globals.iter() {

            match i {
                GLOBAL::STRING { name, data } => {
                    retval.push_str(
                        &format!("data {} = {{ b {} , b 0 }}" , name , data)
                    );
                },
                GLOBAL::FLOAT { name, data } => {
                    retval.push_str(
                        &format!("data {} = {{ s s_{} }}", name , data)
                    );
                }
            }

        }

        return retval;
    }
    
}

pub struct TYPE{
    pub name : String ,
    pub items : Vec<QBE_TYPES>,
}

pub struct TYPE_Handler{
    pub types : Vec<TYPE>,
    pub counter : usize , 
    pub lookup : HashMap<String , usize>,
}

impl TYPE_Handler {
    pub fn new() -> Self {
        Self {
            types : Vec::new(),
            counter : 0,
            lookup : HashMap::new(),
        }
    }

    pub fn lookup_name_exists(&self , name : String ) -> bool {
        
        if self.lookup.contains_key(&name) {
            return true;
        }
        false
    }
    

    pub fn find_all_types_in_ast(&mut self , ast : Vec<AST_Node> ) {

        for i in ast.iter() {

            if let AST_Node::statement(AST_Statement::STRUCTURE { name, body }) = i.clone() {
                
                let ret_name = if let AST_Expression::Identifier(val) = name {
                    val
                }else{
                    panic!("Structure does not have a name");
                };

                let ret_items = {
                    let mut retvec : Vec<QBE_TYPES> = Vec::new();
                    for i in body {
                        if let AST_Node::statement(
                            AST_Statement::LET {
                                name, type_annotation,
                                initializer }) = i.clone() {
                            
                            retvec.push(self.get_types(type_annotation));
                        }
                    }
                    retvec
                };

                self.types.push(
                    TYPE { name: ret_name.clone(), items: ret_items }
                );
                self.lookup.insert(ret_name.clone(), self.counter);
                self.counter += 1;
            }

        }
        
    }

    pub fn write_types(&self ) -> String {
        let mut retval = String::new();

        for i in self.types.iter() {
            retval.push_str(&format!("type :{} = {{", i.name));

            for j in i.items.iter() {
                retval.push_str(&format!("{}" , j.write()));
            }
            retval.push('}');
            retval.push('\n');
        }
    
        return retval;
    }
}

pub struct STATEMENT{
    pub ast : Vec<AST_Node>,
    pub counter : usize,
    pub writer : String , 
    pub gh : GLOBAL_Handler,
    pub th : TYPE_Handler,
    pub idmap : HashMap<String , String>,
    pub param_map : HashMap<String , IndexMap<String , String>>,
    pub tmp_counter : usize,
    
}

impl STATEMENT {

    pub fn new(inp : Vec<AST_Node>) -> Self{
        let mut new_inp = inp;
        new_inp.push(AST_Node::END);

        Self {
            ast : new_inp ,
            counter : 0,
            writer : String::new(),
            gh : GLOBAL_Handler::new(),
            th : TYPE_Handler::new(),
            idmap : HashMap::new(),
            param_map : HashMap::new(),
            tmp_counter : 0,
        }
    }

    pub fn try_and_find(&mut self ) {
        let ast = self.ast.clone();
        for i in  ast  {

            self.write_function(i);
        }
    }

    pub fn current_node(&self ) -> AST_Node {

        let val = self.ast[self.counter].clone();

        if let AST_Node::END = val {
            panic!("Tried to access node at the end");
        }
        return val;

    }

    pub fn consume(&mut self ) -> bool {
        if let AST_Node::END = self.current_node(){
            return false;
        }
        self.counter += 1;
        return true;
    } 

    pub fn write_declaration(&mut self ) -> bool {

        let value_to_parse = self.current_node();
        let mut buffer = String::from("\n");

        if let AST_Node::statement(
            AST_Statement::LET { name, type_annotation, initializer }
        ) = value_to_parse {

            let size = self.th.get_types_size(type_annotation);

            let lhs = if let AST_Expression::Identifier(val) = name {
                
                self.idmap.insert(val.clone(), size.clone());
                buffer.push_str(
                    &format!("%{} ={} copy 0\n", val , size)
                );
                val

            }else{
                panic!("identfier is not a name , wtf , how can this even happen , fuck rust");
            };

            if let Some(init) = initializer {
                 
                let last = self.write_binary_expression_from_postfix_tokens(init , size.clone());

                buffer.push_str(
                    &format!("%{} ={} copy {}\n",lhs , size , last)
                );



            }

            self.writer.push_str(&buffer);

            if !self.consume(){
                panic!("The consumption of the node is not possible");
            }
            
            return true;
        }
 
        return false;
    }

    pub fn write_assignment(&mut self ) -> bool {
    
        let value_to_parse = self.current_node();
        let mut buffer = String::from("\n");

        if let AST_Node::statement(
            AST_Statement::ASSIGNMENT { LHS, RHS }
        ) = value_to_parse {

             
            let (lhs , size) = if let AST_Expression::Identifier(name ) = LHS {
                (name.clone(), self.get_identifier_size(name ))

            }else{
                panic!("LHS is not an identifier");
            };

            let last = self.write_binary_expression_from_postfix_tokens(RHS , size.clone());

            buffer.push_str(
                &format!("%{} ={} copy {}\n",lhs , size , last)
            );

            self.writer.push_str(&buffer);


            if !self.consume(){
                panic!("The consumption of the node is not possible");
            }
            
            return true;
        }
 
        return false; 
        
    }

    
    pub fn get_identifier_size(&self , id : String) -> String {
        match self.idmap.get(&id){
            Some(v) => return v.clone(),
            _ => panic!("Identifer definition not found"),
        }
    }


}







