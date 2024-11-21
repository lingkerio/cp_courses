use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::time::Instant;

/// Token types for the Rust lexer.
#[derive(Debug, PartialEq, Eq, Clone)]
enum TokenType {
    // Keywords
    As,
    Break,
    Const,
    Continue,
    Crate,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    SELF,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Unsafe,
    Use,
    Where,
    While,
    // 2018+ keywords
    Async,
    Await,
    Dyn,
    // Reserved keywords
    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,
    // 2018+ reserved keywords
    Try,
    // Weak keywords
    MacroRules,
    Union,
    StaticLifetime,

    /// Identifiers
    /// An identifier is a sequence of characters that starts with a letter or
    /// underscore and can be followed by alphanumeric characters or underscores.
    /// Identifiers are used to name variables, functions, types, and other entities.
    /// Identifiers cannot be keywords.
    /// Examples: `foo`, `_bar`, `baz123`
    // Identifiers
    Identifier(String),
    // Literals
    CharLiteral(char),
    StringLiteral(String),
    IntegerLiteral(String),
    FloatLiteral(String),
    // Lifetimes and Labels
    LifetimeOrLabel(String),
    // Comments
    Comment(String),
    // Whitespace
    Whitespace,
    // custom
    Dereference,
    Error(usize, usize, String),
    // Punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Not,
    And,
    Or,
    AndAnd,
    OrOr,
    Shl,
    Shr,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    CaretEq,
    AndEq,
    OrEq,
    ShlEq,
    ShrEq,
    Eq,
    EqEq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    At,
    Underscore,
    Dot,
    DotDot,
    DotDotDot,
    DotDotEq,
    Comma,
    Semi,
    Colon,
    PathSep,
    RArrow,
    FatArrow,
    LArrow,
    Pound,
    Dollar,
    Question,
    Tilde,
    // Delimiters
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    // Unknown
    // Unknown(char),
}

/// The Lexer struct manages the input buffers and tokenization process.
struct Lexer<R: Read> {
    reader: BufReader<R>,
    buffer_size: usize,
    buffer: [u8; 8192], // Double buffer (2 * 4096)
    lexeme_start: usize,
    lexeme_forward: usize,
    prev_token: Option<TokenType>,
    eof: bool,
    row: usize,
    col: usize,
}

impl<R: Read> Lexer<R> {
    /// Creates a new Lexer instance with a double buffer.
    fn new(reader: R) -> Self {
        Self {
            reader: BufReader::with_capacity(8190, reader),
            buffer_size: 4096,
            buffer: [0; 8192],
            lexeme_start: 0,
            lexeme_forward: 0,
            prev_token: None,
            eof: false,
            row: 1,
            col: 0,
        }
    }

    fn init(&mut self) -> io::Result<()> {
        let buffer_offset = 0;
        self.reader
            .read(&mut self.buffer[buffer_offset..buffer_offset + self.buffer_size - 1])?;
        Ok(())
    }

    /// Loads the next buffer into the double buffer setup.
    fn load_buffer(&mut self) -> io::Result<()> {
        if self.eof {
            return Ok(());
        }

        let buffer_offset = if self.lexeme_forward >= self.buffer_size {
            0
        } else {
            self.buffer_size
        };

        let bytes_read = self
            .reader
            .read(&mut self.buffer[buffer_offset..buffer_offset + self.buffer_size - 1])?;
        self.buffer[buffer_offset + bytes_read] = 0;
        if bytes_read < self.buffer_size - 1 {
            // Insert sentinel
            self.buffer[buffer_offset + bytes_read] = 0;
            self.eof = true;
        }

        Ok(())
    }

    /// Advances the forward pointer and returns the current character.
    fn next_char(&mut self) -> Option<char> {
        let c = self.buffer[self.lexeme_forward];

        if c == 0 {
            if self.eof {
                return None;
            } else {
                self.load_buffer().unwrap();
                self.consume_char();
                return self.next_char();
            }
        } else {
            self.lexeme_forward += 1;
            self.lexeme_forward %= self.buffer_size * 2;
            return Some(c as char);
        }
    }

