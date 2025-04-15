mod variable_decl;

pub use variable_decl::VariableDecl;


pub enum Stmt {
    VariableDecl(VariableDecl)
}