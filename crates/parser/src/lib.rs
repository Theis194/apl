pub mod core;

trait AstNode {
    fn visit();
    fn accept();    
}

trait Expression {
    fn eval();
}