use crate::utils::{self, extract_op};

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
    }

    #[test]
    fn parse_sub() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
    }

    #[test]
    fn parse_div() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }

    #[test]
    fn parse_mul() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
    }
}
