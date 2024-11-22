use std::io::Read;

use crate::tokens::Token;

pub struct Lexer<R: Read> {
    reader: R,             // 文件流，减少系统调用次数
    buffers: [Vec<u8>; 2], // 双缓冲区
    current_buffer: usize, // 当前缓冲区索引
    position: usize,       // 当前缓冲区内的位置
    eof: bool,             // 文件是否已结束
}

impl<R: Read> Lexer<R> {
    const PAGESIZE: usize = 4096; // 每个缓冲区的大小

    pub fn new(reader: R) -> Self {
        let mut lexer = Self {
            reader,
            buffers: [vec![0; Self::PAGESIZE + 1], vec![0; Self::PAGESIZE + 1]],
            current_buffer: 0,
            position: 0,
            eof: false,
        };

        // 初始化第一个缓冲区
        lexer.fill_buffer(lexer.current_buffer).unwrap();
        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // 跳过空白符
        self.skip_whitespace();

        // 当前字符
        let current = self.peek_char()?;

        // 根据当前字符决定 Token 类型
        let token = match current {
            // 处理单字符符号
            '+' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::PlusEq)
                    }
                    _ => Some(Token::Plus),
                }
            }
            '-' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::MinusEq)
                    }
                    Some('>') => {
                        self.advance();
                        Some(Token::RArrow)
                    }
                    _ => Some(Token::Minus),
                }
            }
            '*' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::StarEq)
                    }
                    _ => Some(Token::Star),
                }
            }
            '/' => {
                self.advance(); // Skip the initial '/'
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::SlashEq)
                    }
                    Some('/') => {
                        self.advance(); // Skip the second '/'
                        self.read_line_comment()
                    }
                    Some('*') => {
                        self.advance(); // Skip the '*'
                        self.read_block_comment()
                    }
                    _ => Some(Token::Slash), // Single slash
                }
            }
            '%' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::PercentEq)
                    }
                    _ => Some(Token::Percent),
                }
            }
            '^' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::CaretEq)
                    }
                    _ => Some(Token::Caret),
                }
            }
            '!' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::Ne)
                    }
                    _ => Some(Token::Not),
                }
            }
            '&' => {
                self.advance();
                match self.peek_char() {
                    Some('&') => {
                        self.advance();
                        Some(Token::AndAnd)
                    }
                    Some('=') => {
                        self.advance();
                        Some(Token::AndEq)
                    }
                    _ => Some(Token::And),
                }
            }
            '|' => {
                self.advance();
                match self.peek_char() {
                    Some('|') => {
                        self.advance();
                        Some(Token::OrOr)
                    }
                    Some('=') => {
                        self.advance();
                        Some(Token::OrEq)
                    }
                    _ => Some(Token::Or),
                }
            }
            '=' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::EqEq)
                    }
                    Some('>') => {
                        self.advance();
                        Some(Token::FatArrow)
                    }
                    _ => Some(Token::Eq),
                }
            }
            '<' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::Le)
                    }
                    Some('<') => {
                        self.advance();
                        match self.peek_char() {
                            Some('=') => {
                                self.advance();
                                Some(Token::ShlEq)
                            }
                            _ => Some(Token::Shl),
                        }
                    }
                    Some('-') => {
                        self.advance();
                        Some(Token::LArrow)
                    }
                    _ => Some(Token::Lt),
                }
            }
            '>' => {
                self.advance();
                match self.peek_char() {
                    Some('=') => {
                        self.advance();
                        Some(Token::Ge)
                    }
                    Some('>') => {
                        self.advance();
                        match self.peek_char() {
                            Some('=') => {
                                self.advance();
                                Some(Token::ShrEq)
                            }
                            _ => Some(Token::Shr),
                        }
                    }
                    _ => Some(Token::Gt),
                }
            }
            '.' => {
                self.advance();
                match self.peek_char() {
                    Some('.') => {
                        self.advance();
                        match self.peek_char() {
                            Some('=') => {
                                self.advance();
                                Some(Token::DotDotEq)
                            }
                            Some('.') => {
                                self.advance();
                                Some(Token::DotDotDot)
                            }
                            _ => Some(Token::DotDot),
                        }
                    }
                    Some('0'..='9') => self.read_float_starting_with_dot(),
                    _ => Some(Token::Dot),
                }
            }
            '@' => {
                self.advance();
                Some(Token::At)
            }
            '_' => {
                self.advance();
                Some(Token::Underscore)
            }
            ',' => {
                self.advance();
                Some(Token::Comma)
            }
            ';' => {
                self.advance();
                Some(Token::Semi)
            }
            ':' => {
                self.advance();
                match self.peek_char() {
                    Some(':') => {
                        self.advance();
                        Some(Token::PathSep)
                    }
                    _ => Some(Token::Colon),
                }
            }
            '#' => {
                self.advance();
                Some(Token::Pound)
            }
            '$' => {
                self.advance();
                Some(Token::Dollar)
            }
            '?' => {
                self.advance();
                Some(Token::Question)
            }
            '~' => {
                self.advance();
                Some(Token::Tilde)
            }
            '(' => {
                self.advance();
                Some(Token::OpenParen)
            }
            ')' => {
                self.advance();
                Some(Token::CloseParen)
            }
            '[' => {
                self.advance();
                Some(Token::OpenBracket)
            }
            ']' => {
                self.advance();
                Some(Token::CloseBracket)
            }
            '{' => {
                self.advance();
                Some(Token::OpenBrace)
            }
            '}' => {
                self.advance();
                Some(Token::CloseBrace)
            }

            // 处理字符字面量
            '\'' => self.read_char_or_lifetime(),

            // 处理字符串字面量
            '"' => self.read_string_literal(),

            // 处理标识符或关键字
            c if c.is_alphabetic() || c == '_' => self.read_identifier_or_keyword(),

            // 处理数字字面量
            c if c.is_digit(10) => self.read_number_or_float(),

            // 处理未知字符
            _ => {
                self.advance();
                Some(Token::Unknown(current))
            }
        };

        token
    }

    fn fill_buffer(&mut self, buffer_index: usize) -> Result<(), std::io::Error> {
        let buffer = &mut self.buffers[buffer_index];
        let bytes_read = self.reader.read(&mut buffer[..Self::PAGESIZE])?;

        // 设置有效数据并添加哨兵字符
        buffer[bytes_read] = '\0' as u8;
        if bytes_read < 4096 {
            self.eof = true; // 标记文件结束
        }
        Ok(())
    }

    fn peek_char(&mut self) -> Option<char> {
        // 获取当前缓冲区和位置
        let buf_idx = self.current_buffer;
        let pos = self.position;

        // 如果当前字符是哨兵，切换缓冲区
        if self.buffers[buf_idx][pos] == '\0' as u8 {
            if self.eof {
                return None;
            }
            self.switch_buffer();
            return self.peek_char();
        }

        // 否则返回当前字符
        Some(self.buffers[buf_idx][pos] as char)
    }

    fn advance(&mut self) {
        let buf_idx = self.current_buffer;
        let pos = self.position;

        // 如果当前位置是哨兵字符
        if self.buffers[buf_idx][pos] == b'\0' {
            // 如果到达文件末尾，直接返回
            if self.eof {
                return;
            }

            // 切换到下一个缓冲区
            self.switch_buffer();
        } else {
            // 移动到下一个位置
            self.position += 1;
        }
    }

    fn extract_range(&self, start: usize, end: usize) -> String {
        let mut result = String::new();
        let mut current_buffer = self.current_buffer;
        let mut current_position = start;

        while current_position < end {
            let buffer = &self.buffers[current_buffer];

            // 查找哨兵标记的有效范围
            let mut range_end = current_position;
            while range_end < end && buffer[range_end] != '\0' as u8 {
                range_end += 1;
            }

            result
                .push_str(std::str::from_utf8(&buffer[current_position..range_end]).unwrap_or(""));

            current_position = range_end;

            // 如果到达哨兵字符，切换缓冲区
            if current_position < end && buffer[current_position] == '\0' as u8 {
                current_buffer = (current_buffer + 1) % 2;
                current_position = 0;
            }
        }

        result
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn switch_buffer(&mut self) {
        // 切换到下一个缓冲区
        self.current_buffer = (self.current_buffer + 1) % 2;
        self.position = 0;

        // 如果新缓冲区为空且未到达 EOF，则填充它
        if !self.eof {
            self.fill_buffer(self.current_buffer).unwrap();
        }
    }

    fn read_line_comment(&mut self) -> Option<Token> {
        let start = self.position;
        while let Some(c) = self.peek_char() {
            if c == '\n' {
                break; // 结束行注释
            }
            self.advance();
        }

        let comment = self.extract_range(start, self.position);
        Some(Token::Comment(comment))
    }

    fn read_block_comment(&mut self) -> Option<Token> {
        let start = self.position;
        let mut depth = 1; // 嵌套注释深度

        while let Some(c) = self.peek_char() {
            if c == '/' && self.peek_ahead(1) == Some('*') {
                self.advance();
                self.advance();
                depth += 1;
            } else if c == '*' && self.peek_ahead(1) == Some('/') {
                self.advance();
                self.advance();
                depth -= 1;

                if depth == 0 {
                    let comment = self.extract_range(start, self.position - 2);
                    return Some(Token::Comment(comment));
                }
            } else {
                self.advance();
            }
        }

        None // 未终止的块注释
    }

    fn read_char_or_lifetime(&mut self) -> Option<Token> {
        self.advance(); // Skip the opening single quote

        if let Some(next_char) = self.peek_char() {
            if next_char == '\\' || self.peek_ahead_is_closing_quote() {
                // Case 1: Potential character literal
                self.read_char_literal_body()
            } else if next_char.is_alphabetic() || next_char == '_' {
                // Case 2: Potential lifetime or label
                self.read_lifetime_or_label()
            } else {
                None // Invalid input (e.g., just a single quote without valid content)
            }
        } else {
            None // Invalid input (e.g., end of file after single quote)
        }
    }

    fn read_char_literal_body(&mut self) -> Option<Token> {
        let mut character = None;

        if let Some(c) = self.peek_char() {
            if c == '\\' {
                // Handle escape sequence
                self.advance();
                if let Some(escaped) = self.read_escape_sequence() {
                    character = Some(escaped);
                } else {
                    return None; // Invalid escape sequence
                }
            } else if c != '\'' {
                // Normal character
                self.advance();
                character = Some(c);
            }
        }

        // Ensure closing quote
        if self.peek_char() == Some('\'') {
            self.advance(); // Skip the closing quote
            character.map(Token::CharLiteral)
        } else {
            None // Invalid char literal (e.g., unclosed single quote)
        }
    }

    fn read_lifetime_or_label(&mut self) -> Option<Token> {
        let start = self.position - 1; // 包括开头的 `'`

        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = self.extract_range(start, self.position);

        match lexeme.as_str() {
            "'_" => Some(Token::LifetimeOrLabel("'_".to_string())), // 特殊标识
            "'static" => Some(Token::LifetimeOrLabel("'static".to_string())), // 静态生命周期
            _ if lexeme.starts_with("'") => Some(Token::LifetimeOrLabel(lexeme)),
            _ => None, // 无效标签
        }
    }

    fn peek_ahead_is_closing_quote(&mut self) -> bool {
        if let Some(c) = self.peek_char() {
            if c == '\\' {
                // 如果是转义字符，检查第二个字符是否为单引号
                if let Some(escaped) = self.peek_ahead(2) {
                    return escaped == '\'';
                }
            } else {
                // 检查下一个字符是否为单引号
                if let Some(next) = self.peek_ahead(1) {
                    return next == '\'';
                }
            }
        }

        false
    }

    fn read_string_literal(&mut self) -> Option<Token> {
        self.advance(); // Skip the opening quote
        let mut lexeme = String::new();

        while let Some(c) = self.peek_char() {
            if c == '"' {
                self.advance(); // Skip the closing quote
                return Some(Token::StringLiteral(lexeme));
            } else if c == '\\' {
                // Handle escape sequence
                self.advance();
                if let Some(escaped) = self.read_escape_sequence() {
                    lexeme.push(escaped);
                } else {
                    return None; // Invalid escape sequence
                }
            } else {
                // Normal character
                self.advance();
                lexeme.push(c);
            }
        }

        None // Unterminated string literal
    }

    fn read_escape_sequence(&mut self) -> Option<char> {
        match self.peek_char()? {
            '\'' => {
                self.advance();
                Some('\'')
            }
            '"' => {
                self.advance();
                Some('"')
            }
            '\\' => {
                self.advance();
                Some('\\')
            }
            'n' => {
                self.advance();
                Some('\n')
            }
            'r' => {
                self.advance();
                Some('\r')
            }
            't' => {
                self.advance();
                Some('\t')
            }
            '0' => {
                self.advance();
                Some('\0')
            }
            // Add other escape sequences as needed
            _ => None, // Invalid escape sequence
        }
    }

    // Helper method to look ahead by a specific offset
    fn peek_ahead(&self, offset: usize) -> Option<char> {
        let mut buffer_idx = self.current_buffer;
        let mut pos = self.position + offset;

        // 处理跨缓冲区的情况
        while pos >= self.buffers[buffer_idx].len() || self.buffers[buffer_idx][pos] == '\0' as u8 {
            // 减去当前缓冲区的有效长度
            pos -= self.buffers[buffer_idx]
                .iter()
                .take_while(|&&b| b != '\0' as u8)
                .count();

            // 切换到下一个缓冲区
            buffer_idx = (buffer_idx + 1) % 2;

            // 如果到达 EOF，返回 None
            if self.eof && self.buffers[buffer_idx][0] == '\0' as u8 {
                return None;
            }
        }

        // 返回偏移量位置的字符
        Some(self.buffers[buffer_idx][pos] as char)
    }

    fn read_identifier_or_keyword(&mut self) -> Option<Token> {
        let start = self.position;
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = self.extract_range(start, self.position);
        match lexeme.as_str() {
            // 严格关键字
            "as" => Some(Token::As),
            "break" => Some(Token::Break),
            "const" => Some(Token::Const),
            "continue" => Some(Token::Continue),
            "crate" => Some(Token::Crate),
            "else" => Some(Token::Else),
            "enum" => Some(Token::Enum),
            "extern" => Some(Token::Extern),
            "false" => Some(Token::False),
            "fn" => Some(Token::Fn),
            "for" => Some(Token::For),
            "if" => Some(Token::If),
            "impl" => Some(Token::Impl),
            "in" => Some(Token::In),
            "let" => Some(Token::Let),
            "loop" => Some(Token::Loop),
            "match" => Some(Token::Match),
            "mod" => Some(Token::Mod),
            "move" => Some(Token::Move),
            "mut" => Some(Token::Mut),
            "pub" => Some(Token::Pub),
            "ref" => Some(Token::Ref),
            "return" => Some(Token::Return),
            "self" => Some(Token::SELFVALUE),
            "Self" => Some(Token::SELFTYPE),
            "static" => Some(Token::Static),
            "struct" => Some(Token::Struct),
            "super" => Some(Token::Super),
            "trait" => Some(Token::Trait),
            "true" => Some(Token::True),
            "type" => Some(Token::Type),
            "unsafe" => Some(Token::Unsafe),
            "use" => Some(Token::Use),
            "where" => Some(Token::Where),
            "while" => Some(Token::While),
            "async" => Some(Token::Async),
            "await" => Some(Token::Await),
            "dyn" => Some(Token::Dyn),

            // 保留关键字
            "abstract" => Some(Token::Abstract),
            "become" => Some(Token::Become),
            "box" => Some(Token::Box),
            "do" => Some(Token::Do),
            "final" => Some(Token::Final),
            "macro" => Some(Token::Macro),
            "override" => Some(Token::Override),
            "priv" => Some(Token::Priv),
            "typeof" => Some(Token::Typeof),
            "unsized" => Some(Token::Unsized),
            "virtual" => Some(Token::Virtual),
            "yield" => Some(Token::Yield),
            "try" => Some(Token::Try),

            // 弱关键字
            "macro_rules" => Some(Token::MacroRules),
            "union" => Some(Token::Union),
            "'static" => Some(Token::StaticLifetime),

            // 其他情况处理为标识符
            _ => Some(Token::Identifier(lexeme.to_string())),
        }
    }

    fn read_number_or_float(&mut self) -> Option<Token> {
        let start = self.position;

        // 整数部分
        while let Some(c) = self.peek_char() {
            if c.is_digit(10) || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // 检查是否是浮点数
        if self.peek_char() == Some('.') {
            self.advance(); // 跳过小数点

            while let Some(c) = self.peek_char() {
                if c.is_digit(10) || c == '_' {
                    self.advance();
                } else {
                    break;
                }
            }

            // 检查是否是科学计数法
            if let Some(c) = self.peek_char() {
                if c == 'e' || c == 'E' {
                    self.advance(); // 跳过 e 或 E

                    if let Some(sign) = self.peek_char() {
                        if sign == '+' || sign == '-' {
                            self.advance(); // 跳过正负号
                        }
                    }

                    while let Some(c) = self.peek_char() {
                        if c.is_digit(10) || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }

            // 返回浮点字面量
            let lexeme = self.extract_range(start, self.position);
            return Some(Token::FloatLiteral(lexeme));
        }

        // 返回整数字面量
        let lexeme = self.extract_range(start, self.position);
        Some(Token::IntegerLiteral(lexeme))
    }

    fn read_float_starting_with_dot(&mut self) -> Option<Token> {
        let start = self.position;

        self.advance(); // 跳过小数点

        while let Some(c) = self.peek_char() {
            if c.is_digit(10) || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // 检查是否是科学计数法
        if let Some(c) = self.peek_char() {
            if c == 'e' || c == 'E' {
                self.advance(); // 跳过 e 或 E

                if let Some(sign) = self.peek_char() {
                    if sign == '+' || sign == '-' {
                        self.advance(); // 跳过正负号
                    }
                }

                while let Some(c) = self.peek_char() {
                    if c.is_digit(10) || c == '_' {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        // 返回浮点字面量
        let lexeme = self.extract_range(start, self.position);
        Some(Token::FloatLiteral(lexeme))
    }
}
