use crate::{utils, val::Val};

use super::{numbers::Int, operators::Op};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Int),
    Operation { lhs: Int, rhs: Int, op: Op },
}

impl Expr {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s).or_else(|_| Self::new_int(s))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Int::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Int::new(s)?;

        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    fn new_int(s: &str) -> Result<(&str, Self), String> {
        Int::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub(crate) fn eval(&self) -> Val {
        match self {
            Self::Number(Int(n)) => Val::Int(*n),
            Self::Operation { lhs, rhs, op } => {
                let Int(lhs) = lhs;
                let Int(rhs) = rhs;

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Div => lhs / rhs,
                    Op::Mul => lhs * rhs,
                };

                Val::Int(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Int(1),
                    rhs: Int(2),
                    op: Op::Add,
                }
            ))
        )
    }

    #[test]
    fn parse_with_whitespaces() {
        assert_eq!(
            Expr::new("1 * 2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Int(1),
                    rhs: Int(2),
                    op: Op::Mul,
                }
            ))
        )
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Int(10),
                rhs: Int(10),
                op: Op::Add,
            }
            .eval(),
            Val::Int(20),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Int(10),
                rhs: Int(10),
                op: Op::Sub,
            }
            .eval(),
            Val::Int(0),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Int(10),
                rhs: Int(2),
                op: Op::Div,
            }
            .eval(),
            Val::Int(5),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Int(10),
                rhs: Int(10),
                op: Op::Mul,
            }
            .eval(),
            Val::Int(100),
        );
    }
}
