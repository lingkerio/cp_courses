use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i32),
    Operator(char),
    LParen,
    RParen,
}

struct Lexer {
    input_string: String,
    position: usize,
}

impl Lexer {
    fn new(input_string: String) -> Self {
        Self {
            input_string,
            position: 0,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input_string.len() {
            return None;
        }

        let current_char = self.input_string.chars().nth(self.position).unwrap();

        // 跳过空白字符
        if current_char.is_whitespace() {
            self.position += 1;
            return self.next_token();
        }

        // 处理数字
        if current_char.is_digit(10) {
            let re = Regex::new(r"^\d+").unwrap();
            let remaining_str = &self.input_string[self.position..];
            if let Some(mat) = re.find(remaining_str) {
                let number: i32 = mat.as_str().parse().unwrap();
                self.position += mat.end();
                return Some(Token::Number(number));
            }
        }

        // 处理操作符和括号
        if "+-*/()".contains(current_char) {
            self.position += 1;
            return match current_char {
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                _ => Some(Token::Operator(current_char)),
            };
        }

        panic!("Unexpected character: {}", current_char);
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn parse(&mut self) -> Box<Expr> {
        self.expression()
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self, expected: &Token) -> Token {
        let token = self.current_token().cloned();
        assert!(token.is_some(), "Expected token {:?}, but got None", expected);
        let token = token.unwrap();
        if &token == expected {
            self.position += 1;
            token
        } else {
            panic!("Expected token {:?}, but got {:?}", expected, token);
        }
    }

    fn expression(&mut self) -> Box<Expr> {
        let mut node = self.term();
        while let Some(Token::Operator(op)) = self.current_token() {
            if *op == '+' || *op == '-' {
                let op = self.consume(&Token::Operator(*op));
                let right_node = self.term();
                node = Box::new(Expr::Binary(op, node, right_node));
            } else {
                break;
            }
        }
        node
    }

    fn term(&mut self) -> Box<Expr> {
        let mut node = self.factor();
        while let Some(Token::Operator(op)) = self.current_token() {
            if *op == '*' || *op == '/' {
                let op = self.consume(&Token::Operator(*op));
                let right_node = self.factor();
                node = Box::new(Expr::Binary(op, node, right_node));
            } else {
                break;
            }
        }
        node
    }

    fn factor(&mut self) -> Box<Expr> {
        let token = self.current_token().cloned();
        assert!(token.is_some(), "Expected a token, but got None");
        let token = token.unwrap();
        match token {
            Token::Number(value) => {
                self.consume(&Token::Number(value));
                Box::new(Expr::Number(value))
            }
            Token::LParen => {
                self.consume(&Token::LParen);
                let node = self.expression();
                self.consume(&Token::RParen);
                node
            }
            _ => panic!("Invalid syntax"),
        }
    }
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    Binary(Token, Box<Expr>, Box<Expr>),
}

fn evaluate(node: &Expr) -> i32 {
    match node {
        Expr::Number(value) => *value,
        Expr::Binary(Token::Operator(op), left, right) => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);
            match op {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                '*' => left_val * right_val,
                '/' => left_val / right_val,
                _ => panic!("Unexpected operator"),
            }
        }
        _ => panic!("Unexpected expression"),
    }
}

fn calculate(input_string: String) -> i32 {
    let mut lexer = Lexer::new(input_string);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    evaluate(&ast)
}

fn main() {
    let result = calculate("3 + 5 * (2 - 8)".to_string());
    println!("{}", result); // 输出计算结果
}