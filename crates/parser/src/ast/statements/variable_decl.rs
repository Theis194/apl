use super::super::Expr;

pub struct VariableDecl {
    name: String,
    initializer: Expr,
}

impl VariableDecl {
    pub fn new(name: String, initializer: Expr) -> Self {
        Self { name, initializer }
    }
}