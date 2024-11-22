#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::Token;

    #[test]
    fn test_tokens() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Some(Token::As)); // Token::As 匹配成功
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Eq));
        assert_eq!(lexer.next_token(), Some(Token::IntegerLiteral("42".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Semi));
    }
}