    /// Peeks at the current character without consuming it.
    fn peek_char(&mut self) -> Option<char> {
        if self.lexeme_forward == self.buffer_size * 2 {
            return None;
        }
        let c = self.buffer[self.lexeme_forward];
        if c == 0 {
            if self.eof {
                return None;
            } else {
                self.load_buffer().unwrap();
                self.consume_char();
                return self.peek_char();
            }
        }
        Some(c as char)
    }

    /// Consumes the current character, moving the forward pointer.
    fn consume_char(&mut self) {
        if self.lexeme_forward == self.buffer_size * 2 {
            self.lexeme_forward = 0;
        }
        self.lexeme_forward += 1;
        self.lexeme_forward %= self.buffer_size * 2;
    }

    /// Sets the lexeme_start to the current forward position.
    fn reset_lexeme_start(&mut self) {
        if self.lexeme_forward < self.lexeme_start {
            for i in self.lexeme_start..self.buffer_size * 2 {
                if self.buffer[i] == '\n' as u8 {
                    self.row += 1;
                    self.col = 0;
                } else {
                    self.col += 1;
                }
            }
            for i in 0..self.lexeme_forward {
                if self.buffer[i] == '\n' as u8 {
                    self.row += 1;
                    self.col = 0;
                } else {
                    self.col += 1;
                }
            }
        } else {
            for i in self.lexeme_start..self.lexeme_forward {
                if self.buffer[i] == '\n' as u8 {
                    self.row += 1;
                    self.col = 0;
                } else {
                    self.col += 1;
                }
            }
        }
        self.lexeme_start = self.lexeme_forward;
    }

