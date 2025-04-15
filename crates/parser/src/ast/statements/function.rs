use apl_derive::New;

use crate::ast::node::Statement;

use super::Stmt;

#[derive(New)]
pub struct Function {
    name: String,
    params: Vec<Box<dyn Statement>>,
    statements: Vec<Stmt>,
}