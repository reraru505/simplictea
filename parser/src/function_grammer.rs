use crate::function::FunctionDef;
use crate::lexer::token_type::{Data_Type, Keyword, Operator, Statement, Token, STC};
use crate::{
    expressions::Expression,
    function::{FunctionCall, FunctionDefType, FunctionRet},
    symboltable::symbol::{get_data_type_from_token, DataType},
};

