#![allow(non_snake_case)]
use crate::siparser::ast::{Node, Data_Type};
use std::collections::{HashMap, HashSet};

#[derive(Clone , Debug)]
pub struct Item{
   pub id : String,  
   pub d_type : Data_Type,
}


impl Item{
    pub fn new(inid : String , in_type : Data_Type) -> Self {
        Self { id: inid, d_type: in_type }
    }
}

#[derive(Clone , Debug)]
pub struct Function{
    pub id : String ,
    pub d_type : Data_Type,
    pub params : Vec<Item>,
}

impl Function{
    pub fn new(inid : String , in_type : Data_Type , inparams : Vec<Item>) -> Self{
        Self { id: inid, d_type: in_type, params: inparams }
    }
}

#[derive(Clone , Debug)]
pub struct SiTable {
    pub ast : Vec<Node>,

    pub Prepass : HashSet<String>,
    pub Structs : HashMap<String , Vec<Item>>, 
    pub Functions : HashMap<String , Function>,
    pub Scope_Stack : Vec<HashMap<String , Data_Type>>,
}

impl SiTable{

    pub fn new(nodes : Vec<Node>) -> Self {
        Self { ast: nodes , Prepass : HashSet::new() , Structs: HashMap::new(), Functions: HashMap::new(), Scope_Stack: Vec::new() }
    }

    pub fn get_prepass(&mut self ) {
        for i in self.ast.iter(){
            if let Node::STRUCTURE { name, body : _} = i {
                if self.Prepass.contains(name) {
                    panic!("Structure definition with same name");
                }
                self.Prepass.insert(name.clone());
            }
        }
    }

    pub fn lookup_prepass(&self , name : &str) -> bool {
        self.Prepass.contains(name)
    }


    pub fn get_all_structs(&mut self) {
        for i in self.ast.iter(){
            if let Node::STRUCTURE { name, body } = i {
                let mut itemvec : Vec<Item> = Vec::new();

                for k in body{
                    if let Node::PARAM { name, d_type } = k{
                        itemvec.push(Item::new(name.clone(), d_type.clone()));
                    }
                }

                self.Structs.insert(name.clone() , itemvec );
            }
        }
    }

    pub fn get_all_functions(&mut self) {
        for i in self.ast.iter(){
            if let Node::FUNCTION_DEC { name, r_type, params, body : _ } = i {

                if self.Functions.contains_key(name) {
                    panic!("Two functions cannot have the same name");
                }

                let mut itemvec : Vec<Item> = Vec::new();

                for k in params.iter(){
                    if let Node::PARAM { name, d_type } = k {
                        itemvec.push(Item::new(name.clone(), d_type.clone()));
                    }
                }

                self.Functions.insert(name.clone(), Function { id: name.clone(), d_type: r_type.clone(), params: itemvec });
            }
        }
    }

    pub fn push_all_function_params_into_scope(&mut self, name : &str) {

        let itemvec = match self.Functions.get(name){
            Some(v) => v.params.clone(),
            None => panic!("Function not present " ),
        };                   

        
        if let Some(scope) = self.Scope_Stack.last_mut(){

            for i in itemvec {
                scope.insert(i.id, i.d_type);
            }
        }else{
            panic!("Scope is empty");
        }
    }

    pub fn push_all_variables_in_scope(&mut self ) {

    }

    pub fn scope_resolver(&mut self ) {

        let mut scope : HashMap<String , Data_Type> = HashMap::new(); 

        self.Scope_Stack.push(scope);
    }


}



