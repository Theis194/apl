use apl_derive::New;

use crate::ast::Expr;

use super::Stmt;

#[derive(New, PartialEq, Debug)]
pub struct Function {
    name: String,
    params: Vec<String>,
    statements: Vec<Stmt>,
}

#[derive(New, PartialEq, Debug)]
pub struct FunctionCall {
    name: String,
    params: Vec<Expr>,
}

#[derive(New, PartialEq, Debug)]
pub struct MethodCall {
    obj_name: String,
    name: String,
    params: Vec<Expr>,
}