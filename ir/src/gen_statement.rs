use crate::parser::ast::AST_Expression;
use crate::parser::ast::AST_Statement;
use crate::parser::ast::AST_TypeAnnotation;
use crate::parser::ast::AST_Node;

use crate::ir::IrProgram;
use crate::ir::Instruction;
use std::collections::HashMap;


impl IrProgram {
    
    pub fn gen_function_statement(&mut self ,statement  : AST_Statement){

        let mut symtable : HashMap<String , AST_TypeAnnotation> = HashMap::new();        
        
        if let AST_Statement::FUNCTION { name, type_annotation, params, body } 
         = statement {
             
             if let Some(annot) = type_annotation{
                 symtable.insert(unwrap_expression(name), annot);
                 self.instructions.push(Instruction::Label(unwrap_expression(name)));

             }else{
                 symtable.insert(unwrap_expression(name), AST_TypeAnnotation::VOID);
                 self.instructions.push(Instruction::Label(unwrap_expression(name)));
             }
             
             if let Some(parameters) = params {
                 for i in parameters{

                     if let AST_Statement::LET { name, type_annotation, initializer } = i.clone(){
                        
                         symtable.insert(unwrap_expression(name), type_annotation);
                     }
                 }
             }

             for i in body{
                
                 if let AST_Node::statement( AST_Statement::LET { name, type_annotation, initializer }) = i.clone(){
                         symtable.insert(unwrap_expression(name), type_annotation);
                         if let Some(initializer) = initializer {

                         } 
                 }
             }
        }

        
    }

    
}




// helpers
impl IrProgram {


    fn gen_unique_label(){}
    
}

fn unwrap_expression(exp : AST_Expression) -> String {
    if let AST_Expression::Identifier(name ) = exp{
        return name;
    }

    return "".to_string();
}