    /// Checks if a character is a valid start for an identifier.
    fn is_identifier_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    /// Checks if a character is valid for continuing an identifier.
    fn is_identifier_continue(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn next_token(&mut self) -> Option<TokenType> {
        let result = loop {
            self.reset_lexeme_start();
            let c = self.next_char()?;

            // Skip whitespace
            if c.is_whitespace() {
                while let Some(c) = self.peek_char() {
                    if !c.is_whitespace() {
                        break;
                    }
                    self.consume_char();
                }
                break Some(TokenType::Whitespace);
            }

            // Handle slash
            if c == '/' {
                let next_char = self.peek_char()?;
                if next_char == '/' {
                    // Line comment
                    self.consume_char();
                    while let Some(c) = self.next_char() {
                        if c == '\n' {
                            break;
                        }
                    }
                    break Some(TokenType::Comment("".to_string()));
                } else if next_char == '*' {
                    // Block comment
                    self.consume_char();
                    while let Some(c) = self.next_char() {
                        if c == '*' {
                            if self.peek_char()? == '/' {
                                self.consume_char();
                                break;
                            }
                        }
                    }
                    break Some(TokenType::Comment("".to_string()));
                } else if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::SlashEq);
                } else {
                    break Some(TokenType::Slash);
                }
            }

            // Handle star
            if c == '*' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::StarEq);
                } else if let Some(TokenType::Identifier(_)) = self.prev_token {
                    break Some(TokenType::Star);
                } else {
                    break Some(TokenType::Dereference);
                }
            }

            // Handle plus
            if c == '+' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::PlusEq);
                } else {
                    break Some(TokenType::Plus);
                }
            }

            // Handle minus
            if c == '-' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::MinusEq);
                }
                if next_char == '>' {
                    self.consume_char();
                    break Some(TokenType::RArrow);
                } else {
                    break Some(TokenType::Minus);
                }
            }

            // Handle percent
            if c == '%' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::PercentEq);
                } else {
                    break Some(TokenType::Percent);
                }
            }

            // Handle caret
            if c == '^' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::CaretEq);
                } else {
                    break Some(TokenType::Caret);
                }
            }

            // Handle ampersand
            if c == '&' {
                let next_char = self.peek_char()?;
                if next_char == '&' {
                    self.consume_char();
                    break Some(TokenType::AndAnd);
                } else if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::AndEq);
                } else {
                    break Some(TokenType::And);
                }
            }

            // Handle pipe
            if c == '|' {
                let next_char = self.peek_char()?;
                if next_char == '|' {
                    self.consume_char();
                    break Some(TokenType::OrOr);
                } else if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::OrEq);
                } else {
                    break Some(TokenType::Or);
                }
            }

            // Handle exclamation
            if c == '!' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::Ne);
                } else {
                    break Some(TokenType::Not);
                }
            }

            // Handle less than
            if c == '<' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::Le);
                } else if next_char == '<' {
                    self.consume_char();
                    let next_char = self.peek_char()?;
                    if next_char == '=' {
                        self.consume_char();
                        break Some(TokenType::ShlEq);
                    } else {
                        break Some(TokenType::Shl);
                    }
                } else if next_char == '-' {
                    self.consume_char();
                    break Some(TokenType::LArrow);
                } else {
                    break Some(TokenType::Lt);
                }
            }

            // Handle greater than
            if c == '>' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::Ge);
                } else if next_char == '>' {
                    self.consume_char();
                    let next_char = self.peek_char()?;
                    if next_char == '=' {
                        self.consume_char();
                        break Some(TokenType::ShrEq);
                    } else {
                        break Some(TokenType::Shr);
                    }
                } else {
                    break Some(TokenType::Gt);
                }
            }

            // Handle equals
            if c == '=' {
                let next_char = self.peek_char()?;
                if next_char == '=' {
                    self.consume_char();
                    break Some(TokenType::EqEq);
                } else if next_char == '>' {
                    self.consume_char();
                    break Some(TokenType::FatArrow);
                } else {
                    break Some(TokenType::Eq);
                }
            }

            // Handle at
            if c == '@' {
                break Some(TokenType::At);
            }

            // Handle underscore
            if c == '_' {
                break Some(TokenType::Underscore);
            }

            // Handle dot
            if c == '.' {
                let next_char = self.peek_char()?;
                if next_char == '.' {
                    self.consume_char();
                    let next_char = self.peek_char()?;
                    if next_char == '.' {
                        self.consume_char();
                        break Some(TokenType::DotDotDot);
                    } else if next_char == '=' {
                        self.consume_char();
                        break Some(TokenType::DotDotEq);
                    } else {
                        break Some(TokenType::DotDot);
                    }
                } else {
                    break Some(TokenType::Dot);
                }
            }

            // Handle comma
            if c == ',' {
                break Some(TokenType::Comma);
            }

            // Handle semicolon
            if c == ';' {
                break Some(TokenType::Semi);
            }

            // Handle colon
            if c == ':' {
                let next_char = self.peek_char()?;

                if next_char == ':' {
                    self.consume_char();
                    break Some(TokenType::PathSep);
                } else {
                    break Some(TokenType::Colon);
                }
            }

            // Handle pound
            if c == '#' {
                break Some(TokenType::Pound);
            }

            // Handle dollar
            if c == '$' {
                break Some(TokenType::Dollar);
            }

            // Handle question mark
            if c == '?' {
                break Some(TokenType::Question);
            }

            // Handle tilde
            if c == '~' {
                break Some(TokenType::Tilde);
            }

            // Handle delimiters
            match c {
                '{' => break Some(TokenType::OpenBrace),
                '}' => break Some(TokenType::CloseBrace),
                '[' => break Some(TokenType::OpenBracket),
                ']' => break Some(TokenType::CloseBracket),
                '(' => break Some(TokenType::OpenParen),
                ')' => break Some(TokenType::CloseParen),
                _ => {}
            }

            // Handle keywords and identifiers
            if Self::is_identifier_start(c) {
                let mut ident = String::new();
                ident.push(c);

                while let Some(c) = self.peek_char() {
                    if Self::is_identifier_continue(c) {
                        ident.push(c);
                        self.consume_char();
                    } else {
                        break;
                    }
                }

                if c.is_ascii_punctuation() {
                    break Some(TokenType::Error(
                        self.row,
                        self.col,
                        String::from("Error in Identifier"),
                    ));
                }

                match ident.as_str() {
                    "as" => break Some(TokenType::As),
                    "break" => break Some(TokenType::Break),
                    "const" => break Some(TokenType::Const),
                    "continue" => break Some(TokenType::Continue),
                    "crate" => break Some(TokenType::Crate),
                    "else" => break Some(TokenType::Else),
                    "enum" => break Some(TokenType::Enum),
                    "extern" => break Some(TokenType::Extern),
                    "false" => break Some(TokenType::False),
                    "fn" => break Some(TokenType::Fn),
                    "for" => break Some(TokenType::For),
                    "if" => break Some(TokenType::If),
                    "impl" => break Some(TokenType::Impl),
                    "in" => break Some(TokenType::In),
                    "let" => break Some(TokenType::Let),
                    "loop" => break Some(TokenType::Loop),
                    "match" => break Some(TokenType::Match),
                    "mod" => break Some(TokenType::Mod),
                    "move" => break Some(TokenType::Move),
                    "mut" => break Some(TokenType::Mut),
                    "pub" => break Some(TokenType::Pub),
                    "ref" => break Some(TokenType::Ref),
                    "return" => break Some(TokenType::Return),
                    "self" => break Some(TokenType::SELF),
                    "static" => break Some(TokenType::Static),
                    "struct" => break Some(TokenType::Struct),
                    "super" => break Some(TokenType::Super),
                    "trait" => break Some(TokenType::Trait),
                    "true" => break Some(TokenType::True),
                    "type" => break Some(TokenType::Type),
                    "unsafe" => break Some(TokenType::Unsafe),
                    "use" => break Some(TokenType::Use),
                    "where" => break Some(TokenType::Where),
                    "while" => break Some(TokenType::While),
                    "async" => break Some(TokenType::Async),
                    "await" => break Some(TokenType::Await),
                    "dyn" => break Some(TokenType::Dyn),
                    "abstract" => break Some(TokenType::Abstract),
                    "become" => break Some(TokenType::Become),
                    "box" => break Some(TokenType::Box),
                    "do" => break Some(TokenType::Do),
                    "final" => break Some(TokenType::Final),
                    "macro" => break Some(TokenType::Macro),
                    "override" => break Some(TokenType::Override),
                    "priv" => break Some(TokenType::Priv),
                    "typeof" => break Some(TokenType::Typeof),
                    "unsized" => break Some(TokenType::Unsized),
                    "virtual" => break Some(TokenType::Virtual),
                    "yield" => break Some(TokenType::Yield),
                    "try" => break Some(TokenType::Try),
                    "macro_rules" => break Some(TokenType::MacroRules),
                    "union" => break Some(TokenType::Union),
                    _ => break Some(TokenType::Identifier(ident)),
                };
            }

            // Handle '
            if c == '\'' {
                let next_char = self.next_char()?;

                if next_char == '\\' {
                    let curr_char = self.next_char()?;
                    let next_char = self.next_char()?;
                    if next_char == '\'' {
                        if curr_char == 'n' {
                            break Some(TokenType::CharLiteral('\n'));
                        } else if curr_char == 'r' {
                            break Some(TokenType::CharLiteral('\r'));
                        } else if curr_char == 't' {
                            break Some(TokenType::CharLiteral('\t'));
                        } else if curr_char == '\\' {
                            break Some(TokenType::CharLiteral('\\'));
                        } else if curr_char == '\'' {
                            break Some(TokenType::CharLiteral('\''));
                        } else if curr_char == '\"' {
                            break Some(TokenType::CharLiteral('\"'));
                        } else {
                            break Some(TokenType::Error(
                                self.row,
                                self.col,
                                String::from("Error in char literal, invalid escape sequence"),
                            ));
                        }
                    }

                    break Some(TokenType::Error(
                        self.row,
                        self.col,
                        String::from("Error in char literal: multiple characters"),
                    ));
                } else if next_char == 's'
                    && self.next_char()? == 't'
                    && self.next_char()? == 'a'
                    && self.next_char()? == 't'
                    && self.next_char()? == 'i'
                    && self.next_char()? == 'c'
                    && !self.next_char()?.is_alphabetic()
                {
                    break Some(TokenType::StaticLifetime);
                } else {
                    self.lexeme_forward = self.lexeme_start;
                    self.consume_char();
                    let mut ident = String::new();

                    let mut next_char = self.next_char()?;
                    while next_char != '\'' && !next_char.is_whitespace() {
                        ident.push(next_char);
                        next_char = self.next_char()?;
                    }

                    if next_char.is_whitespace() {
                        if self.prev_token == Some(TokenType::Lt) {
                            self.lexeme_forward -= 2;
                            ident = ident[..ident.len() - 1].to_string();
                            break Some(TokenType::LifetimeOrLabel(ident));
                        } else {
                            break Some(TokenType::Error(
                                self.row,
                                self.col,
                                String::from("Error in char literal: No Closing Quote"),
                            ));
                        }
                    } else if ident.is_empty() {
                        break Some(TokenType::Error(
                            self.row,
                            self.col,
                            String::from("Error in char literal: empty literal"),
                        ));
                    } else if ident.len() == 1 {
                        break Some(TokenType::CharLiteral(ident.chars().next().unwrap()));
                    } else if ident.len() > 1 {
                        break Some(TokenType::Error(
                            self.row,
                            self.col,
                            String::from("Error in char literal: multiple characters"),
                        ));
                    }

                    if next_char.is_ascii_punctuation() {
                        break Some(TokenType::Error(
                            self.row,
                            self.col,
                            String::from("Error in LifetimeOrLabel literal"),
                        ));
                    }
                }
            }

            // Handle Stirng Literal
            if c == '"' {
                let mut string = String::new();
                
                let mut next_char = self.next_char();
                while let Some(_) = next_char {
                    if next_char.unwrap() == '"' {
                        break;
                    }
                    // handle escape sequences
                    if next_char.unwrap() == '\\' {
                        
                        next_char = self.next_char();
                    }
                    string.push(next_char.unwrap());
                    
                    next_char = self.next_char();
                }
                if next_char.is_none() {
                    break Some(TokenType::Error(
                        self.row,
                        self.col,
                        String::from("Error in String Literal: No Closing Quote"),
                    ));
                }
                break Some(TokenType::StringLiteral(string));
            }

            // Handle Integer Or Float Literal
            if c.is_digit(10) {
                let mut number = String::new();
                number.push(c);

                // Continue pushing characters to `number` until whitespace or non-`.` punctuation is encountered.
                loop {
                    let next_char = self.peek_char()?;

                    if next_char.is_whitespace()
                        || (next_char.is_ascii_punctuation()
                            && next_char != '.'
                            && next_char != '.')
                    {
                        break;
                    }

                    number.push(next_char);
                    self.consume_char();
                }

                // Check the contents of `number` to determine the token type
                if number.contains('.') {
                    let dot_count = number.matches('.').count();
                    let has_alpha = number.chars().any(|ch| ch.is_alphabetic());

                    if dot_count == 1 && !has_alpha {
                        break Some(TokenType::FloatLiteral(number));
                    } else {
                        if number.contains("..") {
                            // Handle integer range like 0..5, return the first integer
                            let parts: Vec<&str> = number.split("..").collect();
                            if parts.len() == 2 && parts[0].chars().all(|ch| ch.is_digit(10)) {
                                self.lexeme_forward = self.lexeme_start + parts[0].len();
                                break Some(TokenType::IntegerLiteral(parts[0].to_string()));
                            } else {
                                break Some(TokenType::Error(
                                    self.row,
                                    self.col,
                                    String::from("Error in Integer Range"),
                                ));
                            }
                        } else {
                            break Some(TokenType::Error(
                                self.row,
                                self.col,
                                String::from("Error in Float Literal"),
                            ));
                        }
                    }
                } else {
                    let has_alpha = number.chars().any(|ch| ch.is_alphabetic());

                    if has_alpha {
                        if self.prev_token == Some(TokenType::Let) {
                            break Some(TokenType::Error(
                                self.row,
                                self.col,
                                String::from("Error in identifier starting with number"),
                            ));
                        } else {
                            break Some(TokenType::Error(
                                self.row,
                                self.col,
                                String::from("Error in Integer Literal with alphabets"),
                            ));
                        }
                    } else {
                        break Some(TokenType::IntegerLiteral(number));
                    }
                }
            }

            break Some(TokenType::Error(
                self.row,
                self.col,
                String::from("Unknown Token"),
            ));
        };

        if !matches!(result, Some(TokenType::Whitespace)) {
            self.prev_token = result.clone();
        }
        result
    }

    pub fn parse_all(&mut self) -> io::Result<ParseResult> {
        let mut identifier_table: HashMap<String, usize> = HashMap::new();
        let mut char_literal_table: HashMap<char, usize> = HashMap::new();
        let mut string_literal_table: HashMap<String, usize> = HashMap::new();
        let mut integer_literal_table: HashMap<String, usize> = HashMap::new();
        let mut float_literal_table: HashMap<String, usize> = HashMap::new();

        let mut identifier_next_id = 1;
        let mut char_literal_next_id = 1;
        let mut string_literal_next_id = 1;
        let mut integer_literal_next_id = 1;
        let mut float_literal_next_id = 1;

        let mut token_stream = Vec::new();

        while let Some(token) = self.next_token() {
            let token_output = match token {
                TokenType::Identifier(ident) => {
                    let id = identifier_table.entry(ident.clone()).or_insert_with(|| {
                        let id = identifier_next_id;
                        identifier_next_id += 1;
                        id
                    });
                    format!("Identifier({})", id)
                }
                TokenType::CharLiteral(ch) => {
                    let id = char_literal_table.entry(ch).or_insert_with(|| {
                        let id = char_literal_next_id;
                        char_literal_next_id += 1;
                        id
                    });
                    format!("CharLiteral({})", id)
                }
                TokenType::StringLiteral(s) => {
                    let id = string_literal_table.entry(s.clone()).or_insert_with(|| {
                        let id = string_literal_next_id;
                        string_literal_next_id += 1;
                        id
                    });
                    format!("StringLiteral({})", id)
                }
                TokenType::IntegerLiteral(i) => {
                    let id = integer_literal_table.entry(i.clone()).or_insert_with(|| {
                        let id = integer_literal_next_id;
                        integer_literal_next_id += 1;
                        id
                    });
                    format!("IntegerLiteral({})", id)
                }
                TokenType::FloatLiteral(f) => {
                    let id = float_literal_table.entry(f.clone()).or_insert_with(|| {
                        let id = float_literal_next_id;
                        float_literal_next_id += 1;
                        id
                    });
                    format!("FloatLiteral({})", id)
                }
                TokenType::Whitespace | TokenType::Comment(_) => continue,
                _ => format!("{:?}", token),
            };

            // 将生成的 Token 内容添加到 token_stream 中
            token_stream.push(token_output);
        }

        Ok(ParseResult {
            identifier_table,
            char_literal_table,
            string_literal_table,
            integer_literal_table,
            float_literal_table,
            token_stream,
        })
    }
}

