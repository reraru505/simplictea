use crate::parser::ast::*;
use crate::parser::ast::AST_TypeAnnotation;
use crate::gen_ir::{TYPE_Handler, QBE_TYPES, STATEMENT};

impl QBE_TYPES {
    
    pub fn get_simple_typess(type_ : AST_TypeAnnotation) -> Option<Self> {

        match type_ {
            AST_TypeAnnotation::I32 => Some(QBE_TYPES::W),
            AST_TypeAnnotation::I64 => Some(QBE_TYPES::L),
            AST_TypeAnnotation::U32 => Some(QBE_TYPES::W),
            AST_TypeAnnotation::U64 => Some(QBE_TYPES::L),
            AST_TypeAnnotation::F32 => Some(QBE_TYPES::S),
            AST_TypeAnnotation::F64 => Some(QBE_TYPES::L),
            AST_TypeAnnotation::CHAR=> Some(QBE_TYPES::B),
            AST_TypeAnnotation::STRING => Some(QBE_TYPES::L),
            AST_TypeAnnotation::VOID => Some(QBE_TYPES::V),
            _ => return None,
        }
        
    }

    pub fn write(&self ) -> String {
        match self {
            QBE_TYPES::V => String::new(),
            QBE_TYPES::B => format!(" b "),
            QBE_TYPES::W => format!(" w "),
            QBE_TYPES::L => format!(" l "),
            QBE_TYPES::S => format!(" s "),
            QBE_TYPES::D => format!(" d "),
            QBE_TYPES::H => format!(" h "),
            QBE_TYPES::A(val) => format!(" {} " , val),
        }
    }
}

impl  TYPE_Handler{

    pub fn get_types(&self , typ : AST_TypeAnnotation) -> QBE_TYPES {

        if let Some(type_) = QBE_TYPES::get_simple_typess(typ.clone()) {
            return type_;
        }else if let AST_TypeAnnotation::CUSTOM(name) = typ {

            if self.lookup_name_exists(name.clone()) {
                return QBE_TYPES::A(format!(":{}",name));
            }else{
                panic!("The type is nor simple not aggregate");
            }
        }else{
            panic!("The type is nor simple nor aggregate");
        }  

    }

    
    pub fn get_types_size(&self , typ : AST_TypeAnnotation) -> String {

        if let Some(type_) = QBE_TYPES::get_simple_typess(typ.clone()) {
            match type_ {
                QBE_TYPES::B => format!("b"),
                QBE_TYPES::W => format!("w"),
                QBE_TYPES::L => format!("l"),
                QBE_TYPES::S => format!("s"),
                QBE_TYPES::D => format!("d"),
                QBE_TYPES::H => format!("h"),
                _ => panic!("should not happen anyway"),
            }
        }else if let AST_TypeAnnotation::CUSTOM(name) = typ {

            if self.lookup_name_exists(name.clone()) {
                todo!("handle custom type size ");
            }else{
                panic!("The type is nor simple not aggregate");
            }
        }else{
            panic!("The type is nor simple nor aggregate");
        }  

    }
 


}

impl STATEMENT {



}
