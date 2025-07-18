#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[derive(Debug , Clone)]
pub enum AST_Operator{
    ADD ,
    SUB ,
    MUL ,
    DIV ,

    ASSIGNMENT ,
    TYPEASSIGNMENT ,

    NOT ,
    CHECK_EQUAL ,
    CHECK_NOT_EQUAL ,
    CHECK_GREATER ,
    CHECK_LESSER ,

    AND ,
    OR ,
    XOR ,
}

#[derive(Debug , Clone)]
pub enum AST_TypeAnnotation{
    I32 ,
    I64 ,
    U32 ,
    U64 ,
    F32 ,
    F64 ,
    CHAR ,
    STRING ,
    VOID ,
    CUSTOM(String),
}

#[derive(Debug , Clone)]
pub enum AST_Expression {
    IntegerLiteral(i64),
    StringLiteral(String),
    DecimalLiteral(f64),
    CharacterLiteral(char),

    Identifier(String),

    BinaryExpression{
        operator : AST_Operator ,
        left : Box<AST_Expression>,
        right : Box<AST_Expression>,
    },
   
    Call{
        calee : Box<AST_Expression>,
        argumments : Option<Vec<AST_Expression>> ,
    },
    OBJECT_ITEM{
        object_item : Vec<AST_Expression>,
    }
}

#[derive(Debug, Clone)]
pub enum AST_ConditionalType{
    IF ,
    ELIF ,
    ELSE ,
}

#[derive(Debug , Clone)]
pub enum AST_IteratorType {
    RANGE_ITERATOR ,
    CONDITIONAL_ITERATOR ,
    UNCONDITIONAL_ITERATOR ,
}

#[derive(Debug, Clone)]
pub enum AST_Statement {


    ASSIGNMENT{
        LHS : AST_Expression,
        RHS : AST_Expression,
    },

    LET{
        name : AST_Expression, 
        type_annotation : AST_TypeAnnotation,
        initializer : Option<AST_Expression>,
    },

    FUNCTION {
        name : AST_Expression,
        type_annotation : Option<AST_TypeAnnotation>,
        params : Option<Vec<AST_Statement>>,
        body : Vec<AST_Node>,
    },

    CONDITIONAL {
        conditional_type : AST_ConditionalType ,
        condition : Option<AST_Expression >,
        body : Vec<AST_Node>,
    },

    ITERATOR {
        iterator_type : AST_IteratorType ,
        condition : Option<AST_Expression> ,
        body : Vec<AST_Node> ,
    },

    STRUCTURE {
        name : AST_Expression ,
        body : Vec<AST_Node> , 
    },

    RETURN(AST_Expression),
}


#[derive(Debug , Clone)]
pub enum AST_Node{
    statement(AST_Statement),
    expression(AST_Expression),
    error(String),
}









