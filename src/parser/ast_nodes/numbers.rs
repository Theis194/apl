use crate::utils::{extract_digits, extract_floats};

#[derive(Debug, PartialEq)]
pub struct Int(pub i32);

impl Int {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, digits) = extract_digits(s)?;
        Ok((s, Self(digits.parse().unwrap())))
    }
}


#[derive(Debug, PartialEq)]
pub struct Float(pub f32);

impl Float {
    pub fn new(s: &str) -> Result<(&str, Float), String> {
        let (s, digits) = extract_floats(s)?;
        Ok((s, Self(digits.parse().unwrap())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() {
        assert_eq!(Int::new("123"), Ok(("", Int(123))));
    }

    #[test]
    fn parse_float() {
        assert_eq!(Float::new("12.3"), Ok(("", Float(12.3))));
    }
}
