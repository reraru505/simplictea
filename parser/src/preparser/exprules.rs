use crate::preparser::preparsingdata::PreParsingData;

use crate::lexer::token_type::Token;
use crate::lexer::lex::Lexeme; 
use crate::lexer::token_type::STC;

pub fn lexemes_to_preparsingdata(lexemes : Vec<Lexeme>) -> Vec<PreParsingData> {

    let mut retval : Vec<PreParsingData> = Vec::new();

    for i in lexemes.iter(){
        retval.push(
            PreParsingData::token(
                i.tokens.clone()
            )
        );
    }

    return retval;

}

pub fn has_inner_block(block : Vec<PreParsingData> ) -> bool {

    for i in block.iter(){
        if i.is_scope_begin(){
            return true;
        }

    }

    return false;
}

pub fn get_all_blocks(ppd : Vec<PreParsingData>) -> Vec<PreParsingData> {

    let mut retval : Vec<PreParsingData> = Vec::new();
    let mut buffer : Vec<PreParsingData> = Vec::new();

    let mut collecting = false;
    let mut inner = 0;

    for i in ppd.iter(){
        
        if !collecting {

            if i.is_token(){

                if i.is_scope_begin(){
                    collecting = true;
                }else{
                    retval.push(i.clone());
                }
            }

        }
        
        else {

            if i.is_scope_end(){

                if inner == 0{

                    retval.push(
                        PreParsingData::block(buffer.clone())
                    );
                    buffer.clear();
                    collecting = false;

                }else {
                    buffer.push(i.clone());
                    inner -= 1;
                }

            }else if i.is_scope_begin(){
                inner += 1;
                buffer.push(i.clone());
            }else{
                buffer.push(i.clone());
            } 
        }





    } // for end 
    
    let mut new_retval : Vec<PreParsingData> = Vec::new();

    for i in retval.iter_mut(){
        
        if i.is_block(){
            if let PreParsingData::block(block) = i.clone(){
                
                if has_inner_block(block.clone()){
                   println!("\n\nFound inner block\n\n"); 
                   // for i in block.clone().iter(){
                   //     i.print();
                   // }
                   new_retval.push(PreParsingData::block( get_all_blocks(block)) ); 
                }else{
                    new_retval.push(i.clone());
                }
            }
        }else {
            new_retval.push(i.clone());
        }
    }

    return new_retval;
}

pub fn is_token_block(block : Vec<PreParsingData> ) -> bool {

    for i in block.iter(){
        if i.is_token(){
            return true;
        }
    }
    return false;
}

pub fn get_all_expressions(ppd : Vec<PreParsingData> ) -> Vec<PreParsingData>{
    
    let mut retval : Vec<PreParsingData> = Vec::new();
    let mut buffer : Vec<Token> = Vec::new();

    for i in ppd.iter(){
    
        if i.is_token(){
            if i.is_exp_end(){
                retval.push(PreParsingData::expression(buffer.clone()));
                buffer.clear();
            }else{
                buffer.push(i.get_token());
            }
        }else if i.is_block(){
            if !buffer.is_empty(){
                retval.push(PreParsingData::expression(buffer.clone()));
            }
            retval.push(i.clone());
            buffer.clear();
        }else {
            retval.push(i.clone());
        }
    }

    let mut new_retval : Vec<PreParsingData> = Vec::new();
    for i in retval.iter(){
        if i.is_block(){
            if let PreParsingData::block(block ) = i.clone(){
                if is_token_block(block.clone()){
                    new_retval.push(
                        PreParsingData::block(get_all_expressions(block))
                    );
                }else {
                    new_retval.push(i.clone());
                }
            }
        }else{
            new_retval.push(i.clone());
        }
    }

    return new_retval;
}





