
use crate::binaryexp::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use crate::lexer::token_type::*;
use crate::binaryexp_helpers::*;

pub fn break_binary_expression(context : &mut Vec<Token> , scope : &str , tmp_count : &mut usize  ) -> BinaryExpressionTree{

    let mut new_context = remove_brackets_from_single_token_inside_brackets(Rc::new(RefCell::new(context.clone())));
    
//    for i in new_context.clone().iter(){
//	println!("{:?}" , i);
//    }
    
    
    let mut tree_maker : Vec<Option<Token>> = Vec::new();
    

    
    handle_binary_expression(&mut new_context ,&mut tree_maker , scope , tmp_count );

 //   for i in tree_maker.iter(){
 //	println!("{:?}" , i);
 //   }
    
    let mut tree = BinaryExpressionTree::new();
    tree.maketree(tree_maker);
    print_binary_expression_tree_debug(tree.clone() );

    return tree;
    
}

pub fn handle_binary_expression(context : &mut Vec<Token> ,
				tree_maker : &mut Vec<Option<Token>> ,
				scope : &str ,
				tmp_count : &mut usize){

    let mut newcon = context.clone();
    let mut conholder  : Vec<Vec<Token>> = Vec::new();
    
    let mut precedence = BinaryExpressionType::get_highest_precedence(newcon.clone());
    let mut index = 0;
    let mut len = context.len();
    let mut tmp_counter = *tmp_count;
    
    while precedence > 1{

	if precedence == 4 {
	    
	    conholder.push(newcon.clone());
	    let mut replacer =  handle_inside_brackets(newcon.clone() );
	    
	  //  println!("\nreplacer = ");
	  //  for i in replacer.clone(){
	  //	println!("{:?}", i);
	  //  }

	
	    
	    len = replacer.len();
	    index = 0;
	    precedence = BinaryExpressionType::get_highest_precedence(replacer.clone());
	    newcon = replacer;
	    
	}else{
	    println!(" precedence = {}" ,precedence);
	    while index < len {

		if BinaryExpressionType::is_binary_operator(newcon[index].clone()) &&
		    BinaryExpressionType::check_precedence(newcon[index].clone()) == precedence {

		//	println!("\nncon = ");
		//	for i in newcon.clone(){
		//	    println!("{:?}", i);
		//	}

			tree_maker.push(Some(Token::t_identifier(format!("{}_temp_{}", scope , tmp_counter))));
			tree_maker.push(Some(newcon[index].clone()));
			tree_maker.push(Some(newcon[index - 1].clone()));
			tree_maker.push(Some(newcon[index + 1].clone()));

		//	println!("\nPushed\n");
			let mut replacer = newcon.clone();
			replacer[index] = Token::t_identifier(format!("{}_temp_{}", scope , tmp_counter));
			replacer.remove(index + 1);
			replacer.remove(index - 1);

			newcon = replacer;
			len = newcon.len().clone();

			if len > 2{
		
			    tmp_counter += 1;
			    index = 0;
			    
			}else {
			    if conholder.len() > 0{
				
				newcon = handle_outside_brackets(conholder.pop().unwrap() ,
							     Token::t_identifier(format!("{}_temp_{}", scope , tmp_counter)));
				len = newcon.len().clone();
				tmp_counter += 1;
				index = 0;
			    }
			}

			precedence = BinaryExpressionType::get_highest_precedence(newcon.clone());
			break;
		    }
		index += 1;
	    }

	    	    
	}

	
    }

    for i in newcon.clone(){
    	println!("{:?}", i);
    }
    precedence = BinaryExpressionType::get_highest_precedence(newcon.clone());
    println!("{}",precedence);
    if precedence == 1{
    	tree_maker.push(Some(newcon[0].clone()));
	tree_maker.push(None);
	tree_maker.push(Some(newcon[2 ].clone()));
	tree_maker.push(None);
	//tree_maker.push(None);
    }else if precedence == 0{
	tree_maker.push(Some(newcon[0].clone()));
	tree_maker.push(None);
	tree_maker.push(None);
	tree_maker.push(None);
    }
    
    *tmp_count = tmp_counter;
    
}
