use crate::binaryexp::BinaryExpressionTree;
use crate::scope::Block;
use crate::symboltable::symbol::DataType;

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
}

#[derive(Debug)]
pub struct FunctionCall {
    pub fn_id: String,
    pub super_scope: String,
    pub fn_args: Vec<BinaryExpressionTree>,
}

#[derive(Debug)]
pub struct FunctionRet {
    pub super_scope: String,
    pub fn_ret: Vec<BinaryExpressionTree>,
}
