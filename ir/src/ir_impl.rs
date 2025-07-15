use parser::ast::AST_Node;

use crate::ir::IrProgram;
use crate::ir::Operand;
use crate::parser::ast::AST_Statement;
use crate::parser::ast::AST_Expression;


impl IrProgram{
    pub fn new(inast : Vec<AST_Node>) ->  Self {
        return Self{
            ast : inast,
            
            instructions : Vec::new(),
            next_reg_id : 0,
        };
    }

    pub fn get_new_register(&mut self) -> Operand{

        let retval =  Operand::Register(self.next_reg_id);
        self.next_reg_id = self.next_reg_id + 1;
        return retval;
    }


}
