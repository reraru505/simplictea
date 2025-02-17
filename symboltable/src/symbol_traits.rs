use crate::symbol:: *;



impl Clone for  DataType{

    fn clone (&self ) -> Self{

	match self{
	    
	    DataType::I32 => DataType::I32 ,
	    DataType::I64 => DataType::I64,
	    DataType::F32 => DataType::F32,
	    DataType::F64 => DataType::F64,
	    DataType::CHAR => DataType::CHAR,
	    DataType::STRING => DataType::STRING,
	    DataType::VOID => DataType::VOID,

	}
	
    }
}


impl Clone for Qualifier{

    fn clone(&self ) -> Self {

	match self{
	    
	    Qualifier::CONSTANT => Qualifier::CONSTANT,
	    Qualifier::VARIABLE => Qualifier::VARIABLE,
	    Qualifier::FUNCTION => Qualifier::FUNCTION, // returns will be handled by C for now

	}
	
    }

}


