use crate::gen_ir::STATEMENT;
use crate::parser::ast::*;



impl STATEMENT {

    pub fn write_binary_expression_from_postfix_tokens(&mut self , exp : AST_Expression , size : String ) -> String  {

        let mut buffer : String = String::new();

        let postfix = if let AST_Expression::BinaryExpression { operator, left, right } = exp.clone() {

            self.capture_binary_expression_traversal(exp)
        }else{
            let expclone = exp.clone();
            match exp {
                AST_Expression::IntegerLiteral(val) => {
                    return format!("{}",val);
                },  

                AST_Expression::DecimalLiteral(val) => {
                    return self.gh.register_float(val.clone());
                },

                AST_Expression::StringLiteral(val) => {
                    return self.gh.register_string(val.clone());
                },

                AST_Expression::CharacterLiteral(val) => {
                    return format!("{}",val);
                },

                AST_Expression::Identifier(val) => {
                    return format!("%{}", val);
                },

                AST_Expression::Call { calee, argumments } => {
                    
                    let retval = self.write_function_call(AST_Node::expression(expclone), &mut buffer).unwrap();
                    self.writer.push_str(&buffer);
                    return retval;
                }

                _ => panic!("This expression in not possible in binary expression"),

            }
        };

        let mut stack : Vec<String> = Vec::new();
        // @todo 
        // do not write to the writer and find a way to back propogate this 

        self.write_postfix_to_buffer(&mut buffer, &mut stack, postfix, size);

        self.writer.push_str(&buffer);

        return stack.pop().unwrap();


    }

    pub fn write_postfix_to_buffer(&mut self , buffer : &mut String , stack : &mut Vec<String> , postfix : Vec<AST_Expression> , size : String ) -> String {

        for i in postfix.iter() {

            match  i {

                AST_Expression::IntegerLiteral(val) => {
                    stack.push(format!("{}",val));
                },  

                AST_Expression::DecimalLiteral(val) => {
                    stack.push(self.gh.register_float(val.clone()));
                },

                AST_Expression::StringLiteral(val) => {
                    stack.push(self.gh.register_string(val.clone()));
                },

                AST_Expression::CharacterLiteral(val) => {
                    stack.push(format!("{}",val));
                },

                AST_Expression::Identifier(val) => {
                    stack.push(format!("%{}", val));
                },

                AST_Expression::Operator(op) => {
                    self.hanlde_operator_in_postfix(buffer, stack, op.clone(), size.clone());
                },

                _ => panic!("This expression in not possible in binary expression"),

            } // match end 

        }// for end

       String::new()
    }


    pub fn hanlde_operator_in_postfix(&mut self , buffer : &mut String , stack : &mut Vec<String> , operator : AST_Operator  , size : String ) {

        let mut local_tmp_counter = self.tmp_counter;

        match operator {

            AST_Operator::ADD => {

                let x = format!("%tmp_var_{}" , local_tmp_counter);
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let buffval = format!(
                    "\t{} ={} add {} , {}\n",
                    x , size , lhs , rhs
                );

                local_tmp_counter += 1;
                buffer.push_str(&buffval);
                stack.push(x);
            },

            AST_Operator::SUB => {

                let x = format!("%tmp_var_{}" , local_tmp_counter);
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let buffval = format!(
                    "\t{} ={} sub {} , {}\n",
                    x , size , lhs , rhs
                );

                local_tmp_counter += 1;
                buffer.push_str(&buffval);
                stack.push(x);
            },

            AST_Operator::MUL => {

                let x = format!("%tmp_var_{}" , local_tmp_counter);
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let buffval = format!(
                    "\t{} ={} mul {} , {}\n",
                    x , size , lhs , rhs
                );


                local_tmp_counter += 1;
                buffer.push_str(&buffval);
                stack.push(x);
            },

            AST_Operator::DIV => {

                let x = format!("%tmp_var_{}" , local_tmp_counter);
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let buffval = format!(
                    "\t{} ={} div {} , {}\n",
                    x , size , lhs , rhs
                );

                local_tmp_counter += 1;
                buffer.push_str(&buffval);

                stack.push(x);
            },

            AST_Operator::OR => {

                let x = format!("%tmp_var_{}" , local_tmp_counter);
                local_tmp_counter += 1;
                let y = format!("%tmp_var_{}" , local_tmp_counter);
                local_tmp_counter += 1;
                let z = format!("%tmp_var_{}" , local_tmp_counter);
                local_tmp_counter += 1; 

                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let buffval = format!(
                    "\t{} ={} cne{}  {} , 0\n\
                     \t{} ={} cne{}  {} , 0\n\
                     \t{} ={} or {} , {} \n",
                    x , size , size , lhs ,
                    y , size , size , rhs ,
                    z , size , x , y 
                );

                local_tmp_counter += 1;
                buffer.push_str(&buffval);

                stack.push(z);                  

            },

            AST_Operator::AND => {

                let x = format!("%tmp_var_{}" , local_tmp_counter);
                local_tmp_counter += 1;
                let y = format!("%tmp_var_{}" , local_tmp_counter);
                local_tmp_counter += 1;
                let z = format!("%tmp_var_{}" , local_tmp_counter);
                local_tmp_counter += 1; 

                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let buffval = format!(
                    "\t{} ={} cne{} {} , 0\n\
                     \t{} ={} cne{} {} , 0\n\
                     \t{} ={} and {} , {} \n",
                    x , size , size , lhs ,
                    y , size , size , rhs ,
                    z , size , x , y 
                );

                local_tmp_counter += 1;
                buffer.push_str(&buffval);

                stack.push(z);                  

            }
 

            _ => todo!("Unimplemented"),



        }

        self.tmp_counter = local_tmp_counter;

    }

    pub fn capture_value_from_expression(&mut self , node : AST_Expression) -> String  {

        match node {

            AST_Expression::Identifier(val) => {
                return format!("%{}",val);
            },
            AST_Expression::StringLiteral(val) => {
                todo!("Trying to operate on String Literal , the feature is unimplemented");
            },
            AST_Expression::DecimalLiteral(val) => {
                self.gh.register_float(val)
            },
            AST_Expression::IntegerLiteral(val) => {
                return format!("{}" ,val);
            },
            AST_Expression::CharacterLiteral(val) => {
                todo!("Trying to operate on Char literal , the feature is not implemented");
            },
            _ => panic!("not a value "),

        }
    }    

    pub fn capture_binary_expression_traversal(&self , node : AST_Expression) -> Vec<AST_Expression> {

        let mut capture : Vec<AST_Expression> = Vec::new();

        self.traverse_binary_expression(&node, &mut capture);

        println!(" captured postfix \n {:#?}" , capture);

        return capture;

    }


    pub fn traverse_binary_expression(&self , node : &AST_Expression , capture : &mut Vec<AST_Expression>)  {

        match node {

            AST_Expression::Identifier(_) => {
                capture.push(node.clone());
            },
            AST_Expression::IntegerLiteral(_) => {
                capture.push(node.clone());
            },
            AST_Expression::StringLiteral(_) => {
                capture.push(node.clone());
            },
            AST_Expression::DecimalLiteral(_) => {
                capture.push(node.clone());
            },
            AST_Expression::BinaryExpression { operator, left, right } => {
                self.traverse_binary_expression(&left, capture);
                self.traverse_binary_expression(&right, capture);
                capture.push(AST_Expression::Operator(operator.clone()));
            },
            _ => panic!("found something else inside the BinaryExpression"),

        }

    } 
}
