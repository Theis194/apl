use crate::{env::Env, utils};

use super::expression::Expr;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.store_binding(self.name.clone(), self.val.eval());
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast_nodes::{numbers::Int, operators::Op};

    use super::*;

    #[test]
    fn binding_definition() {
        assert_eq!(
            BindingDef::new("let test = 10 - 2"),
            Ok((
                "",
                BindingDef {
                    name: "test".to_string(),
                    val: Expr::Operation {
                        lhs: Int(10),
                        rhs: Int(2),
                        op: Op::Sub
                    }
                }
            ))
        )
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(
            BindingDef::new("letaaa=1+2"),
            Err("Expected a space".to_string()),
        );
    }
}
