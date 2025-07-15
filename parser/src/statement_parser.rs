#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::ast::AST_ConditionalType;
use crate::ast::AST_Expression;
use crate::ast::AST_IteratorType;
use crate::parser::Parser;
use crate::ast::AST_Node;
use crate::ast::AST_Statement;
use crate::ast::AST_TypeAnnotation;

use crate::lexer::token_type::Token;
use crate::lexer::token_type::Keyword;
use crate::lexer::token_type::Statement;

use crate::symboltable::symbol::Type;

impl Parser {
    pub fn Variable_Declaration_Parser(&mut self) -> Option<AST_Statement> {
        let last = self.current;

        let name: AST_Expression;
        let type_annotation: AST_TypeAnnotation;

        // Parse identifier
        if let Some(identifier) = self.Declaration_Identifier_Parser() {
            name = identifier;
        } else {
            self.current = last;
            return None;
        }

        // Check for type assignment marker
        if !self.is_type_assignment_marker() {
            self.current = last;
            return None;
        }

        // Parse type annotation
        if let Some(annotation) = self.Type_Annotation_Parser() {
            type_annotation = annotation;
        } else {
            self.current = last;
            return None;
        }

        // Register the variable/field
        let var_name = self.extract_identifier_name(name.clone()).unwrap();
        let sym_type = self.annotation_to_symbol_primary(type_annotation.clone());
        self.symtable.register_variable(var_name, sym_type);

        // Handle end of expression or assignment
        if self.is_expression_end_marker() {
            return Some(AST_Statement::LET {
                name,
                type_annotation,
                initializer: None,
            });
        } else if self.is_assignment_marker() {
            if let Some(AST_Node::expression(expression)) = self.Expression_Parser() {
                if self.is_expression_end_marker() {
                    return Some(AST_Statement::LET {
                        name,
                        type_annotation,
                        initializer: Some(expression),
                    });
                }
            }
        }

        self.current = last;
        None
    }                                    
    pub fn Function_Declaration_Parser(&mut self) -> Option<AST_Statement> {
        let last = self.current;

        let name: AST_Expression;
        let type_annotation: AST_TypeAnnotation;
        let param: Option<Vec<AST_Statement>>;
        let body: Vec<AST_Node>;

        // Parse function name
        if let Some(identifier) = self.Declaration_Identifier_Parser() {
            name = identifier;
        } else {
            return self.vomit_and_die(last);
        }

        // Check for type assignment marker
        if !self.is_type_assignment_marker() {
            return self.vomit_and_die(last);
        }

        // Parse return type
        if let Some(annotation) = self.Type_Annotation_Parser() {
            type_annotation = annotation;
        } else {
            return self.vomit_and_die(last);
        }

        // Check for function marker
        if !self.is_function_marker() {
            return self.vomit_and_die(last);
        }

        // Parse parameters if any
        param = if self.is_param_start() {
            self.param_parser()
        } else {
            None
        };

        // Parse function body
        if self.is_body_start() {
            let fn_name = if let AST_Expression::Identifier(ref name) = name {
                name.clone()
            } else {
                return self.vomit_and_die(last);
            };

            // Track if we entered a function scope
            let mut entered_function_scope = false;

            // Only register if not duplicate
            if !self.symtable.check_duplicate_in_current_scope(&fn_name) {
                self.symtable.register_function(
                    fn_name.clone(),
                    self.annotation_to_symbol_primary(type_annotation.clone()),
                );
                entered_function_scope = true;
            }

            // Parse function body while still in function scope
            let body_result = self.Body_Parser();

            // Only exit scope if we entered it, and do it AFTER parsing body
            if entered_function_scope {
                self.symtable.exit_scope();
            }

            match body_result {
                Some(fn_body) => {
                    body = fn_body;
                }
                None => {
                    body = Vec::new();
                }
            }

            return Some(AST_Statement::FUNCTION {
                name,
                type_annotation: Some(type_annotation),
                params: param,
                body,
            });
        } else {
            return self.vomit_and_die(last);
        }
    }

