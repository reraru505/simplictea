use crate::function::*;


impl Clone for FunctionDefType{
    fn clone(&self ) -> Self {
	match self {
	    FunctionDefType::DEF_WITH_ARGS =>  FunctionDefType::DEF_WITH_ARGS,          
	    
	    FunctionDefType::DEF_NO_ARGS =>  FunctionDefType::DEF_NO_ARGS,            
	    
	}
    }
}

impl Clone for FunctionDef{

    fn clone(&self ) -> Self {

	Self {
	    super_scope : self.super_scope.clone(),
	    fn_id : self.fn_id.clone(),
	    fn_type : self.fn_type.clone(),
	    fn_return_type : self.fn_return_type.clone(),
	    fn_args : self.fn_args.clone(),
	    fn_body : self.fn_body.clone(),
	}
	
    }
    
}
