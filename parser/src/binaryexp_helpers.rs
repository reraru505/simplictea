
use crate::binaryexp::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::lexer::token_type::*;


pub fn handle_outside_brackets(context : Vec<Token> ,replacer : Token) -> Vec<Token>{

    let mut ncon = context.clone();
    let mut start = 0;
    
    for (index , i) in ncon.iter().enumerate() {
	if matches!(i , Token::t_stc(STC::stc_arg_begin(_))){
	    start = index ;
	    break;
	} 
    }

    let mut inner = 0;
    let mut end = 0;
    let mut index = start + 1;
    let len = ncon.len().clone();
    
    while index < len {
	
	if matches!(ncon[index] , Token::t_stc(STC::stc_arg_begin(_))){
	    inner += 1;
	}else if matches!(ncon[index] , Token::t_stc(STC::stc_arg_end(_))){
	    if inner == 0{
		end = index;
		break;
	    }else{
		inner -= 1;
	    }
	   
	}
	index +=1 ;
    }

    ncon[start] = replacer;
    ncon.drain(start+1 .. end + 1);

   // for i in ncon.clone(){
   // 	println!("outer = {:?}", i);
   // }

    println!("\n");
    
    return ncon;
    
}


pub fn handle_inside_brackets(context : Vec<Token> ) -> Vec<Token>{

    
    let mut ncon = context.clone();
    let mut start = 0;
    
    for (index , i) in ncon.iter().enumerate() {
	if matches!(i , Token::t_stc(STC::stc_arg_begin(_))){
	    start = index ;
	    break;
	} 
    }

    let mut inner = 0;
    let mut end = 0;
    let mut index = start + 1;
    let len = ncon.len().clone();
    
    while index < len {
	
	if matches!(ncon[index] , Token::t_stc(STC::stc_arg_begin(_))){
	    inner += 1;
	}else if matches!(ncon[index] , Token::t_stc(STC::stc_arg_end(_))){
	    if inner == 0{
		end = index;
		break;
	    }else{
		inner -= 1;
	    }
	}
	index +=1 ;
    }

    let retval = ncon[start + 1 .. end ].to_vec();

  //  for i in retval.clone(){
  //  	println!("inner = {:?}", i);
  //  }


    return retval;
    
}



pub fn remove_brackets_from_single_token_inside_brackets(context : Rc<RefCell<Vec<Token>>>) ->Vec<Token>{

    let new_context = context.borrow().clone();
    let mut replacer : Vec<Token> = Vec::new();
    let len = new_context.len();
    let mut index = 0;
    println!("\n\nskip_single_token_inside_brackets Starts\n\n");

    while index < len {

	if index < (len - 2) &&
	    matches!(new_context[index] , Token::t_stc(STC::stc_arg_begin(_))) &&
	    matches!(new_context[index + 2] ,Token::t_stc(STC::stc_arg_end(_))){
		index += 1;
		replacer.push(new_context[index].clone());
		index += 2;
	}else {
		
		replacer.push(new_context[index].clone());
		index += 1;
	    }	
    }
    println!("\n\nskip_single_token_inside_brackets Ends\n\n");
    return replacer;
    
}

pub fn print_binary_expression_tree_debug(tree : BinaryExpressionTree){
    
    let mut printName = String::new();
    let mut printRight = String::new();
    let mut printLeft = String::new();
    let mut printOp = String::new();
    for i in tree.tree.iter(){
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
	println!("let {} = {} {} {} ;" , printName , printLeft , printOp , printRight);
    }

}