    pub fn Function_Declaration_Parser_old(&mut self) -> Option<AST_Statement> {
        let last = self.current;

        let name: AST_Expression;
        let type_annotation: AST_TypeAnnotation;
        let param: Option<Vec<AST_Statement>>;
        let body: Vec<AST_Node>;

        // Parse function name
        if let Some(identifier) = self.Declaration_Identifier_Parser() {
            name = identifier;
        } else {
            return self.vomit_and_die(last);
        }

        // Check for type assignment marker
        if !self.is_type_assignment_marker() {
            return self.vomit_and_die(last);
        }

        // Parse return type
        if let Some(annotation) = self.Type_Annotation_Parser() {
            type_annotation = annotation;
        } else {
            return self.vomit_and_die(last);
        }

        // Check for function marker
        if !self.is_function_marker() {
            return self.vomit_and_die(last);
        }

        // Parse parameters if any
        param = if self.is_param_start() {
            self.param_parser()
        } else {
            None
        };

        // Parse function body
        if self.is_body_start() {
            let fn_name = if let AST_Expression::Identifier(ref name) = name {
                name.clone()
            } else {
                return self.vomit_and_die(last);
            };

            // Track if we entered a function scope
            let mut entered_function_scope = false;

            // Only register if not duplicate
            if !self.symtable.check_duplicate_in_current_scope(&fn_name) {
                self.symtable.register_function(
                    fn_name.clone(),
                    self.annotation_to_symbol_primary(type_annotation.clone()),
                );
                entered_function_scope = true;
            }

            // Parse function body
            if let Some(fn_body) = self.Body_Parser() {
                body = fn_body;
            } else {
                body = Vec::new();
            }

            // Only exit scope if we entered it
            if entered_function_scope {
                self.symtable.exit_scope();
            }

            return Some(AST_Statement::FUNCTION {
                name,
                type_annotation: Some(type_annotation),
                params: param,
                body,
            });
        } else {
            return self.vomit_and_die(last);
        }
    }

    pub fn Assignment_Parser(&mut self) -> Option<AST_Statement> {
        let checkpoint = self.current;

        let lhs: AST_Expression;
        let rhs: AST_Expression;

        if let Some(identifier) = self.lhs_parser() {
            lhs = identifier;
        } else {
            return self.vomit_and_die(checkpoint);
        }

        if !self.is_assignment_marker() {
            return self.vomit_and_die(checkpoint);
        }

        if let Some(AST_Node::expression(expr)) = self.Expression_Parser() {
            rhs = expr;
        } else {
            return self.vomit_and_die(checkpoint);
        }

        if self.is_expression_end_marker() {
            // Skip the lookup for object items since they're composite expressions
            if let AST_Expression::OBJECT_ITEM { .. } = lhs {
                // No need to lookup object items as variables
            } 
            // Only verify simple identifiers
            else if let AST_Expression::Identifier(id) = &lhs {
                if self.symtable.lookup_variable(id).is_none() {
                    panic!(" variable {:?} is not in the active scope", id);
                    return self.vomit_and_die(checkpoint);
                }
            }

            return Some(AST_Statement::ASSIGNMENT {
                LHS: lhs,
                RHS: rhs,
            });
        } else {
            // Try to be forgiving - maybe they forgot the semicolon?
            if self.is_body_end() {
                return Some(AST_Statement::ASSIGNMENT {
                    LHS: lhs,
                    RHS: rhs,
                });
            }
            return self.vomit_and_die(checkpoint);
        }
    }
    pub fn Assignment_Parser_old(&mut self) -> Option<AST_Statement> {
        let checkpoint = self.current;

        let lhs: AST_Expression;
        let rhs: AST_Expression;

        if let Some(identifier) = self.lhs_parser() {
            lhs = identifier;
        } else {
            return self.vomit_and_die(checkpoint);
        }

        if !self.is_assignment_marker() {
            return self.vomit_and_die(checkpoint);
        }

        if let Some(AST_Node::expression(expr)) = self.Expression_Parser() {
            rhs = expr;
        } else {
            return self.vomit_and_die(checkpoint);
        }

        if self.is_expression_end_marker() {
            if let AST_Expression::Identifier(id) = lhs.clone() {
                if self.symtable.lookup_variable(&id).is_none() {
                    panic!(" variable {:?} is not in the active scope", lhs.clone());
                    return self.vomit_and_die(checkpoint);
                }
            }

            return Some(AST_Statement::ASSIGNMENT {
                LHS: lhs,
                RHS: rhs,
            });
        } else {
            return self.vomit_and_die(checkpoint);
        }
    }

