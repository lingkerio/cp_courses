mod lexer;
mod tokens;

use lexer::Lexer;

fn main() {
    let input = "\"Hello, World!\"";
    let mut lexer = Lexer::new(input);

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}
