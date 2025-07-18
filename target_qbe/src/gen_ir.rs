






























//use qbe::{Block, DataDef, TypeDef};
//use qbe::BlockItem;
//use qbe::{Module, Function, Linkage, Type, Value, Instr};
//use qbe::Statement;
//use qbe::DataItem;
//
//use crate::parser::ast::AST_Node;
//use crate::parser::ast::AST_Statement;
//use crate::parser::ast::AST_Expression;
//use crate::parser::ast::AST_TypeAnnotation;
//use crate::parser::ast::AST_Operator;
//
//use std::rc::Rc;
//use std::cell::RefCell;
//
//#[derive(Clone, Debug)]
//pub enum PostfixToken{
//    Operator(AST_Operator),
//
//    IntegerLit(i64),
//    DecimalLit(f64),
//    StringLit(String),
//    CharacterLit(char),
//
//    Identifier(String),
//}
//
//
//// @todo : Note to myself , please ditch this as soon as possible
//// this is shit , this is garbage 
//
//#[derive(Clone, Debug)]
//pub struct GEN_IR <'a>{
//    pub nodes : Vec<AST_Node>,
//    pub type_cache : Vec<TypeDef<'a>>,
//    pub module : Module<'a>,
//    pub global_counter : usize ,
//}
//
//impl <'a>GEN_IR<'a>{
//
//    pub fn new(nodes : Vec<AST_Node>) -> Self {
//        return Self{
//            nodes : nodes,
//            type_cache : Vec::new(),
//            module : Module::new(),
//            global_counter : 0, 
//        };
//    }
//
//
//
//    pub fn function_resolver(&mut self , node : AST_Node , &mut module : Module) -> bool  {
//        if let AST_Node::statement(
//            AST_Statement::FUNCTION { name, type_annotation, params, body }
//            )  = node {
//
//            let mut func : Function;
//
//            if let Some(typ) = type_annotation && let Some(args) = params {
//                func = Function::new(
//                    Linkage::public() ,
//                    self.extract_identifier(name),
//                    self.extract_args(args),
//                    Some(self.extract_type(typ)),
//                );
//
//            }else if let Some(args) = params && matches!(type_annotation , None) {
//                func = Function::new(
//                    Linkage::public(),
//                    self.extract_identifier(name),
//                    self.extract_args(args),
//                    None,
//                    );
//            }
//
//            let mut block = func.add_block("start");
//
//        }
//
//        return false;
//    }
//
//    pub fn body_resolver(&mut self , body : Vec<AST_Node> ) -> Vec<BlockItem>{
//                
//    }
//
//    fn variable_declaration_in_block(&mut self, node: AST_Node) -> Vec<BlockItem> {
//
//        let mut retval: Vec<BlockItem> = Vec::new();
//
//        if let AST_Node::statement(AST_Statement::LET { name, type_annotation, initializer }) = node {
//            // Extract variable name
//            let var_name = self.extract_identifier(name);
//            let val_ptr = Value::Temporary(var_name.clone());
//
//            // Get type information
//            let var_type = self.extract_type(type_annotation.clone());
//            let alloc_size = self.extract_type_allocation_size(type_annotation.clone());
//
//            // Always allocate space for the variable
//            
//
//                retval.push(BlockItem::Statement(
//                        Statement::Assign(
//                            val_ptr.clone(),
//                            var_type.clone(),
//                            self.extract_type_allocation_size(type_annotation)
//                        )
//                ));
//
//            
//
//            if let Some(initializer_expr) = initializer {
//                // Generate code for initializer expression
//                let init_code = self.binary_expression_in_block(initializer_expr);
//                retval.extend(init_code);
//
//                // Get the last temporary from the initializer code
//                let init_value = if let Some(BlockItem::Statement(Statement::Assign(
//                            Value::Temporary(last_temp), _, _)
//                )) = retval.last() {
//                    Value::Temporary(last_temp.clone())
//                } else {
//                    // Fallback if no temporary found
//                    Value::Const(0)
//                };
//
//                // Add store instruction
//                retval.push(BlockItem::Statement(
//                        Statement::Volatile(
//                            Instr::Store(
//                                var_type.clone(),
//                                init_value,
//                                val_ptr.clone(),
//                            )
//                        )
//                ));
//            }
//        }
//
//        retval
//    }
//    pub fn binary_expression_in_block(&mut self , node : AST_Expression  , type_ : AST_TypeAnnotation , lhs : Value) -> Vec<BlockItem> {
//
//        let mut retval : Vec<BlockItem> = Vec::new();
//        let postfix_vec : Vec<PostfixToken> = Vec::new();
//
//        fn traverse(node : &AST_Expression , postfix_vec: &mut Vec<PostfixToken>){
//            match node {
//                AST_Expression::Identifier(value) => {
//                    postfix_vec.push(PostfixToken::Identifier(value.clone()));
//                },
//                AST_Expression::IntegerLiteral(value) => {
//                    postfix_vec.push(PostfixToken::IntegerLit(value.clone()));
//                },
//                AST_Expression::DecimalLiteral(value) => {
//                    postfix_vec.push(PostfixToken::DecimalLit(value.clone()));
//                },
//                AST_Expression::CharacterLiteral(value) => {
//                    postfix_vec.push(PostfixToken::CharacterLit(value.clone()));
//                },
//                AST_Expression::StringLiteral(value) => {
//                    postfix_vec.push(PostfixToken::StringLit(value.clone()));
//                },
//                AST_Expression::BinaryExpression { operator, left, right } => {
//                    traverse(left, postfix_vec);
//                    traverse(right, postfix_vec);
//                    postfix_vec.push(PostfixToken::Operator(operator.clone()));
//                },
//                _ => panic!(" the case here should not happen , but has happened , in binary_expression_in_block"),
//            }
//        }
//
//        let mut counter = 0;
//        let mut tmp_counter = 0;
//        let mut operand_x : Value;
//
//        match &postfix_vec[counter ] {
//            PostfixToken::IntegerLit(val) => {
//                operand_x = Value::Const(val.clone() as u64);
//            },
//            PostfixToken::CharacterLit(val ) => {
//                operand_x = Value::Const(val.clone() as u64);
//            },
//            PostfixToken::DecimalLit(val) => {
//                operand_x = Value::Global(self.register_global_constant(format!("{}" , val), None));
//            },
//            PostfixToken::StringLit(val) => {
//                operand_x = Value::Global(self.register_global_constant(format!("{}" , val), Some(0)));
//            },
//            PostfixToken::Identifier(val) => {
//                operand_x = Value::Temporary(val.clone());
//            },
//            PostfixToken::Operator(val) => {
//                panic!("Encounterd Operator , needed a value");
//            }
//
//        }
//
//        if postfix_vec.len() == 1 {
//            retval.push(
//                BlockItem::Statement(
//                    Statement::Volatile(
//                        Instr::Store(
//                            self.extract_type(type_),
//                            lhs,
//                            operand_x)
//                    )
//                )
//            );
//
//            return retval;
//        }else{
//
//            counter += 1;    
//
//
//            while counter < postfix_vec.len(){
//
//                let tmp = format!("tmp_{}" , tmp_counter);
//                tmp_counter += 1;
//                let operand_y : Value;
//
//                match &postfix_vec[counter ] {
//                    PostfixToken::IntegerLit(val) => {
//                        operand_y = Value::Const(val.clone() as u64);
//                    },
//                    PostfixToken::CharacterLit(val ) => {
//                        operand_y = Value::Const(val.clone() as u64);
//                    },
//                    PostfixToken::DecimalLit(val) => {
//                        operand_y = Value::Global(self.register_global_constant(format!("{}" , val), None));
//                    },
//                    PostfixToken::StringLit(val) => {
//                        operand_y = Value::Global(self.register_global_constant(format!("{}" , val), Some(0)));
//                    },
//                    PostfixToken::Identifier(val) => {
//                        operand_y = Value::Temporary(val.clone());
//                    },
//                    PostfixToken::Operator(val) => {
//                        panic!("Encounterd Operator , needed a value");
//                    }
//
//                }
//
//                counter += 1;
//
//                match  &postfix_vec[counter] {
//                    PostfixToken::Operator(op ) => {
//                        match op {
//                            AST_Operator::ADD => {
//
//                                retval.push(
//                                    BlockItem::Statement(
//                                        Statement::Assign(
//                                            Value::Temporary(tmp.clone()),
//                                            self.extract_type(type_.clone()) , 
//                                            Instr::Add(operand_x , operand_y),
//                                        )
//                                    )
//                                );
//
//                                operand_x = Value::Temporary(tmp);
//                            },
//
//                            AST_Operator::SUB => {
//
//                                retval.push(
//                                    BlockItem::Statement(
//                                        Statement::Assign(
//                                            Value::Temporary(tmp.clone()),
//                                            self.extract_type(type_.clone()) , 
//                                            Instr::Sub(operand_x , operand_y),
//                                        )
//                                    )
//                                ); 
//
//                                operand_x = Value::Temporary(tmp);
//                            },
//
//                            AST_Operator::MUL => {
//
//                                retval.push(
//                                    BlockItem::Statement(
//                                        Statement::Assign(
//                                            Value::Temporary(tmp.clone()),
//                                            self.extract_type(type_.clone()) , 
//                                            Instr::Mul(operand_x , operand_y),
//                                        )
//                                    )
//                                );
//
//                                operand_x = Value::Temporary(tmp);
//                            },
//
//                            AST_Operator::DIV => {
//
//                                retval.push(
//                                    BlockItem::Statement(
//                                        Statement::Assign(
//                                            Value::Temporary(tmp.clone()),
//                                            self.extract_type(type_.clone()) , 
//                                            Instr::Div(operand_x , operand_y),
//                                        )
//                                    )
//                                ); 
//
//                                operand_x = Value::Temporary(tmp);
//                            },
//
//                            AST_Operator::OR => {
//
//                                retval.push(
//                                    BlockItem::Statement(
//                                        Statement::Assign(
//                                            Value::Temporary(tmp.clone()),
//                                            self.extract_type(type_.clone()) , 
//                                            Instr::Or(operand_x , operand_y),
//                                        )
//                                    )
//                                );
//
//                                operand_x = Value::Temporary(tmp);
//                            },
//
//                            AST_Operator::AND => {
//
//                                retval.push(
//                                    BlockItem::Statement(
//                                        Statement::Assign(
//                                            Value::Temporary(tmp.clone()),
//                                            self.extract_type(type_.clone()) , 
//                                            Instr::And(operand_x , operand_y),
//                                        )
//                                    )
//                                ); 
//
//                                operand_x = Value::Temporary(tmp);
//                            },
//
//                            _ => {
//                                panic!("These Instructions should not be handled by the binary_expression_in_block handler ");
//                            } 
//
//
//
//
//                        } // inner match
//
//
//                    }, 
//                    _ => {
//                        panic!("Encounterd value , needed an operator ");
//                    }
//                }// outer match
//
//                counter += 1;
//            }
//
//        }
//
//
//        return retval;
//    }
//
//    fn register_global_constant(&mut self , data : String , end : Option<u64>) -> String {
//        if let Some(e) = end {
//            let items = vec![
//                (Type::Byte , DataItem::Str(data)),
//                (Type::Byte , DataItem::Const(e)),
//            ];
//
//            let name = format!("global_constant_{}" , self.global_counter);
//            let new_data = DataDef::new(Linkage::private(), name.clone() ,  None , items);
//            self.global_counter += 1;
//            self.module.add_data(new_data);
//            return name;
//        }else{
//            let items = vec![
//                (Type::Byte , DataItem::Str(data)),
//            ];
//
//            let name = format!("global_constant_{}" , self.global_counter);
//            let new_data = DataDef::new(Linkage::private(),name.clone(), None , items);
//            self.global_counter += 1;
//            self.module.add_data(new_data);
//            return name;
//        }
//    }
//
//
//
//    fn extract_identifier(&mut self , identifier : AST_Expression) -> String {
//
//        if let AST_Expression::Identifier(name) = identifier {
//            return name;
//        }
//
//        todo!("The name of the function is not an identifier");
//    }
//
//
//    fn extract_type_allocation_size(&mut self , type_ : AST_TypeAnnotation) -> Instr {
//        match type_ {
//            AST_TypeAnnotation::I32 => return Instr::Alloc4(4),
//            AST_TypeAnnotation::U32 => return Instr::Alloc4(4),
//            AST_TypeAnnotation::I64 => return Instr::Alloc8(8),
//            AST_TypeAnnotation::U64 => return Instr::Alloc8(8),
//
//            AST_TypeAnnotation::F32 => return Instr::Alloc4(4),
//            AST_TypeAnnotation::F64 => return Instr::Alloc8(8),
//
//            AST_TypeAnnotation::STRING => return Instr::Alloc8(8),
//            AST_TypeAnnotation::VOID => return   Instr::Alloc8(8),
//            AST_TypeAnnotation::CHAR => return   Instr::Alloc4(1),
//            _ => todo!(" Get the size of the custom types as well ") ,
//        } 
//    }
//
//    fn extract_simple_type(&mut self , type_ : AST_TypeAnnotation) -> Option<Type> {
//        match type_ {
//            AST_TypeAnnotation::I32 => return Some(Type::Word),
//            AST_TypeAnnotation::U32 => return Some(Type::Word),
//            AST_TypeAnnotation::I64 => return Some(Type::Long),
//            AST_TypeAnnotation::U64 => return Some(Type::Long),
//
//            AST_TypeAnnotation::F32 => return Some(Type::Single),
//            AST_TypeAnnotation::F64 => return Some(Type::Double),
//
//            AST_TypeAnnotation::STRING => return Some(Type::Word),
//            AST_TypeAnnotation::VOID => return   Some(Type::Zero),
//            AST_TypeAnnotation::CHAR => return   Some(Type::Byte),
//            _ => return None ,
//        }
//    }
//
//    fn extract_custom_type(&mut self , type_ : AST_TypeAnnotation) -> Type {
//        
//        for i in self.type_cache.iter(){
//            if let AST_TypeAnnotation::CUSTOM(name ) = type_ {
//                if name == i.name {
//                    return Type::Aggregate(i);
//                }
//            } 
//        }
//
//        panic!("The Custom Type was not found in the Type Cache");
//
//    }
//
//    fn extract_type(&mut self , type_ : AST_TypeAnnotation) -> Type {
//
//        if let Some(typ) = self.extract_simple_type(type_.clone()){
//            return typ;
//        }else{
//            return self.extract_custom_type(type_);
//        }
//
//    }
//
//    fn extract_args(&mut self , args : Vec<AST_Statement>  ) -> Vec<(Type, Value)> {
//        
//        let mut retval : Vec<(Type , Value)>  = Vec::new();
//
//        for i in args {
//           if let AST_Statement::LET { name, type_annotation, initializer } = i {
//               retval.push((self.extract_type(type_annotation) , Value::Temporary(self.extract_identifier(name))));
//           } 
//        }
//
//        return retval;
//    }
//
//}
//
//




