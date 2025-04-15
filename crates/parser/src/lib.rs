pub mod ast;
pub mod core;
mod declarations;
mod expressions;
mod statements;

use ast::{BinaryExpr, BinaryOp};
use ast::{Expr, Literal, Stmt};
use ast::{Variable, VariableDecl};
use core::Parser;
