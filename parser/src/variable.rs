
use crate::lexer::token_type::{Token , Operator , Keyword , Data_Type , STC};
use crate::symboltable::symbol::{DataType , Qualifier};
use crate::lexer::lex::Lexeme;
use crate::binaryexp::BinaryExpressionTree;
use crate::parsingdata::ParsingData;
use crate::binaryexp_handle::break_binary_expression;

#[derive(Debug)]
pub struct Variable{
    pub var_id : String,
    pub var_type : DataType,
    pub var_qualifier : Qualifier,
    pub var_value : Option<BinaryExpressionTree>, 
}

impl Clone for Variable{
    fn clone(&self) ->Self {
	Self{
	    var_id : self.var_id.clone(),
	    var_type : self.var_type.clone(),
	    var_qualifier : self.var_qualifier.clone(),
	    var_value : self.var_value.clone(), 
	}

    }
}

impl Variable{

    pub fn new(v_id : String , v_type : DataType , v_qualifier : Qualifier , v_value : Option<BinaryExpressionTree>) -> Self{

	Self {
	    var_id : v_id,
	    var_type : v_type,
	    var_qualifier : v_qualifier,
	    var_value : v_value, 
	}
    }
    
}


pub fn get_variable_def_from_args(args : Vec<Lexeme>) ->  Vec<Variable>{

    let len = args.len();
    let mut retval : Vec<Variable> = Vec::new();
    let mut context : Vec<Lexeme> = Vec::new();

    if len == 3 {
	context = args.clone();
	if matches!(context[0].tokens , Token::t_identifier(_)) &&
	    matches!(context[1].tokens , Token::t_operator(Operator::type_assignment_op(_))) &&
	    matches!(context[2].tokens , Token::t_keyword(Keyword::data_type(_))){
		
		if let Token::t_identifier(name ) = context[0].clone().tokens{
		    retval.push(Variable::new(name ,
					      get_variable_data_type(context[2].clone().tokens) ,
					      Qualifier::VARIABLE ,
					      None) );
		}
	}

	return retval;
    }
    
    for i in 0 .. len {
	
	if i > 2{
	    context = args[i-3 .. i].to_vec();
	}
	if context.len() > 2 &&
	    matches!(context[0].tokens , Token::t_identifier(_)) &&
	    matches!(context[1].tokens , Token::t_operator(Operator::type_assignment_op(_))) &&
	    matches!(context[2].tokens , Token::t_keyword(Keyword::data_type(_))){

		if let Token::t_identifier(name ) = context[0].clone().tokens{
		    retval.push(Variable::new(name ,
					      get_variable_data_type(context[2].clone().tokens) ,
					      Qualifier::VARIABLE ,
					      None) );
		}
	}
    }
    
    return retval;
}


pub fn get_variable_data_type(token : Token) -> DataType{

    match token {
	    
	Token::t_keyword(Keyword::data_type(Data_Type::CHAR(_))) => return DataType::CHAR,
	Token::t_keyword(Keyword::data_type(Data_Type::I32(_))) => return DataType::I32,
	Token::t_keyword(Keyword::data_type(Data_Type::I64(_))) => return DataType::I64,
	Token::t_keyword(Keyword::data_type(Data_Type::F32(_))) => return DataType::F32,
	Token::t_keyword(Keyword::data_type(Data_Type::F64(_))) => return DataType::F64,
	Token::t_keyword(Keyword::data_type(Data_Type::STRING(_)))  => return DataType::STRING,
	Token::t_keyword(Keyword::data_type(Data_Type::VOID(_))) => return DataType::VOID,
	_ => return DataType::VOID, //this case will not be possible but the rust compiler is a bitch
    }
    
}


pub fn find_variable_declarations_in_scope(parsingvec : Vec<ParsingData> , scope : String , tmp_count : &mut usize ) -> Vec<ParsingData>{

    let len = parsingvec.len();
    let mut context : Vec<ParsingData> = Vec::new();
    let mut retval : Vec<ParsingData> = Vec::new();
    

    let mut i  = 0;
    while i < len {

	println!("i is {}", i);
	if i < len - 2{
	    context = parsingvec[i .. i + 3].to_vec();
	}else {
	    retval.push(parsingvec[i].clone());
	}
	 	

	if context.len() >  2 {

	    let mut is_var_def = false;
	    
	    if let ParsingData::lexeme(l) = context[0].clone(){
		if matches!(l.tokens , Token::t_identifier(_)){
		    is_var_def = true;
		    println!("\nONE\n");
		}else{
		    is_var_def = false;
		}
	    }
	    if let ParsingData::lexeme(l) = context[1].clone(){
		if matches!(l.tokens , Token::t_operator(Operator::type_assignment_op(_))){
		    is_var_def = true;
		    println!("\nTWO\n");
		}else{
		    is_var_def = false;
		}
	    }
	    
	    if let ParsingData::lexeme(l) = context[2].clone(){
		 if matches!(l.tokens , Token::t_keyword(Keyword::data_type(_))){
		     is_var_def = true;
		     println!("\nTHREE\n");
		 }else{
		     is_var_def = false;
		 }
	    }
	    
	    if is_var_def == true {

		let mut binexp : Vec<Token> = Vec::new();
		i += 4;
		
		loop {

		    if let ParsingData::lexeme(l) = parsingvec[i].clone(){
			if matches!(l.tokens , Token::t_stc(STC::stc_end_expression(_))){
			    binexp.push(l.tokens.clone());
			    break;
			}else {
			    binexp.push(l.tokens.clone());
			}
		    }
		    i += 1;
		}
		println!("\n i was set to {} ", i);
		
		let mut name = String::new();
		let mut vtype = DataType::VOID;

		if let ParsingData::lexeme(l) = context[0].clone(){
		    if let Token::t_identifier(n) = l.tokens{
			name = n;
		    }
		}
		
		if let ParsingData::lexeme(l) = context[2].clone(){
		    vtype = get_variable_data_type(l.tokens);
		}
		
		retval.push(ParsingData::variable(Variable::new(name ,
								vtype ,
								Qualifier::VARIABLE ,
								Some(break_binary_expression(&mut binexp ,
											     &scope ,
											     tmp_count)))));
		*tmp_count += 1;
		context.clear();
		
		//i += 1;
	    }else{
		
		
		println!("else met ");
		retval.push(parsingvec[i].clone());
		retval.push(parsingvec[i+1].clone());
		retval.push(parsingvec[i+2].clone());
		i += 2;
		
		context.clear();
	    }

	    

	}
	i += 1;
	
    }

//    for i in context.iter(){
//	retval.push(i.clone());
//    }
    
    return retval;
}
