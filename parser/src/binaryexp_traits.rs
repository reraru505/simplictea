use std::fmt;
use crate::binaryexp::*;


impl fmt::Display for BinaryExpressionType{

    fn fmt(&self ,  f: &mut fmt::Formatter<'_>) -> fmt::Result{
	match self{
	    BinaryExpressionType::Assignment_op => write!(f, " = ", ),
	    BinaryExpressionType::Addition_op => write!(f, " + ", ),
	    BinaryExpressionType::Subtraction_op => write!(f, " - ", ),
	    BinaryExpressionType::Multiplication_op => write!(f, " * ", ),
	    BinaryExpressionType::Division_op => write!(f, " / ", ),
	}
    }
    
}

impl Clone for BinaryExpressionType{
    fn clone(&self ) -> Self{
	match self{
	    BinaryExpressionType::Assignment_op => BinaryExpressionType::Assignment_op,
	    BinaryExpressionType::Addition_op => BinaryExpressionType::Addition_op,
	    BinaryExpressionType::Subtraction_op => BinaryExpressionType::Subtraction_op,
	    BinaryExpressionType::Multiplication_op => BinaryExpressionType::Multiplication_op,
	    BinaryExpressionType::Division_op => BinaryExpressionType::Division_op,
	}
    }
}

impl Clone for BinaryExpression {

    fn clone(&self ) ->Self{
	Self{
	    exp_value : self.exp_value.clone(),
	    exp_type :  self.exp_type.clone(),
	    exp_left :  self.exp_left.clone(),
	    exp_right : self.exp_right.clone(),
	}
    }
    
}

impl Clone for BinaryExpressionTree{
    fn clone(&self ) -> Self{
	Self {
	    tree : self.tree.clone(),
	}
    }
}


impl Clone for  BinaryExpressionBlock {
    
    fn clone (&self ) -> Self {
	Self {
	    b_tree : self.b_tree.clone(),
	    b_type : self.b_type.clone(),
	    b_qualifier : self.b_qualifier.clone(),
	}
    }
}



