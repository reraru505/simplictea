
use crate::lexer::token_type::*;
use crate::binaryexp::*;

impl BinaryExpressionType{
    pub fn get_highest_precedence(context : Vec<Token>) -> usize {

	let mut precedence : usize  = 0;

	for i in context.iter(){
	    match i {
		Token::t_operator(Operator::addition_op(s)) | Token::t_operator(Operator::subtraction_op(s)) =>  {
		    precedence = precedence.max(2);
		} ,
		Token::t_operator(Operator::multiplication_op(s)) | Token::t_operator(Operator::division_op(s)) =>  {
		    precedence = precedence.max(3);
		},
		Token::t_stc(STC::stc_arg_begin(s)) =>{
		    precedence = precedence.max(4);
		},
		Token::t_operator(Operator::assignment_op(s)) =>{
		    precedence = precedence.max(1);
		},
		_ => {
		    precedence = precedence.max(0);
		},
	    };
	}

	return precedence;
    }

    pub fn check_precedence(token :Token) -> usize{
	match token{
	    Token::t_operator(Operator::addition_op(s)) | Token::t_operator(Operator::subtraction_op(s)) => return 2,
	    Token::t_operator(Operator::multiplication_op(s)) | Token::t_operator(Operator::division_op(s)) => return 3,
	    Token::t_operator(Operator::assignment_op(s)) => return 1,
	    _ => return 0,
	}
    }

    pub fn get_binary_expression_type(token :Option<Token>) -> BinaryExpressionType{
	
	match token{
	    Some(Token::t_operator(Operator::assignment_op(_))) => return BinaryExpressionType::Assignment_op,
	    Some(Token::t_operator(Operator::addition_op(_))) => return BinaryExpressionType::Addition_op,
	    Some(Token::t_operator(Operator::subtraction_op(_))) => return BinaryExpressionType::Subtraction_op,
	    Some(Token::t_operator(Operator::division_op(_))) => return BinaryExpressionType::Division_op,
	    Some(Token::t_operator(Operator::multiplication_op(_))) => return BinaryExpressionType::Multiplication_op,
	    _ => return BinaryExpressionType::Assignment_op,
	}
	
    }

    pub fn is_binary_operator(token : Token ) -> bool{
	
	match token{
	    Token::t_operator(Operator::assignment_op(_)) => return true,
	    Token::t_operator(Operator::addition_op(_)) => return true,
	    Token::t_operator(Operator::subtraction_op(_)) => return true,
	    Token::t_operator(Operator::division_op(_)) => return true,
	    Token::t_operator(Operator::multiplication_op(_)) => return true,
	    _ => return false,
	}
	
    }
}

impl BinaryExpression{

    pub fn new() -> Self {
	Self{
	    
	    exp_value : None,
	    exp_type :  None,
	    exp_left :  None,
	    exp_right : None,
	}
    }
}

impl BinaryExpressionTree{
    pub fn new() -> Self{
	Self{
	    tree : Vec::new(),
	}
    }

    pub fn maketree(&mut self , tree_maker : Vec<Option<Token>> ){

	let len = tree_maker.len();
	let mut index : usize  = 0;
	while index < len{

	    self.tree.push(BinaryExpression{
		exp_value : tree_maker[index].clone(),
		exp_type :  {if let Some(_) = tree_maker[index + 1].clone() {
		    Some(BinaryExpressionType::get_binary_expression_type(tree_maker[index + 1].clone()))
		}else {
		    None
		}},
		exp_left :  tree_maker[index + 2].clone(),
		exp_right :  tree_maker[index + 3].clone(),}
	    );

	    index += 4;
	}
	
    }

}