struct TableWriter {
    writer: BufWriter<File>,
}

impl TableWriter {
    fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            writer: BufWriter::new(File::create(path)?),
        })
    }

    fn write_entry<T: std::fmt::Display>(&mut self, id: usize, value: T) {
        writeln!(self.writer, "{}: {}", id, value).unwrap();
    }
}

fn process_literal<T: Eq + Hash, F: FnOnce() -> String>(
    table: &mut HashMap<T, usize>,
    next_id: &mut usize,
    writer: &mut TableWriter,
    value: T,
    format_value: F,
) -> usize {
    *table.entry(value).or_insert_with(|| {
        let id = *next_id;
        *next_id += 1;
        writer.write_entry(id, format_value());
        id
    })
}

struct ParseResult {
    identifier_table: HashMap<String, usize>,
    char_literal_table: HashMap<char, usize>,
    string_literal_table: HashMap<String, usize>,
    integer_literal_table: HashMap<String, usize>,
    float_literal_table: HashMap<String, usize>,
    token_stream: Vec<String>, // 解析后的单词串内容
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let input_file = File::open("source.txt")?;
    let mut lexer = Lexer::new(input_file);
    lexer.init()?;

    let mut identifier_table: HashMap<String, usize> = HashMap::new();
    let mut char_literal_table: HashMap<char, usize> = HashMap::new();
    let mut string_literal_table: HashMap<String, usize> = HashMap::new();
    let mut integer_literal_table: HashMap<String, usize> = HashMap::new();
    let mut float_literal_table: HashMap<String, usize> = HashMap::new();

