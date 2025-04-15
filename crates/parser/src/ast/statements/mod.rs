mod variable_decl;
mod function;

pub use variable_decl::VariableDecl;


pub enum Stmt {
    VariableDecl(VariableDecl)
}