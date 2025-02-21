use crate::iterator::*;


impl Clone for IteratorType{

    fn clone( &self ) -> Self{

	match self {
	    IteratorType::IterateOverRange => return IteratorType::IterateOverRange,
	}
	
    }
}


impl Clone for Iterator{

    fn clone( &self ) -> Self {

	Self{
	    iter_type   : self.iter_type  .clone(), 
	    iter_value  : self.iter_value .clone(), 
	    super_scope : self.super_scope.clone(), 
	    range_start : self.range_start.clone(), 
	    range_end   : self.range_end  .clone(), 
	    iter_body   : self.iter_body  .clone(), 
	}
	
    }
    
}
