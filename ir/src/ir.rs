use crate::parser::ast::AST_Node;

// Data types in the IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    Int,
    Float,
    Bool,
    Void,
    Ptr,
}

// Immediate (literal) values.
#[derive(Debug, Clone, PartialEq)]
pub enum ImmediateData {
    Int(i64),
    Uint(u64),
    Float(f64),
    Bool(bool),
    StringLiteral(String),
    LabelAddr(String),
}

// Operands for instructions: registers or immediates.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// Virtual register and its type.
    Register(usize),
    /// Immediate literal value.
    Immediate(ImmediateData),
    /// No operand.
    NoOp,
}


// A single operation in the flat IR.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {

    // Arithmetic: result = op1 op op2
    Add(Operand, Operand, Operand),
    Sub(Operand, Operand, Operand),
    Mul(Operand, Operand, Operand),
    Div(Operand, Operand, Operand),
    Mod(Operand, Operand, Operand),

    // Logical/Bitwise: result = op1 op op2
    And(Operand, Operand, Operand),
    Or(Operand, Operand, Operand),
    Xor(Operand, Operand, Operand),
    Not(Operand, Operand),

    // Comparison: result_bool = op1 rel_op op2
    CmpEq(Operand, Operand, Operand),
    CmpNe(Operand, Operand, Operand),
    CmpLt(Operand, Operand, Operand),
    CmpLe(Operand, Operand, Operand),
    CmpGt(Operand, Operand, Operand),
    CmpGe(Operand, Operand, Operand),

    // Data Movement
    Assign(Operand, Operand),

    // Memory Access
    Load(Operand, Operand),
    Store(Operand, Operand),
    GetAddress(Operand, String),

    // Control Flow
    Label(String),
    Jump(String),
    JumpIfTrue(String, Operand),
    JumpIfFalse(String, Operand),

    // Function Calls
    PassArg(Operand),
    Call(Operand, String, usize),
    Return(Operand),

    // Allocation
    AllocStack(Operand, Operand),

    // Utility
    Nop,
}


#[derive(Debug, Clone)]
pub struct IrProgram {

    pub ast : Vec<AST_Node>,

    pub instructions: Vec<Instruction>,
    pub next_reg_id: usize,
}

