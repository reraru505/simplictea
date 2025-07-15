#![allow(non_camel_case_types)]

use std::panic;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryType {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    CHAR,
    VOID,
    STRING,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    GLOBAL,
    FUNCTION(String),
    STRUCT(String),
    BLOCK(usize),     
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    PRIMARY(PrimaryType),
    CUSTOM(String),
    IS_A_TYPE,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    FUNCTION,
    STRUCT,
    VARIABLE,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: Scope,
    pub data_type: Type,
    pub kind: Kind,
}


#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// A flat list of all symbols declared in any scope.
    pub symbols: Vec<Symbol>,
    /// A stack to manage the current scope hierarchy. The top is the current scope.
    scope_stack: Vec<Scope>,
    /// A counter to generate unique IDs for block scopes.
    block_counter: usize,
}

impl SymbolTable {
    /// Creates a new, empty symbol table.
    /// It automatically initializes with the GLOBAL scope.
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            // The scope stack always starts with the global scope.
            scope_stack: vec![Scope::GLOBAL],
            block_counter: 0,
        }
    }

    // --- Scope Management ---

    /// Enters a new function scope.
    pub fn enter_function_scope(&mut self, name: String) {
        self.scope_stack.push(Scope::FUNCTION(name));
    }

    /// Enters a new block scope (e.g., inside an if, while, or for).
    pub fn enter_block_scope(&mut self) {
        let block_id = self.block_counter;
        self.block_counter += 1;
        self.scope_stack.push(Scope::BLOCK(block_id));
    }

    /// Exits the current scope.
    /// Panics if you try to exit the GLOBAL scope, which indicates a bug in the compiler.
    pub fn exit_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        } else {
            panic!("Compiler Error: Attempted to exit the GLOBAL scope.");
        }
    }

    /// Returns a reference to the current, innermost scope.
    fn current_scope(&self) -> &Scope {
        // This unwrap is safe because the stack is never empty.
        self.scope_stack.last().unwrap()
    }

    // --- Symbol Registration ---

    /// A private helper to register a new symbol in the current scope.
    fn register(&mut self, name: String, data_type: Type, kind: Kind) {
        let scope = self.current_scope().clone();

        // Check for duplicates in the *current* scope only.
        // Shadowing is allowed, but re-declaration in the same scope is not.
        if self.check_duplicate_in_scope(&name, &scope) {
            panic!(
                "Identifier '{}' has already been declared in the current scope.",
                name
            );
        }

        self.symbols.push(Symbol {
            name,
            scope,
            data_type,
            kind,
        });
    }

    /// Checks if a symbol with the given name already exists in a specific scope.
    fn check_duplicate_in_scope(&self, name: &str, scope: &Scope) -> bool {
        self.symbols
            .iter()
            .any(|s| s.scope == *scope && s.name == name)
    }

    /// Checks if a symbol with the given name already exists in the current scope.
    pub fn check_duplicate_in_current_scope(&self, name: &str) -> bool {
        let scope = self.current_scope();
        self.check_duplicate_in_scope(name, scope)
    }

    /// Registers a new function in the current scope.
    pub fn register_function(&mut self, name: String, return_type: Type) {
        self.register(name.clone(), return_type, Kind::FUNCTION);
        self.enter_function_scope(name);
    }

    /// Registers a new struct definition in the current scope.
    pub fn register_struct(&mut self, name: String, type_info: Type) {
        self.register(name, type_info, Kind::STRUCT);
    }

    /// Registers a new variable in the current scope.
    pub fn register_variable(&mut self, name: String, data_type: Type) {
        self.register(name, data_type, Kind::VARIABLE);
    }

    // --- Symbol Lookup ---

    /// Looks up a symbol by name, searching from the current scope outwards.
    /// This correctly handles scope hierarchy and shadowing.
    ///
    /// # Arguments
    /// * `name` - The name of the symbol to find.
    ///
    /// # Returns
    /// An `Option<&Symbol>` containing the found symbol, or `None` if it's not found.
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Iterate through the scope stack in reverse (from current to global).
        for scope in self.scope_stack.iter().rev() {
            // Find the first symbol in our flat list that matches both the name and the current scope in the hierarchy.
            if let Some(symbol) = self
                .symbols
                    .iter()
                    .find(|s| s.scope == *scope && s.name == name)
            {
                // Found the symbol in the closest scope, so return it.
                return Some(symbol);
            }
        }
        // If the loop finishes, the symbol was not found in any active scope.
        None
    }

    /// A convenience method to look up a symbol that is specifically a struct.
    pub fn lookup_struct(&self, name: &str) -> Option<&Symbol> {

        self.lookup(name).and_then(|symbol| {
            if matches!(symbol.kind, Kind::STRUCT) {
                Some(symbol)
            } else {
                println!("Struct lookup returned none");
                None
            }
        })
    }


    /// A convenience method to look up a symbol that is specifically a variable.
    pub fn lookup_variable(&self, name: &str) -> Option<&Symbol> {
        self.lookup(name).and_then(|symbol| {
            if matches!(symbol.kind, Kind::VARIABLE) {
                Some(symbol)
            } else {
                None
            }
        })
    }

    /// A convenience method to look up a symbol that is specifically a function.
    pub fn lookup_function(&self, name: &str) -> Option<&Symbol> {
        self.lookup(name).and_then(|symbol| {
            if matches!(symbol.kind, Kind::FUNCTION) {
                Some(symbol)
            } else {
                None
            }
        })
    }

    // Enter struct definition scope
    pub fn enter_struct_scope(&mut self, struct_name: String) {
        self.scope_stack.push(Scope::STRUCT(struct_name));
    }

    // Look up a field within a specific struct
    pub fn lookup_struct_field_from_struct_name(&self, struct_name: &str, field_name: &str) -> Option<&Symbol> {
        let target_scope = Scope::STRUCT(struct_name.to_string());
        self.symbols.iter()
            .find(|s| s.scope == target_scope && s.name == field_name && s.kind == Kind::VARIABLE)
    }

    // Look up a field within a specific object's struct
    pub fn lookup_struct_field(&self, object_name: &str, field_name: &str) -> Option<&Symbol> {
        if let Some(object_symbol) = self.lookup_variable(object_name) {
            if let Type::CUSTOM(struct_name) = &object_symbol.data_type {
                // Only check if struct exists
                if self.lookup_struct(struct_name).is_some() {
                    let target_scope = Scope::STRUCT(struct_name.to_string());
                    return self.symbols.iter()
                        .find(|s| s.scope == target_scope && s.name == field_name && s.kind == Kind::VARIABLE);
                }
            }
        }
        None
    }
}
