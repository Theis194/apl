mod variable_decl;
mod function;

pub use function::Function;
pub use variable_decl::VariableDecl;

use super::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VariableDecl(VariableDecl),
    Expression(Expr),
    FunctionDecl(Function),
}