use crate::lexer::Lexer;
use crate::tokens::Token;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

/// 处理字面量并将其存入表中
pub fn process_literal<T, F>(
    table: &mut HashMap<T, usize>,
    next_id: &mut usize,
    writer: &mut BufWriter<File>,
    value: T,
    formatter: F,
) -> usize
where
    T: std::hash::Hash + Eq + Clone,
    F: FnOnce() -> String,
{
    if let Some(&id) = table.get(&value) {
        id
    } else {
        let id = *next_id;
        *next_id += 1;
        table.insert(value.clone(), id);

        let formatted_value = formatter();
        writeln!(writer, "{} {}", id, formatted_value).expect("Failed to write to table file");

        id
    }
}

/// 运行词法分析并生成对应的输出文件
pub fn run_lexer_pipeline(input_file: &str) -> std::io::Result<()> {
    // 打开输入文件
    let input_file = File::open(input_file)?;
    let reader = BufReader::new(input_file);
    let mut lexer = Lexer::new(reader);

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

    let identifier_writer = File::create("identifier_table.txt")?;
    let mut identifier_writer = BufWriter::new(identifier_writer);

    let char_literal_writer = File::create("char_literal_table.txt")?;
    let mut char_literal_writer = BufWriter::new(char_literal_writer);

    let string_literal_writer = File::create("string_literal_table.txt")?;
    let mut string_literal_writer = BufWriter::new(string_literal_writer);

    let integer_literal_writer = File::create("integer_literal_table.txt")?;
    let mut integer_literal_writer = BufWriter::new(integer_literal_writer);

    let float_literal_writer = File::create("float_literal_table.txt")?;
    let mut float_literal_writer = BufWriter::new(float_literal_writer);

    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);

    while let Some(token) = lexer.next_token() {
        let token_output = match token {
            Token::Identifier(ref ident) => {
                let id = process_literal(
                    &mut identifier_table,
                    &mut identifier_next_id,
                    &mut identifier_writer,
                    ident.clone(),
                    || ident.clone(),
                );
                format!("Identifier({})", id)
            }
            Token::CharLiteral(ref ch) => {
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
                        '"' => "\\\"".to_string(),
                        '\0' => "\\0".to_string(),
                        _ => ch.to_string(),
                    },
                );
                format!("CharLiteral({})", id)
            }
            Token::StringLiteral(ref s) => {
                let id = process_literal(
                    &mut string_literal_table,
                    &mut string_literal_next_id,
                    &mut string_literal_writer,
                    s.clone(),
                    || s.clone(),
                );
                format!("StringLiteral({})", id)
            }
            Token::IntegerLiteral(ref i) => {
                let id = process_literal(
                    &mut integer_literal_table,
                    &mut integer_literal_next_id,
                    &mut integer_literal_writer,
                    i.clone(),
                    || i.clone(),
                );
                format!("IntegerLiteral({})", id)
            }
            Token::FloatLiteral(ref f) => {
                let id = process_literal(
                    &mut float_literal_table,
                    &mut float_literal_next_id,
                    &mut float_literal_writer,
                    f.clone(),
                    || f.clone(),
                );
                format!("FloatLiteral({})", id)
            }
            Token::Comment(_) => continue,
            _ => format!("{:?}", token),
        };

        writeln!(writer, "{}", token_output)?;
    }

    writer.flush()?;
    identifier_writer.flush()?;
    char_literal_writer.flush()?;
    string_literal_writer.flush()?;
    integer_literal_writer.flush()?;
    float_literal_writer.flush()?;

    Ok(())
}
