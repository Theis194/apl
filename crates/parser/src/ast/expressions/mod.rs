mod binary;
mod variable;

pub use binary::{BinaryExpr, BinaryOp};
pub use variable::Variable;

pub enum Expr {
    Literal(Literal),
    Variable(Variable),
    Binary(BinaryExpr)
}

pub enum Literal {
    Integer(i32),
    Float(f32),
}
