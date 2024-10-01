use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Symbol {
    Terminal(String),
    NonTerminal(String),
}

type Production = (Vec<Symbol>, Vec<Symbol>);

fn parse_symbol(s: &str, non_terminals: &[Symbol], terminals: &[Symbol]) -> Symbol {
    if non_terminals.iter().any(|nt| matches!(nt, Symbol::NonTerminal(x) if x == s)) {
        Symbol::NonTerminal(s.to_string())
    } else if terminals.iter().any(|t| matches!(t, Symbol::Terminal(x) if x == s)) {
        Symbol::Terminal(s.to_string())
    } else {
        panic!("Unknown symbol: {}", s)
    }
}

fn parse_production(
    (lhs, rhs): (Vec<String>, Vec<String>),
    non_terminals: &[Symbol],
    terminals: &[Symbol],
) -> Production {
    let lhs_symbols = lhs.iter().map(|s| parse_symbol(s, non_terminals, terminals)).collect();
    let rhs_symbols = rhs.iter().map(|s| parse_symbol(s, non_terminals, terminals)).collect();
    (lhs_symbols, rhs_symbols)
}

fn parse_productions(
    productions: Vec<(Vec<String>, Vec<String>)>,
    non_terminals: &[Symbol],
    terminals: &[Symbol],
) -> Vec<Production> {
    productions
        .into_iter()
        .map(|production| parse_production(production, non_terminals, terminals))
        .collect()
}

fn is_left_linear(productions: &[Production]) -> bool {
    productions.iter().all(|(lhs_symbols, rhs_symbols)| {
        matches!(lhs_symbols.as_slice(), [Symbol::NonTerminal(_)])
            && (matches!(rhs_symbols.as_slice(), [Symbol::Terminal(_)])
                || matches!(rhs_symbols.split_last(), Some((Symbol::Terminal(_), non_terminals)) 
                    if non_terminals.iter().all(|s| matches!(s, Symbol::NonTerminal(_)))))
    })
}

fn is_right_linear(productions: &[Production]) -> bool {
    productions.iter().all(|(lhs_symbols, rhs_symbols)| {
        matches!(lhs_symbols.as_slice(), [Symbol::NonTerminal(_)])
            && (matches!(rhs_symbols.as_slice(), [Symbol::Terminal(_)])
                || matches!(rhs_symbols.split_first(), Some((Symbol::Terminal(_), non_terminals)) 
                    if non_terminals.iter().all(|s| matches!(s, Symbol::NonTerminal(_)))))
    })
}

fn is_regular_grammar(productions: &[Production]) -> bool {
    productions.iter().all(|(lhs_symbols, rhs_symbols)| {
        matches!(lhs_symbols.as_slice(), [Symbol::NonTerminal(_)])
            && (matches!(rhs_symbols.as_slice(), [Symbol::Terminal(_)])
                || matches!(rhs_symbols.as_slice(), [Symbol::Terminal(_), Symbol::NonTerminal(_)]))
    })
}

fn is_type_2(productions: &[Production]) -> bool {
    productions.iter().all(|(lhs_symbols, _)| matches!(lhs_symbols.as_slice(), [Symbol::NonTerminal(_)]))
}

fn is_type_1(productions: &[Production]) -> bool {
    productions.iter().all(|(lhs_symbols, rhs_symbols)| {
        lhs_symbols.len() >= 1 && rhs_symbols.len() >= 1 && lhs_symbols.len() <= rhs_symbols.len()
    })
}

fn classify_grammar(
    non_terminals: &[Symbol],
    terminals: &[Symbol],
    productions: Vec<(Vec<String>, Vec<String>)>,
) -> String {
    let parsed_productions = parse_productions(productions, non_terminals, terminals);
    if is_regular_grammar(&parsed_productions) {
        "正则文法".to_string()
    } else if is_right_linear(&parsed_productions) {
        "右线性文法".to_string()
    } else if is_left_linear(&parsed_productions) {
        "左线性文法".to_string()
    } else if is_type_1(&parsed_productions) {
        "上下文有关文法".to_string()
    } else if is_type_2(&parsed_productions) {
        "上下文无关文法".to_string()
    } else {
        "未知文法".to_string()
    }
}

fn read_symbols(prompt: &str) -> Vec<Symbol> {
    print!("{} (用空格分隔): ", prompt);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .split_whitespace()
        .map(|s| {
            if s.chars().next().unwrap().is_ascii_uppercase() {
                Symbol::NonTerminal(s.to_string())
            } else {
                Symbol::Terminal(s.to_string())
            }
        })
        .collect()
}

fn read_productions() -> Vec<(Vec<String>, Vec<String>)> {
    let mut productions = Vec::new();
    loop {
        print!("输入产生式 (格式: LHS -> RHS，用空格分隔，输入空行结束): ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split("->").collect();
        if parts.len() != 2 {
            println!("格式错误，请重新输入。");
            continue;
        }
        let lhs: Vec<String> = parts[0].trim().split_whitespace().map(String::from).collect();
        let rhs: Vec<String> = parts[1].trim().split_whitespace().map(String::from).collect();
        productions.push((lhs, rhs));
    }
    productions
}

fn main() {
    let non_terminals = read_symbols("输入非终结符");
    let terminals = read_symbols("输入终结符");
    let productions = read_productions();
    print!("输入开始符号: ");
    io::stdout().flush().unwrap();
    let mut start_symbol_input = String::new();
    io::stdin().read_line(&mut start_symbol_input).unwrap();
    let start_symbol_input = start_symbol_input.trim();
    let start_symbol = non_terminals
        .iter()
        .find(|nt| matches!(nt, Symbol::NonTerminal(x) if x == start_symbol_input))
        .cloned()
        .expect(&format!("Unknown start symbol: {}", start_symbol_input));

    let grammar_type = classify_grammar(&non_terminals, &terminals, productions);
    println!("文法类型: {}", grammar_type);
}
