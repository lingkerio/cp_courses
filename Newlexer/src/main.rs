// mod lexer;
// mod tokens;
// use lexer::Lexer;
// use std::fs::File;
// use std::io::BufReader;

// fn main() {
//     // 打开测试文件
//     let file = File::open("source.txt").expect("Failed to open tests.rs");
//     let reader = BufReader::new(file);

//     // 创建 Lexer
//     let mut lexer = Lexer::new(reader);

//     // 循环解析文件中的 Token
//     println!("Tokens:");
//     while let Some(token) = lexer.next_token() {
//         println!("{:?}", token);
//     }
//     println!("End of file");
// }

mod lexer;
mod tokens;
mod utils;

use utils::run_lexer_pipeline;

fn main() -> std::io::Result<()> {
    // 调用 utils 中的 pipeline 方法
    run_lexer_pipeline("source.txt")
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::Token;

    #[test]
    fn test_lexer_simple_input() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input.as_bytes());

        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Eq));
        assert_eq!(
            lexer.next_token(),
            Some(Token::IntegerLiteral("42".to_string()))
        );
        assert_eq!(lexer.next_token(), Some(Token::Semi));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_lexer_with_comments() {
        let input = "let x = 42; // A comment";
        let mut lexer = Lexer::new(input.as_bytes());

        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Eq));
        assert_eq!(
            lexer.next_token(),
            Some(Token::IntegerLiteral("42".to_string()))
        );
        assert_eq!(lexer.next_token(), Some(Token::Semi));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Comment(" A comment".to_string()))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_lexer_with_block_comment() {
        let input = "let x = 42; /* A block comment */ let y = 3.14;";
        let mut lexer = Lexer::new(input.as_bytes());

        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Eq));
        assert_eq!(
            lexer.next_token(),
            Some(Token::IntegerLiteral("42".to_string()))
        );
        assert_eq!(lexer.next_token(), Some(Token::Semi));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Comment(" A block comment ".to_string()))
        );
        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("y".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Eq));
        assert_eq!(
            lexer.next_token(),
            Some(Token::FloatLiteral("3.14".to_string()))
        );
        assert_eq!(lexer.next_token(), Some(Token::Semi));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_lexer_unterminated_block_comment() {
        let input = "let x = 42; /* Unterminated comment";
        let mut lexer = Lexer::new(input.as_bytes());

        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Eq));
        assert_eq!(
            lexer.next_token(),
            Some(Token::IntegerLiteral("42".to_string()))
        );
        assert_eq!(lexer.next_token(), Some(Token::Semi));
        assert_eq!(
            lexer.next_token(),
            Some(Token::Error("Unterminated block comment starting at 1:15".to_string()))
        );
    }

    #[test]
    fn test_valid_numbers() {
        let input = "123 0 42_42 3.14 0.1 42.42 1e10 6.02e-23 0.1e+2";
        let mut lexer = Lexer::new(input.as_bytes());

        assert_eq!(
            lexer.next_token(),
            Some(Token::IntegerLiteral("123".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::IntegerLiteral("0".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::IntegerLiteral("42_42".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::FloatLiteral("3.14".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::FloatLiteral("0.1".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::FloatLiteral("42.42".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::FloatLiteral("1e10".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::FloatLiteral("6.02e-23".to_string()))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::FloatLiteral("0.1e+2".to_string()))
        );
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_invalid_suffix() {
        let input = "23abc34 42.5abc";
        let mut lexer = Lexer::new(input.as_bytes());

        assert_eq!(
            lexer.next_token(),
            Some(Token::Error(
                "Invalid suffix 'abc34' for number literal starting at 1:1".to_string()
            ))
        );
        assert_eq!(
            lexer.next_token(),
            Some(Token::Error(
                "Invalid suffix 'abc' for number literal starting at 1:9".to_string()
            ))
        );
        assert_eq!(lexer.next_token(), None);
    }
}
