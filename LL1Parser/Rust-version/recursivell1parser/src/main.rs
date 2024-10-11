use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::slice::Iter;

type Grammar = HashMap<String, Vec<Vec<String>>>;

fn main() {
    let grammar: Grammar = [
        ("E".to_string(), vec![vec!["T".to_string(), "E'".to_string()]]),
        ("E'".to_string(), vec![vec!["ADD_SUB".to_string(), "T".to_string(), "E'".to_string()], vec!["".to_string()]]),
        ("T".to_string(), vec![vec!["F".to_string(), "T'".to_string()]]),
        ("T'".to_string(), vec![vec!["MUL_DIV".to_string(), "F".to_string(), "T'".to_string()], vec!["".to_string()]]),
        ("F".to_string(), vec![vec!["number".to_string()], vec!["(".to_string(), "E".to_string(), ")".to_string()]]),
        ("ADD_SUB".to_string(), vec![vec!["+".to_string()], vec!["-".to_string()]]),
        ("MUL_DIV".to_string(), vec![vec!["*".to_string()], vec!["div".to_string()], vec!["mod".to_string()]]),
    ].iter().cloned().collect();

    let first = compute_first(&grammar);
    let follow = compute_follow(&grammar, "E");

    println!("First set: {:?}", first);
    println!("Follow set: {:?}", follow);

    let parsing_table = build_parsing_table(&grammar, &first, &follow);
    println!("Parsing table: {:?}", parsing_table);

    let input_str = "number + number * number $";
    let input: Vec<&str> = input_str.trim().split_whitespace().collect();
    let mut input_iter = input.iter().peekable();
    let success = parse(&grammar, &parsing_table, "E", &mut input_iter);

    println!("Parsing result: {}", if success { "Success" } else { "Failure" });
}

// LL(1) parser main function
fn parse<'a>(
    grammar: &Grammar,
    parsing_table: &HashMap<(String, String), Vec<String>>,
    start_symbol: &str,
    input_iter: &mut Peekable<Iter<'a, &'a str>>,
) -> bool {
    parse_non_terminal(grammar, parsing_table, start_symbol, input_iter)
}

// Recursive function to parse non-terminal symbols
fn parse_non_terminal<'a>(
    grammar: &Grammar,
    parsing_table: &HashMap<(String, String), Vec<String>>,
    symbol: &str,
    input_iter: &mut Peekable<Iter<'a, &'a str>>,
) -> bool {
    if let Some(&lookahead) = input_iter.peek() {
        if let Some(production) = parsing_table.get(&(symbol.to_string(), lookahead.to_string())) {
            println!("Using production: {} -> {:?}", symbol, production);
            for prod_symbol in production {
                if prod_symbol == "" {
                    println!("Parsing {} -> ε", symbol);
                    continue;
                }

                if grammar.contains_key(prod_symbol) {
                    if !parse_non_terminal(grammar, parsing_table, prod_symbol, input_iter) {
                        return false;
                    }
                } else {
                    if !match_terminal(input_iter, prod_symbol) {
                        return false;
                    }
                }
            }
            return true;
        } else {
            println!("Error: No matching production for ({}, {})", symbol, lookahead);
            return false;
        }
    }
    false
}

// Function to match terminal symbols
fn match_terminal<'a>(
    input_iter: &mut Peekable<Iter<'a, &'a str>>,
    expected: &str,
) -> bool {
    if let Some(&next_token) = input_iter.peek() {
        if *next_token == expected {
            println!("Matched terminal: {}", next_token);
            input_iter.next(); // Consume input
            return true;
        }
    }
    println!("Error: Expected '{}', but found '{:?}'", expected, input_iter.peek());
    false
}

// Compute the First set for the grammar
fn compute_first(grammar: &Grammar) -> HashMap<String, HashSet<String>> {
    let mut first: HashMap<String, HashSet<String>> = HashMap::new();

    for non_terminal in grammar.keys() {
        first.insert(non_terminal.clone(), HashSet::new());
    }

    let mut changed = true;
    while changed {
        changed = false;
        for (non_terminal, productions) in grammar.iter() {
            for production in productions {
                if update_first_set(grammar, &mut first, non_terminal, production) {
                    changed = true;
                }
            }
        }
    }

    first
}

