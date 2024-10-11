

use std::collections::{HashMap, HashSet};

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

    let input_str = "number mod number div number * number";
    let input: Vec<&str> = input_str.trim().split_whitespace().collect();
    let success = parse(&grammar, &parsing_table, &follow, "E", &input);
    
    println!("Parsing result: {}", if success { "Success" } else { "Failure" });
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
            if !first_set.contains(&"".to_owned()) {
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

    follow.get_mut(start_symbol).unwrap().insert("$".to_owned());

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
                    if first_set.contains(&"".to_owned()) {
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
            for terminal in first_set.iter().filter(|&&ref x| x != "") {
                table.insert((non_terminal.clone(), terminal.clone()), production.clone());
            }
            if first_set.contains(&"".to_owned()) {
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
            if !first_set.contains(&"".to_owned()) {
                break;
            }
        } else {
            result.insert(symbol.clone());
            break;
        }
    }
    result
}

// LL(1) parser with panic mode error recovery
fn parse(
    grammar: &Grammar,
    parsing_table: &HashMap<(String, String), Vec<String>>,
    follow: &HashMap<String, HashSet<String>>,
    start_symbol: &str,
    input: &Vec<&str>
) -> bool {
    let mut stack = vec!["$".to_owned(), start_symbol.to_owned()];
    let mut input_iter = input.iter();
    let mut lookahead = input_iter.next().unwrap(); // Initialize with the first input symbol

    println!("Initial stack: {:?}", stack);
    println!("Initial input: {:?}", input);

    while let Some(top) = stack.pop() {
        println!("Stack top: {}", top);
        
        // If the top of the stack is a terminal
        if !grammar.contains_key(&top) {
            if &top == lookahead {
                println!("Matched terminal: {}", lookahead);  // Stack top matches input symbol
                
                // Move to the next input symbol
                if let Some(next_symbol) = input_iter.next() {
                    lookahead = next_symbol;
                } else {
                    lookahead = &"$";  // End of input
                }
            } else {
                println!("Error: Stack top does not match input ({} != {})", top, lookahead);
                return false;
            }
        } else {
            // If the top of the stack is a non-terminal, look up the parsing table
            if let Some(production) = parsing_table.get(&(top.clone(), lookahead.to_string())) {
                println!("Using production: {:?} -> {:?}", top, production);
                if production == &vec!["".to_owned()] {
                    println!("Empty production, skipping");
                    continue;
                }
                // Push the production in reverse order onto the stack
                for symbol in production.iter().rev() {
                    if !symbol.is_empty() {
                        stack.push(symbol.clone());
                    }
                }
            } else {
                println!("Error: No matching production for ({}, {})", top, lookahead);
                // Panic mode error recovery: skip symbols until a synchronization point is found
                while let Some(next_symbol) = input_iter.next() {
                    lookahead = next_symbol;
                    if follow.get(&top).unwrap().contains(*lookahead) {
                        println!("Recovered at synchronization point: {}", lookahead);
                        break;
                    }
                }
                if !follow.get(&top).unwrap().contains(*lookahead) {
                    println!("Error: Unable to recover, no synchronization point found");
                    return false;
                }
            }
        }
        
        println!("Current stack: {:?}", stack);
    }

    // Parsing is successful if the stack is empty and the input symbol is the end symbol `$`
    let success = lookahead == &"$";
    if success {
        println!("Parsing successful");
    } else {
        println!("Parsing failed");
    }

    success
}