    pub fn param_parser(&mut self) -> Option<Vec<AST_Statement>> {
        let mut param: Vec<AST_Statement> = Vec::new();
        let checkpoint = self.current;

        if self.is_param_end() {
            return None;
        }

        loop {
            let name: AST_Expression;
            let type_annotation: AST_TypeAnnotation;

            if let Some(identifier) = self.Declaration_Identifier_Parser() {
                name = identifier;
            } else {
                return self.vomit_and_die(checkpoint);
            }

            if !self.is_type_assignment_marker() {
                return self.vomit_and_die(checkpoint);
            }

            if let Some(annotation) = self.Type_Annotation_Parser() {
                type_annotation = annotation;
            } else {
                return self.vomit_and_die(checkpoint);
            }

            // Register parameter
            let param_name = self.extract_identifier_name(name.clone()).unwrap();
            let sym_type = self.annotation_to_symbol_primary(type_annotation.clone());
            self.symtable.register_variable(param_name, sym_type);

            param.push(AST_Statement::LET {
                name,
                type_annotation,
                initializer: None,
            });

            if self.is_comma_seperator() {
                continue;
            } else if self.is_param_end() {
                return Some(param);
            } else {
                return self.vomit_and_die(checkpoint);
            }
        }
    }

    pub fn Conditional_Parser(&mut self) -> Option<AST_Statement> {
        let checkpoint = self.current;
        let con_type: AST_ConditionalType;
        let condition: AST_Expression;
        let body: Vec<AST_Node>;

        if let Some(contype) = self.conditional_type_finder() {
            con_type = contype;
        } else {
            return self.vomit_and_die(checkpoint);
        }

        match con_type {
            AST_ConditionalType::IF => {
                if let Some(AST_Node::expression(expr)) = self.Expression_Parser() {
                    condition = expr;

                    if self.is_body_start() {
                        self.symtable.enter_block_scope();
                        body = self.Body_Parser()?;
                        self.symtable.exit_scope();

                        return Some(AST_Statement::CONDITIONAL {
                            conditional_type: con_type,
                            condition: Some(condition),
                            body,
                        });
                    } else {
                        return self.vomit_and_die(checkpoint);
                    }
                } else {
                    return self.vomit_and_die(checkpoint);
                }
            }

            AST_ConditionalType::ELIF => {
                if let Some(AST_Node::expression(expr)) = self.Expression_Parser() {
                    condition = expr;

                    if self.is_body_start() {
                        self.symtable.enter_block_scope();
                        body = self.Body_Parser()?;
                        self.symtable.exit_scope();

                        return Some(AST_Statement::CONDITIONAL {
                            conditional_type: con_type,
                            condition: Some(condition),
                            body,
                        });
                    } else {
                        return self.vomit_and_die(checkpoint);
                    }
                } else {
                    return self.vomit_and_die(checkpoint);
                }
            }

            AST_ConditionalType::ELSE => {
                if self.is_body_start() {
                    self.symtable.enter_block_scope();
                    body = self.Body_Parser()?;
                    self.symtable.exit_scope();

                    return Some(AST_Statement::CONDITIONAL {
                        conditional_type: con_type,
                        condition: None,
                        body,
                    });
                } else {
                    return self.vomit_and_die(checkpoint);
                }
            }
        }
    }

