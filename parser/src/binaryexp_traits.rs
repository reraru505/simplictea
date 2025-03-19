use crate::binaryexp::{BinaryExpression, BinaryExpressionTree, Operator};

impl Clone for Operator {
    fn clone(&self) -> Self {
        match self {
            Operator::ADDITION => return Operator::ADDITION,
            Operator::SUBTRACTION => return Operator::SUBTRACTION,
            Operator::DIVISION => return Operator::DIVISION,
            Operator::MULTIPLICATION => return Operator::MULTIPLICATION,
            Operator::ASSIGNMENT => return Operator::ASSIGNMENT,
            Operator::AND => return Operator::AND,
            Operator::OR => return Operator::OR,
            Operator::NOT => return Operator::NOT,
            Operator::CHECKEQUAL => return Operator::CHECKEQUAL,
            Operator::CHECKNEQUAL => return Operator::CHECKNEQUAL,
            Operator::GREATER => return Operator::GREATER,
            Operator::LESSER => return Operator::LESSER,
            Operator::BRACKET => return Operator::BRACKET,
        }
    }
}

impl Clone for BinaryExpression {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            operator: self.operator.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl Clone for BinaryExpressionTree {
    fn clone(&self) -> Self {
        Self {
            super_scope: self.super_scope.clone(),
            tree: self.tree.clone(),
        }
    }
}
