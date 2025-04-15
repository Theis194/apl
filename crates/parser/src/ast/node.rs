pub trait AstNode {
    fn visit(&self);
    fn accept(&self);
}

pub trait Statement: AstNode {
    fn execute(&self);
}

pub trait Expression {
    fn eval();
}