    pub fn Return_Statement_Parser_old(&mut self) -> Option<AST_Statement> {
        let current_token = self.current_token();
        let checkpoint = self.current;

        if let Token::t_keyword(Keyword::statement(Statement::return_statement(_))) = current_token {
            if !self.consume() {
                return self.vomit_and_die(checkpoint);
            }

            // Handle empty return (void functions)
            if self.is_expression_end_marker() {
                return Some(AST_Statement::RETURN(
                        AST_Expression::Identifier("".to_string())
                ));
            }

            let exp: AST_Expression;

            if let Some(AST_Node::expression(expr)) = self.Expression_Parser() {
                exp = expr;
            } else {
                return self.vomit_and_die(checkpoint);
            }

            if self.is_expression_end_marker() {
                return Some(AST_Statement::RETURN(exp));
            } else {
                // Try to be forgiving - maybe they forgot the semicolon?
                if self.is_body_end() {
                    return Some(AST_Statement::RETURN(exp));
                }
                return self.vomit_and_die(checkpoint);
            }
        } else {
            None
        }
    }

    pub fn Return_Statement_Parser(&mut self) -> Option<AST_Statement> {
        let current_token = self.current_token();
        let checkpoint = self.current;

        if let Token::t_keyword(Keyword::statement(Statement::return_statement(_))) = current_token {
            if !self.consume() {
                return self.vomit_and_die(checkpoint);
            }

            // Check if next token is end marker (empty return)
            if self.is_expression_end_marker() {
                return Some(AST_Statement::RETURN(
                        AST_Expression::Identifier("".to_string())
                ));
            }

            // Parse return expression
            if let Some(AST_Node::expression(expr)) = self.Expression_Parser() {
                // Handle optional semicolon
                if self.is_expression_end_marker() || self.is_body_end() {
                    return Some(AST_Statement::RETURN(expr));
                }
            }
        }

        self.current = checkpoint;
        None
    }
    pub fn Iterator_Parser(&mut self) -> Option<AST_Statement> {
        let checkpoint = self.current;

        // Checking for <FOR> keyword
        if let Token::t_keyword(Keyword::statement(Statement::for_statement(_))) = self.current_token()
        {
            if !self.consume() {
                return self.vomit_and_die(checkpoint);
            }
        } else {
            return None;
        }

        if let Some(AST_Node::expression(expr)) = self.Expression_Parser() {
            let body: Vec<AST_Node>;

            self.symtable.enter_block_scope();
            if self.is_body_start() {
                body = self.Body_Parser()?;
            } else {
                return self.vomit_and_die(checkpoint);
            }
            self.symtable.exit_scope();

            return Some(AST_Statement::ITERATOR {
                iterator_type: AST_IteratorType::CONDITIONAL_ITERATOR,
                condition: Some(expr),
                body,
            });
        } else {
            let body: Vec<AST_Node>;

            self.symtable.enter_block_scope();
            if self.is_body_start() {
                body = self.Body_Parser()?;
            } else {
                return self.vomit_and_die(checkpoint);
            }
            self.symtable.exit_scope();

            return Some(AST_Statement::ITERATOR {
                iterator_type: AST_IteratorType::UNCONDITIONAL_ITERATOR,
                condition: None,
                body,
            });
        }
    }

