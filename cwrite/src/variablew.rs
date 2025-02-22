use crate::parser::variable::Variable;
use crate::parser::binaryexp::BinaryExpressionTree;
use crate::lexer::token_type::{Token , Literal };
use crate::functionw::write_data_type;


// shit optimization  , please dont forget to optimize

pub fn write_variable(var : Variable , tabs : String  ) -> String {

    let mut retval : Vec<String>  = Vec::new();

    if var.clone().var_value.unwrap().tree.len() > 1{
	retval.push(write_binary_expression_tree(var.clone().var_value.unwrap() ,
						 write_data_type(var.clone().var_type),
						 tabs.clone()));

	if let Some(Token::t_identifier(last_tmp )) = var.clone().var_value.unwrap().tree[
	    var.clone().var_value.clone().unwrap().tree.len().clone() - 1]
	    .exp_value.clone(){
		retval.push(format!("{}\t{} {} = {};\n\n",tabs ,write_data_type(var.clone().var_type) , var.clone().var_id , last_tmp));
	}
	
    }else if var.clone().var_value.unwrap().tree.len() > 0{
	
	if let Some(Token::t_identifier(last_tmp )) = var.clone().var_value.unwrap().tree[0].exp_value.clone(){
	    retval.push(format!("{}\t{} {} = {};\n\n" ,tabs , write_data_type(var.clone().var_type) , var.var_id , last_tmp));
	}else if let Some(Token::t_literal(last_tmp )) = var.clone().var_value.unwrap().tree[0].exp_value.clone(){
	    if let Literal::integer_literal(l) = last_tmp{
		retval.push(format!("{}\t{} {} = {};\n\n",tabs ,write_data_type(var.clone().var_type) , var.var_id , l));
	    }
	    else if let Literal::decimal_literal(l) = last_tmp{
		retval.push(format!("{}\t{} {} = {};\n\n",tabs ,write_data_type(var.clone().var_type) , var.var_id , l));
	    }
	}
    }


    return retval.join("");
    
}

pub fn write_binary_expression_tree(tree : BinaryExpressionTree , var_type : String , tabs : String ) -> String{
        
    let mut retval : Vec<String>  = Vec::new();
    
    let mut printName = String::new();
    let mut printRight = String::new();
    let mut printLeft = String::new();
    let mut printOp = String::new();

    
    for i in tree.tree[0 .. tree.tree.len() - 1].to_vec().iter(){
	if let Some(Token::t_identifier(s)) = i.exp_value.clone(){
	    printName = s;
	}else{
	    printName.clear();
	}
	if let Some(Token::t_identifier(s)) = i.exp_left.clone(){
	    printLeft = s;
	}else if let Some(Token::t_literal(Literal::integer_literal(s))) = i.exp_left.clone(){
	    printLeft = s;
	}else {
	    printLeft.clear();
	}
	if let Some(Token::t_identifier(s)) = i.exp_right.clone(){
	    printRight = s;
	}else if let Some(Token::t_literal(Literal::integer_literal(s))) = i.exp_right.clone(){
	    printRight = s;
	}else{
	    printRight.clear();
	}if let Some(b) = i.exp_type.clone(){
	    printOp = format!("{}",b);
	}else{
	    printOp.clear();
	}
	retval.push(format!("{}\t{} {} = {} {} {} ;\n" , tabs ,var_type , printName , printLeft , printOp , printRight));
    }
   
    return retval.join("");
}
