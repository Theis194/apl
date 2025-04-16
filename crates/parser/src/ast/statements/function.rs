use apl_derive::New;

use super::Stmt;

#[derive(New)]
pub struct Function {
    name: String,
    params: Vec<String>,
    statements: Vec<Stmt>,
}