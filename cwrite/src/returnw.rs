use crate::parser::binaryexp::BinaryExpressionTree;
use crate::symboltable::symbol::DataType;
use crate::variablew::write_binary_expression_tree;
use crate::functionw::write_data_type;
use crate::lexer::token_type::{Literal , Token};

pub fn write_return(tree : BinaryExpressionTree , return_type : DataType , tabs : String) -> String {

    
    let mut retval : Vec<String>  = Vec::new();

    
    if tree.tree.len() > 1{
	retval.push(write_binary_expression_tree(tree.clone() , write_data_type(return_type) , tabs.clone()));

	if let Some(Token::t_identifier(last_tmp )) = tree.tree[tree.tree.len() - 1].exp_value.clone(){
	    retval.push(format!("{}\treturn {};\n\n" ,tabs ,last_tmp));
	}
	
    }else if tree.tree.len() > 0{
	if let Some(Token::t_identifier(last_tmp )) = tree.tree[0].exp_value.clone(){

	    retval.push(format!("{}\treturn {};\n\n" ,tabs , last_tmp));
	    
	}else if let Some(Token::t_literal(last_tmp )) = tree.tree[0].exp_value.clone(){

	    if let Literal::integer_literal(l) = last_tmp{
		retval.push(format!("{}\treturn {};\n\n" ,tabs , l));
	    }
	}
    }

    
    return retval.join("");

    
}

