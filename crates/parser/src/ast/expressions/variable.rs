use apl_derive::New;

use crate::ast::node::{AstNode, Statement};

#[derive(Debug, New)]
pub struct Variable {
    pub name: String,
}

impl AstNode for Variable {
    fn visit(&self) {
        todo!()
    }
    
    fn accept(&self) {
        todo!()
    }
}

impl Statement for Variable {
    fn execute(&self) {
        todo!()
    }
}