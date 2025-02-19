#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::token_type::*;

impl PartialEq for Literal {

    fn eq(&self , other : &Self) -> bool{

	match (self , other){
	    (Literal::character_literal(a) ,Literal::character_literal(b) ) => return true ,
	    (Literal::string_literal(a) ,Literal::string_literal(b) ) => return true ,
	    (Literal::integer_literal(a) ,Literal::integer_literal(b) ) => return true ,
	    (Literal::decimal_literal(a) ,Literal::decimal_literal(b) ) => return true ,
	    _ => return false,
	}
	
    }
}

impl Clone for Literal {

    fn clone(&self ) -> Self{

	match self {
	    Literal::character_literal(a)  => Literal::character_literal(a.clone())  ,
	    Literal::string_literal(a)     => Literal::string_literal(a.clone())     ,
	    Literal::integer_literal(a)    => Literal::integer_literal(a.clone())    ,
	    Literal::decimal_literal(a)    => Literal::decimal_literal(a.clone())    ,
	}
	
    }
}



impl PartialEq for Data_Type {

    fn eq(&self , other : &Self ) -> bool {

	match (self , other ){
	    
	    (Data_Type::CHAR(a) ,   Data_Type::CHAR(b) ) => return true,
	    (Data_Type::VOID(a) ,   Data_Type::VOID(b) ) => return true,
	    (Data_Type::I32(a) ,    Data_Type::I32(b) ) => return true,
	    (Data_Type::I64(a) ,    Data_Type::I64(b) ) => return true,
	    (Data_Type::F32(a) ,    Data_Type::F32(b) ) => return true,
	    (Data_Type::F64(a) ,    Data_Type::F64(b) ) => return true,
	    (Data_Type::STRING(a) , Data_Type::STRING(b) ) => return true,
	    _ => return false,
	}
    }
    
}



impl Clone for Data_Type {

    fn clone(&self ) -> Self {

	match self{
	    
	    Data_Type::CHAR(a)   =>   Data_Type::CHAR(a.clone()) ,
	    Data_Type::VOID(a)   =>   Data_Type::VOID(a.clone()) ,
	    Data_Type::I32(a)    =>    Data_Type::I32(a.clone()) ,
	    Data_Type::I64(a)    =>    Data_Type::I64(a.clone()) ,
	    Data_Type::F32(a)    =>    Data_Type::F32(a.clone()) ,
	    Data_Type::F64(a)    =>    Data_Type::F64(a.clone()) ,
	    Data_Type::STRING(a) => Data_Type::STRING(a.clone()) ,
	}
    }
    
}



impl PartialEq for Statement{

    fn eq(&self , other : &Self) -> bool{

	match (self , other) {

	    (Statement::function_marker(a) , Statement::function_marker(b) ) => return true,
	    (Statement::if_statement(a) , Statement::if_statement(b) ) => return true,
	    (Statement::else_statement(a) , Statement::else_statement(b) ) => return true,
	    (Statement::return_statement(a) , Statement::return_statement(b) ) => return true,
	    (Statement::for_statement(a) , Statement::for_statement(b) ) => return true,
	    _ => return false,
	    
	}
	
    }
}



impl Clone for Statement{

    fn clone(&self) -> Self{

	match self {

	    Statement::function_marker(a)  => Statement::function_marker(a.clone())  ,
	    Statement::if_statement(a)     => Statement::if_statement(a.clone())     ,
	    Statement::else_statement(a)   => Statement::else_statement(a.clone())   ,
	    Statement::return_statement(a) => Statement::return_statement(a.clone()) ,
	    Statement::for_statement(a)    => Statement::for_statement(a.clone())    ,
	}
	
    }
}


impl PartialEq for Keyword{

    fn eq(&self , other : &Self) -> bool{

	match(self , other ) {

	    (Keyword::data_type(a) , Keyword::data_type(b)) => return true,
	    (Keyword::statement(a) , Keyword::statement(b)) => return true,
	    _ => false,
	    
	}
    }
    
}



impl Clone for Keyword{

    fn clone(&self) -> Self{

	match self {

	    Keyword::data_type(a) => Keyword::data_type(a.clone()),
	    Keyword::statement(a) => Keyword::statement(a.clone()),
	    
	}
    }
    
}


