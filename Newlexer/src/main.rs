mod lexer;
mod tokens;
use std::fs::File;
use std::io::BufReader;
use lexer::Lexer;

fn main() {
    // 打开测试文件
    let file = File::open("source.txt").expect("Failed to open tests.rs");
    let reader = BufReader::new(file);

    // 创建 Lexer
    let mut lexer = Lexer::new(reader);

    // 循环解析文件中的 Token
    println!("Tokens:");
    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::Token;

    #[test]
    fn test_lexer_simple_input() {
        let input = "let x = 42;"; // 测试输入字符串
        let mut lexer = Lexer::new(input.as_bytes()); // 使用字节流创建 Lexer

        // 检查解析的 Token 顺序是否正确
        assert_eq!(lexer.next_token(), Some(Token::Let)); // 关键字 let
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string()))); // 标识符 x
        assert_eq!(lexer.next_token(), Some(Token::Eq)); // 等号 =
        assert_eq!(lexer.next_token(), Some(Token::IntegerLiteral("42".to_string()))); // 整数字面量 42
        assert_eq!(lexer.next_token(), Some(Token::Semi)); // 分号 ;
        assert_eq!(lexer.next_token(), None); // 文件结束
    }

    #[test]
    fn test_lexer_with_comments() {
        let input = "let x = 42; // A comment";
        let mut lexer = Lexer::new(input.as_bytes());

        // 检查解析的 Token
        assert_eq!(lexer.next_token(), Some(Token::Let)); // 关键字 let
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string()))); // 标识符 x
        assert_eq!(lexer.next_token(), Some(Token::Eq)); // 等号 =
        assert_eq!(lexer.next_token(), Some(Token::IntegerLiteral("42".to_string()))); // 整数字面量 42
        assert_eq!(lexer.next_token(), Some(Token::Semi)); // 分号 ;
        assert_eq!(lexer.next_token(), Some(Token::Comment(" A comment".to_string()))); // 行注释
        assert_eq!(lexer.next_token(), None); // 文件结束
    }

    #[test]
    fn test_lexer_with_block_comment() {
        let input = "let x = 42; /* A block comment */ let y = 3.14;";
        let mut lexer = Lexer::new(input.as_bytes());

        // 检查解析的 Token
        assert_eq!(lexer.next_token(), Some(Token::Let)); // 关键字 let
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string()))); // 标识符 x
        assert_eq!(lexer.next_token(), Some(Token::Eq)); // 等号 =
        assert_eq!(lexer.next_token(), Some(Token::IntegerLiteral("42".to_string()))); // 整数字面量 42
        assert_eq!(lexer.next_token(), Some(Token::Semi)); // 分号 ;
        assert_eq!(lexer.next_token(), Some(Token::Comment(" A block comment ".to_string()))); // 块注释
        assert_eq!(lexer.next_token(), Some(Token::Let)); // 关键字 let
        assert_eq!(lexer.next_token(), Some(Token::Identifier("y".to_string()))); // 标识符 y
        assert_eq!(lexer.next_token(), Some(Token::Eq)); // 等号 =
        assert_eq!(lexer.next_token(), Some(Token::FloatLiteral("3.14".to_string()))); // 浮点数字面量 3.14
        assert_eq!(lexer.next_token(), Some(Token::Semi)); // 分号 ;
        assert_eq!(lexer.next_token(), None); // 文件结束
    }

    #[test]
    fn test_lexer_unterminated_block_comment() {
        let input = "let x = 42; /* Unterminated comment";
        let mut lexer = Lexer::new(input.as_bytes());

        // 检查解析的 Token
        assert_eq!(lexer.next_token(), Some(Token::Let)); // 关键字 let
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string()))); // 标识符 x
        assert_eq!(lexer.next_token(), Some(Token::Eq)); // 等号 =
        assert_eq!(lexer.next_token(), Some(Token::IntegerLiteral("42".to_string()))); // 整数字面量 42
        assert_eq!(lexer.next_token(), Some(Token::Semi)); // 分号 ;
        assert_eq!(lexer.next_token(), None); // 块注释未闭合，返回 None 或报错
    }
}

