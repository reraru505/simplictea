use crate::binaryexp::BinaryExpressionTree;
use crate::scope::Block;
use crate::symboltable::symbol::DataType;
use crate::expressions::Expression;


#[derive(Debug)]
pub enum FunctionDefType {
    DEF_WITH_ARGS,
    DEF_NO_ARGS,
}

#[derive(Debug)]
pub struct FunctionDef {
    pub fn_id: String,
    pub super_scope: String,
    pub fn_type: FunctionDefType,
    pub fn_return_type: DataType,
    pub fn_body: Option<Block>,
    pub fn_args: Option<Vec<Expression>>,
}

impl FunctionDef{
    pub fn new() -> Self {
        Self {
            fn_id          : String::new(),
            super_scope    : String::new(),
            fn_type        : FunctionDefType::DEF_NO_ARGS ,
            fn_return_type : DataType::VOID,
            fn_body        : None,
            fn_args        : None,
        }
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub fn_id: String,
    pub super_scope: String,
    pub fn_args: Option<Vec<Expression>>,
}

impl FunctionCall {

    pub fn new() -> Self {
        Self {
            fn_id : String::new(),
            super_scope : String::new(),
            fn_args : None ,
        }
    }
}

#[derive(Debug)]
pub struct FunctionRet {
    pub super_scope: String,
    pub fn_ret: Vec<BinaryExpressionTree>,
}
