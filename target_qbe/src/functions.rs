use std::collections::HashMap;
use std::panic;
use indexmap::IndexMap;
use crate::gen_ir::STATEMENT;
use crate::parser::ast::*;


impl STATEMENT {
   
    pub fn write_function(&mut self  , value_to_parse : AST_Node) -> bool {

        let mut buffer = String::new();

        if let AST_Node::statement(
            AST_Statement::FUNCTION { name, type_annotation, params, body }
        ) = value_to_parse {

            // getting the function name 
            let fname = if let AST_Expression::Identifier(id) = name {
                id
            }else{
                panic!("function name not an identifier");
            };

            // getting the return size of the function
            let size = if let Some(typ ) = type_annotation {
                self.th.get_types_size(typ)  
            }else {
                String::new()
            };

            self.idmap.insert(fname.clone(), size.clone());

            let fparams = if let Some(p) = params{
                self.pararm_handler(p,fname.clone())
            }else{
                String::new()
            };



            self.writer.push_str(

                &format!("\nexport function {} ${}({}){{ \n@start\n\n",
                         size , fname , fparams )
            );

            let fbody = self.function_body_handler(body, size);

            self.writer.push_str(
                &format!("{} \n}}\n", fbody)
            );

            return true;
        }
    

        false
    }

    pub fn pararm_handler(&mut self, p : Vec<AST_Statement> , fname : String) -> String {

        let mut buffer = String::new();
        let mut map : IndexMap<String, String> = IndexMap::new();

        for i in p {
            if let AST_Statement::LET { name, type_annotation, initializer } = i {

                let n = if let AST_Expression::Identifier(id) = name {
                    id
                }else{
                    panic!(" The name of the param is not an identifier");
                };

                let size = self.th.get_types_size(type_annotation);

                map.insert(n.clone(), size.clone());
                self.idmap.insert(format!("{}_param_{}",fname , n.clone()), size.clone());

                buffer.push_str(&format!("{} %{}," , size , n));
            }
        }

        self.param_map.insert(fname , map);

        buffer.pop();

        buffer
    }

    pub fn function_body_handler(&mut self , p : Vec<AST_Node> , size : String) -> String {
        
        let mut val = String::new();
        let mut has_return = false;
        
        for i in p.iter() {
           if let Some(exp) = self.write_declaration_inside_function_body(i.clone()) {
               val.push_str(&exp);
           }else if let Some(exp) = self.write_assignment_inside_function_body(i.clone()) {
               val.push_str(&exp);
           }else if let AST_Node::statement(AST_Statement::RETURN(exp)) = i.clone() {

               let last = self.write_binary_expression_from_postfix_tokens(exp, size.clone());
               val.push_str(
                   &format!("\tret {}\n", last)
               );
               has_return = true;
           }
        }

        if has_return == false {
            val.push_str(
                &format!("\tret\n")
            );
        }

        return val;
    }


    pub fn write_declaration_inside_function_body(&mut self  , value_to_parse : AST_Node) -> Option<String> {

        let mut buffer = String::from("\n");

        if let AST_Node::statement(
            AST_Statement::LET { name, type_annotation, initializer }
        ) = value_to_parse {

            let size = self.th.get_types_size(type_annotation);

            let lhs = if let AST_Expression::Identifier(val) = name {
                
                self.idmap.insert(val.clone(), size.clone());
                buffer.push_str(
                    &format!("\t%{} ={} copy 0\n", val , size)
                );
                val

            }else{
                panic!("identfier is not a name , wtf , how can this even happen , fuck rust");
            };

            if let Some(init) = initializer {
                 
                let last = self.write_binary_expression_from_postfix_tokens(init , size.clone());

                buffer.push_str(
                    &format!("\t%{} ={} copy {}\n",lhs , size , last)
                );



            }

            return Some(buffer);
        }

        None
    }

    pub fn write_assignment_inside_function_body(&mut self , value_to_parse : AST_Node ) -> Option<String> {
    
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
                &format!("\t%{} ={} copy {}\n",lhs , size , last)
            );

            
            return Some(buffer);
        }
 
        return None; 
        
    }
                              
    pub fn write_function_call(&mut self , value_to_parse : AST_Node , buffer : &mut String ) -> Option<String> {

        if let AST_Node::expression(AST_Expression::Call { calee, argumments }) = value_to_parse {
            
            let fname = if let AST_Expression::Identifier(cname ) = *calee{
               cname
            }else{
                panic!("The function called name is not an identfier");
            };

            let size = self.idmap.get(&fname).unwrap().clone();

            let x = format!("%tmp_var_{}", self.tmp_counter);

            if let Some(exp) = argumments {
                buffer.push_str(
                    &format!("\t{} ={} call ${} ({})\n",x , size ,fname , self.write_function_call_arguments(fname.clone(), exp))
                );
            }else{
                buffer.push_str(
                    &format!("\t{} ={} call ${} ()\n",x , size ,fname )
                ); 
            }

            self.tmp_counter += 1;
            return Some(x);
        }

        return None;
    }

    pub fn write_function_call_arguments(&mut self , fname : String , 
        nodes : Vec<AST_Expression> ) -> String {

        let mut params_len = 0;

        let params = if let Some(val) = self.param_map.get(&fname) {
            params_len = val.len();
            val
        }else{
            panic!("The function <{}> not found", fname);
        };

        if params_len != nodes.len() {
            panic!("The function parameters arity do not match ");
        }

        let mut buffer = String::new();
        let sizevec = {
            let retval : Vec<String> = params.values().cloned().collect();
            retval
        };

        let mut counter = 0;

        for i in nodes {
            let size = sizevec[counter].clone();
            counter += 1;

            buffer.push_str(&format!("{} {}" , size.clone() , self.write_binary_expression_from_postfix_tokens(i,size ) ));
            buffer.push(',');
        }

        buffer.pop();
        return buffer;
    }
 

}