    let mut identifier_next_id = 1;
    let mut char_literal_next_id = 1;
    let mut string_literal_next_id = 1;
    let mut integer_literal_next_id = 1;
    let mut float_literal_next_id = 1;

    let mut identifier_writer = TableWriter::new("identifier_table.txt")?;
    let mut char_literal_writer = TableWriter::new("char_literal_table.txt")?;
    let mut string_literal_writer = TableWriter::new("string_literal_table.txt")?;
    let mut integer_literal_writer = TableWriter::new("integer_literal_table.txt")?;
    let mut float_literal_writer = TableWriter::new("float_literal_table.txt")?;

    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::with_capacity(64 * 1024, output_file);

    while let Some(token) = lexer.next_token() {
        let token_output = match token {
            TokenType::Identifier(ref ident) => {
                let id = process_literal(
                    &mut identifier_table,
                    &mut identifier_next_id,
                    &mut identifier_writer,
                    ident.clone(),
                    || ident.clone(),
                );
                format!("Identifier({})", id)
            }
            TokenType::CharLiteral(ref ch) => {
                let id = process_literal(
                    &mut char_literal_table,
                    &mut char_literal_next_id,
                    &mut char_literal_writer,
                    *ch,
                    || match ch {
                        '\n' => "\\n".to_string(),
                        '\r' => "\\r".to_string(),
                        '\t' => "\\t".to_string(),
                        '\\' => "\\\\".to_string(),
                        '\'' => "\\'".to_string(),
                        '\"' => "\\\"".to_string(),
                        _ => ch.to_string(),
                    },
                );
                format!("CharLiteral({})", id)
            }
            TokenType::StringLiteral(ref s) => {
                let id = process_literal(
                    &mut string_literal_table,
                    &mut string_literal_next_id,
                    &mut string_literal_writer,
                    s.clone(),
                    || s.clone(),
                );
                format!("StringLiteral({})", id)
            }
            TokenType::IntegerLiteral(ref i) => {
                let id = process_literal(
                    &mut integer_literal_table,
                    &mut integer_literal_next_id,
                    &mut integer_literal_writer,
                    i.clone(),
                    || i.clone(),
                );
                format!("IntegerLiteral({})", id)
            }
            TokenType::FloatLiteral(ref f) => {
                let id = process_literal(
                    &mut float_literal_table,
                    &mut float_literal_next_id,
                    &mut float_literal_writer,
                    f.clone(),
                    || f.clone(),
                );
                format!("FloatLiteral({})", id)
            }
            TokenType::Whitespace | TokenType::Comment(_) => continue,
            _ => format!("{:?}", token),
        };

        writeln!(writer, "{}", token_output)?;
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    Ok(())
}

// fn main() -> io::Result<()> {
//     let input_file = File::open("source.txt")?;
//     let mut lexer = Lexer::new(input_file);

//     let result = lexer.parse_all()?;

//     println!("Identifier Table: {:?}", result.identifier_table);
//     println!("Char Literal Table: {:?}", result.char_literal_table);
//     println!("String Literal Table: {:?}", result.string_literal_table);
//     println!("Integer Literal Table: {:?}", result.integer_literal_table);
//     println!("Float Literal Table: {:?}", result.float_literal_table);


//     println!("Token Stream:");
//     for token in &result.token_stream {
//         println!("{}", token);
//     }


//     let output_file = File::create("token_stream.txt")?;
//     let mut writer = BufWriter::new(output_file);
//     for token in result.token_stream {
//         writeln!(writer, "{}", token)?;
//     }

//     Ok(())
// }