impl PartialEq for STC{

    fn eq(&self , other : &Self) -> bool{

	match (self , other ){
	    (STC::stc_scope_begin(a),STC::stc_scope_begin(b)) => return true,
	    (STC::stc_scope_end(a),STC::stc_scope_end(b)) => return true,
	    (STC::stc_comma_seperator(a) , STC::stc_comma_seperator(b)) => return true,
	    (STC::stc_end_expression(a),STC::stc_end_expression(b)) => return true,
	    (STC::stc_arg_begin(a),STC::stc_arg_begin(b)) => return true,
	    (STC::stc_arg_end(a),STC::stc_arg_end(b)) => return true,
	     (STC::stc_dot(a),STC::stc_dot(b)) => return true,
	    _ => return false,
	}
    }
}

impl Clone for STC{

    fn clone(&self) -> Self{

	match self {
	    STC::stc_scope_begin(a)    => STC::stc_scope_begin(a.clone())   ,
	    STC::stc_scope_end(a)      => STC::stc_scope_end(a.clone())     ,
	    STC::stc_comma_seperator(a) => STC::stc_comma_seperator(a.clone()) , 
	    STC::stc_end_expression(a) => STC::stc_end_expression(a.clone()),
	    STC::stc_arg_begin(a)      => STC::stc_arg_begin(a.clone())     ,
	    STC::stc_arg_end(a)        => STC::stc_arg_end(a.clone())       ,
	    STC::stc_dot(a)        => STC::stc_dot(a.clone())       ,

	}
    }
}



impl PartialEq for Operator{

    fn eq(&self , other : &Self) -> bool{

	match (self , other ){
	    (Operator::assignment_op(a),Operator::assignment_op(b)) => return true,
	    (Operator::type_assignment_op(a),Operator::type_assignment_op(b)) => return true,
	    (Operator::addition_op(a),Operator::addition_op(b)) => return true,
	    (Operator::subtraction_op(a),Operator::subtraction_op(b)) => return true,
	    (Operator::multiplication_op(a),Operator::multiplication_op(b)) => return true,
	    (Operator::division_op(a),Operator::division_op(b)) => return true,
	    _ => return false,
	}
    }
}


impl Clone for Operator{

    fn clone(&self) -> Self{

	match self{
	    Operator::assignment_op(a)      =>  Operator::assignment_op(a.clone())    ,
	    Operator::type_assignment_op(a)      =>  Operator::type_assignment_op(a.clone())    ,
	    Operator::addition_op(a)        =>  Operator::addition_op(a.clone())      ,
	    Operator::subtraction_op(a)     =>  Operator::subtraction_op(a.clone())   ,
	    Operator::multiplication_op(a)  =>  Operator::multiplication_op(a.clone()),
	    Operator::division_op(a)        =>  Operator::division_op(a.clone())      ,
	}
    }
}



impl PartialEq for  Token{

    fn eq(&self , other : &Self) -> bool{

	match(self , other  ){

	    (Token::t_keyword(a) , Token::t_keyword(b)) => return true,
	    (Token::t_literal(a) , Token::t_literal(b)) => return true,
	    (Token::t_stc(a) , Token::t_stc(b)) => return true,
	    (Token::t_operator(a) , Token::t_operator(b)) => return true,
	    (Token::t_identifier(a) , Token::t_identifier(b)) => return true,
	    _ => return false,
	}
	
    }

}

impl Clone for  Token{

    fn clone(&self) -> Self{

	match self {

	    Token::t_keyword(a)    =>  Token::t_keyword(a.clone())    ,
	    Token::t_literal(a)    =>  Token::t_literal(a.clone())    ,
	    Token::t_stc(a)        =>  Token::t_stc(a.clone())        ,
	    Token::t_operator(a)   =>  Token::t_operator(a.clone())   ,
	    Token::t_identifier(a) =>  Token::t_identifier(a.clone()) ,
	}
	
    }

}


impl Clone for Position {

    fn clone(&self) -> Self {
	Self{
	    x : self.x,
	    y : self.y,
	}
    }
}
