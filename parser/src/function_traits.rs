use crate::function::{FunctionCall, FunctionDef, FunctionRet};

impl Clone for FunctionDef {
    fn clone(&self) -> Self {
        Self {
            fn_id: self.fn_id.clone(),
            super_scope: self.super_scope.clone(),
            fn_type: self.fn_type.clone(),
            fn_return_type: self.fn_return_type.clone(),
            fn_body: self.fn_body.clone(),
        }
    }
}

impl Clone for FunctionCall {
    fn clone(&self) -> Self {
        Self {
            fn_id: self.fn_id.clone(),
            super_scope: self.super_scope.clone(),
            fn_args: self.fn_args.clone(),
        }
    }
}

impl Clone for FunctionRet {
    fn clone(&self) -> Self {
        Self {
            super_scope: self.super_scope.clone(),
            fn_ret: self.fn_ret.clone(),
        }
    }
}
