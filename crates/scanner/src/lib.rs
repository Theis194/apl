mod core;
mod modes;
mod position;
mod tokens;

use core::{Scanner, ScannerMode};
use position::Position;
use tokens::{Token, TokenType};

#[cfg(test)]
mod tests {
    use super::*;
    use apl_error::lexerror::*;

    // Helper to get errors from scanning
    fn scan_with_errors(source: &str) -> (Vec<Token>, Vec<LexError>) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        (tokens, scanner.errors)
    }

    // Helper to assert specific errors
    fn assert_has_error(
        errors: &[LexError],
        expected_type: LexErrorType,
        line: usize,
        column: usize,
    ) {
        assert!(
            errors
                .iter()
                .any(|e| e.error_type == expected_type && e.line == line && e.column == column),
            "Expected {:?} at {}:{}, but didn't find it in {:?}",
            expected_type,
            line,
            column,
            errors
        );
    }

    #[test]
    fn scan_variable_declaration_int() {
        let mut scanner = Scanner::new("let a = 10;");

        let tokens: Vec<TokenType> = scanner
            .scan_tokens()
            .iter()
            .map(|token| token.token_type.clone())
            .collect();
        assert_eq!(
            tokens,
            vec![
                TokenType::Let,
                TokenType::Identifier,
                TokenType::Equals,
                TokenType::Number
            ]
        )
    }

    #[test]
    fn scan_variable_declaration_float() {
        let mut scanner = Scanner::new("let a = 10.0;");

        let tokens: Vec<TokenType> = scanner
            .scan_tokens()
            .iter()
            .map(|token| token.token_type.clone())
            .collect();
        assert_eq!(
            tokens,
            vec![
                TokenType::Let,
                TokenType::Identifier,
                TokenType::Equals,
                TokenType::Number
            ]
        )
    }

    #[test]
    fn scan_variable_declaration_string_literal() {
        let mut scanner = Scanner::new(r#"let a = "test""#);

        let tokens: Vec<TokenType> = scanner
            .scan_tokens()
            .iter()
            .map(|token| token.token_type.clone())
            .collect();
        assert_eq!(
            tokens,
            vec![
                TokenType::Let,
                TokenType::Identifier,
                TokenType::Equals,
                TokenType::String("test".to_string())
            ]
        )
    }

    #[test]
    fn unexpected_character_error() {
        let source = "let a = @;";
        let (tokens, errors) = scan_with_errors(source);

        // Should still produce valid tokens despite error
        assert_eq!(
            tokens.iter().map(|t| &t.token_type).collect::<Vec<_>>(),
            vec![&TokenType::Let, &TokenType::Identifier, &TokenType::Equals,]
        );

        // Verify error
        assert_has_error(&errors, LexErrorType::UnexpectedCharacter('@'), 1, 10);
    }

    #[test]
    fn number_format_errors_multiple_decimal_points() {
        // Test multiple decimal points
        let (_, errors1) = scan_with_errors("123.45.67");
        assert_has_error(&errors1, LexErrorType::TooManyDecimalPoints, 1, 10);
    }

    #[test]
    fn number_format_errors_trailing_decimal_points() {
        // Test trailing decimal
        let (_, errors2) = scan_with_errors("42.");
        assert_has_error(&errors2, LexErrorType::TrailingDecimalPoint, 1, 4);
    }

    #[test]
    fn string_literal_errors() {
        // Unterminated string
        let (tokens, errors) = scan_with_errors(r#"let s = "unterminated"#);

        assert_eq!(
            tokens.last().unwrap().token_type,
            TokenType::String("unterminated".to_string())
        );
        assert_has_error(&errors, LexErrorType::UnterminatedString, 1, 22);

        // Invalid escape sequence
        let (_, errors) = scan_with_errors(r#""invalid \x escape""#);
        assert_has_error(&errors, LexErrorType::InvalidEscape('x'), 1, 12);
    }

    #[test]
    fn operator_errors() {
        // Single ! without =
        let (tokens, errors) = scan_with_errors("if !true");
        assert_eq!(
            tokens.iter().map(|t| &t.token_type).collect::<Vec<_>>(),
            vec![&TokenType::If, &TokenType::Bang, &TokenType::Identifier,]
        );
        assert!(errors.is_empty()); // This is actually valid syntax
    }

    #[test]
    fn error_positions_are_correct() {
        let source = r#"
let x = 123.45.67
let y = "unclosde"#;

        let (_, errors) = scan_with_errors(source);

        // First error (line 2)
        assert_has_error(&errors, LexErrorType::TooManyDecimalPoints, 2, 19);

        // Second error (line 3)
        assert_has_error(&errors, LexErrorType::UnterminatedString, 3, 19);
    }

    #[test]
    fn char_literals() {
        let mut scanner = Scanner::new("'a'");
        assert_eq!(
            scanner.scan_char_literal().unwrap().token_type,
            TokenType::Char('a')
        );

        let mut scanner = Scanner::new("'\\n'");
        assert_eq!(
            scanner.scan_char_literal().unwrap().token_type,
            TokenType::Char('\n')
        );

        // Error cases
        let mut scanner = Scanner::new("''");
        scanner.scan_char_literal();
        assert!(
            scanner
                .errors
                .iter()
                .any(|e| matches!(e.error_type, LexErrorType::EmptyCharLiteral))
        );

        let mut scanner = Scanner::new("'ab'");
        scanner.scan_char_literal();
        assert!(
            scanner
                .errors
                .iter()
                .any(|e| matches!(e.error_type, LexErrorType::TooManyChars))
        );
    }
}