    pub fn Struct_Parser(&mut self) -> Option<AST_Statement> {
        let checkpoint = self.current;

        if matches!(
            self.current_token(),
            Token::t_keyword(Keyword::statement(Statement::struct_statement(_)))
        ) {
            if !self.consume() {
                return self.vomit_and_die(checkpoint);
            }
        } else {
            return None;
        }

        let name: AST_Expression;

        if let Some(id) = self.Declaration_Identifier_Parser() {
            name = id;
        } else {
            return self.vomit_and_die(checkpoint);
        }

        if self.is_body_start() {
            // Entering the struct scope
            let struct_name = self.extract_identifier_name(name.clone()).unwrap();
            self.symtable.enter_struct_scope(struct_name.clone());

            if let Some(body) = self.Struct_Body_Parser() {
                // Exiting the scope
                self.symtable.exit_scope();

                // Register the struct type itself
                self.symtable.register_struct(struct_name, Type::IS_A_TYPE);

                return Some(AST_Statement::STRUCTURE { name, body });
            } else {
                return self.vomit_and_die(checkpoint);
            }
        } else {
            return self.vomit_and_die(checkpoint);
        }
    }

    pub fn Struct_Body_Parser(&mut self) -> Option<Vec<AST_Node>> {
        let mut body: Vec<AST_Node> = Vec::new();

        loop {
            if self.is_body_end() {
                return Some(body);
            }

            // Only allow variable declarations and nested structs in struct bodies
            if let Some(var_dec) = self.Variable_Declaration_Parser() {
                body.push(AST_Node::statement(var_dec));
            } else if let Some(structure) = self.Struct_Parser() {
                body.push(AST_Node::statement(structure));
            } else {
                // If we didn't parse anything, advance to avoid infinite loop
                if !self.consume() {
                    return Some(body);
                }
            }
        }
    }
    pub fn Body_Parser_old(&mut self) -> Option<Vec<AST_Node>> {
        let mut body: Vec<AST_Node> = Vec::new();

        loop {
            if self.is_body_end() {
                return Some(body);
            }

            // Check for return statement first since it's common
            if let Some(ret) = self.Return_Statement_Parser() {
                body.push(AST_Node::statement(ret));
                continue;
            }

            // Then try other statements
            if let Some(var_dec) = self.Variable_Declaration_Parser() {
                body.push(AST_Node::statement(var_dec));
            } else if let Some(fn_dec) = self.Function_Declaration_Parser() {
                body.push(AST_Node::statement(fn_dec));
            } else if let Some(var_assign) = self.Assignment_Parser() {
                body.push(AST_Node::statement(var_assign));
            } else if let Some(conditional) = self.Conditional_Parser() {
                body.push(AST_Node::statement(conditional));
            } else if let Some(itera) = self.Iterator_Parser() {
                body.push(AST_Node::statement(itera));
            } else if let Some(structure) = self.Struct_Parser() {
                body.push(AST_Node::statement(structure));
            } else {
                // If we didn't parse anything, advance to avoid infinite loop
                if !self.consume() {
                    return Some(body);
                }
            }
        }
    }



    pub fn Body_Parser(&mut self) -> Option<Vec<AST_Node>> {
        let mut body: Vec<AST_Node> = Vec::new();

        loop {
            if self.is_body_end() {
                return Some(body);
            }

            let checkpoint = self.current;

            // Try all statement types in sequence
            if let Some(ret) = self.Return_Statement_Parser() {
                body.push(AST_Node::statement(ret));
                continue;
            }

            if let Some(var_dec) = self.Variable_Declaration_Parser() {
                body.push(AST_Node::statement(var_dec));
                continue;
            }

            if let Some(var_assign) = self.Assignment_Parser() {
                body.push(AST_Node::statement(var_assign));
                continue;
            }

            if let Some(fn_dec) = self.Function_Declaration_Parser() {
                body.push(AST_Node::statement(fn_dec));
                continue;
            }

            if let Some(conditional) = self.Conditional_Parser() {
                body.push(AST_Node::statement(conditional));
                continue;
            }

            if let Some(itera) = self.Iterator_Parser() {
                body.push(AST_Node::statement(itera));
                continue;
            }

            if let Some(structure) = self.Struct_Parser() {
                body.push(AST_Node::statement(structure));
                continue;
            }

            // If no statement was parsed, advance to avoid infinite loop
            if !self.consume() {
                return Some(body);
            }
        }
    }
}