// Update the First set for a given production
fn update_first_set(
    grammar: &Grammar,
    first: &mut HashMap<String, HashSet<String>>,
    non_terminal: &String,
    production: &Vec<String>,
) -> bool {
    let mut changed = false;
    for symbol in production {
        if grammar.contains_key(symbol) {
            let first_set = first.get(symbol).unwrap().clone();
            let len_before = first.get(non_terminal).unwrap().len();
            first.get_mut(non_terminal).unwrap().extend(first_set.clone());
            let len_after = first.get(non_terminal).unwrap().len();
            if len_before != len_after {
                changed = true;
            }
            if !first_set.contains(&"".to_string()) {
                break;
            }
        } else {
            let len_before = first.get(non_terminal).unwrap().len();
            first.get_mut(non_terminal).unwrap().insert(symbol.clone());
            let len_after = first.get(non_terminal).unwrap().len();
            if len_before != len_after {
                changed = true;
            }
            break;
        }
    }
    changed
}

// Compute the Follow set for the grammar
fn compute_follow(grammar: &Grammar, start_symbol: &str) -> HashMap<String, HashSet<String>> {
    let mut follow: HashMap<String, HashSet<String>> = HashMap::new();

    for non_terminal in grammar.keys() {
        follow.insert(non_terminal.clone(), HashSet::new());
    }

    follow.get_mut(start_symbol).unwrap().insert("$".to_string());

    let mut changed = true;
    while changed {
        changed = false;
        for (non_terminal, productions) in grammar.iter() {
            for production in productions {
                if update_follow_set(grammar, &mut follow, non_terminal, production) {
                    changed = true;
                }
            }
        }
    }

    follow
}

// Update the Follow set for a given production
fn update_follow_set(
    grammar: &Grammar,
    follow: &mut HashMap<String, HashSet<String>>,
    non_terminal: &String,
    production: &Vec<String>,
) -> bool {
    let mut changed = false;
    for (i, symbol) in production.iter().enumerate() {
        if grammar.contains_key(symbol) {
            let mut follow_set = follow.get(symbol).unwrap().clone();
            if i + 1 < production.len() {
                let next_symbol = &production[i + 1];
                if grammar.contains_key(next_symbol) {
                    let first_set = compute_first(grammar).get(next_symbol).unwrap().clone();
                    follow_set.extend(first_set.iter().filter(|x| **x != "").cloned());
                    if first_set.contains(&"".to_string()) {
                        follow_set.extend(follow.get(non_terminal).unwrap().clone());
                    }
                } else {
                    follow_set.insert(next_symbol.clone());
                }
            } else {
                follow_set.extend(follow.get(non_terminal).unwrap().clone());
            }
            let len_before = follow.get(symbol).unwrap().len();
            follow.get_mut(symbol).unwrap().extend(follow_set.clone());
            let len_after = follow.get(symbol).unwrap().len();
            if len_before != len_after {
                changed = true;
            }
        }
    }
    changed
}

// Build the parsing table for the grammar
fn build_parsing_table(
    grammar: &Grammar,
    first: &HashMap<String, HashSet<String>>,
    follow: &HashMap<String, HashSet<String>>
) -> HashMap<(String, String), Vec<String>> {
    let mut table: HashMap<(String, String), Vec<String>> = HashMap::new();

    for (non_terminal, productions) in grammar.iter() {
        for production in productions {
            let first_set = compute_first_for_production(production, first);
            println!("{:?}", first_set);
            for terminal in first_set.iter().filter(|&&ref x| x != "") {
                table.insert((non_terminal.clone(), terminal.clone()), production.clone());
            }
            if first_set.contains(&"".to_string()) {
                for terminal in follow.get(non_terminal).unwrap() {
                    table.insert((non_terminal.clone(), terminal.clone()), production.clone());
                }
            }
        }
    }

    table
}

// Compute the First set for a given production
fn compute_first_for_production(
    production: &Vec<String>,
    first: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    let mut result = HashSet::new();
    for symbol in production {
        if first.contains_key(symbol) {
            let first_set = first.get(symbol).unwrap();
            result.extend(first_set.clone());
            if !first_set.contains(&"".to_string()) {
                break;
            }
        } else {
            result.insert(symbol.clone());
            break;
        }
    }
    result
}