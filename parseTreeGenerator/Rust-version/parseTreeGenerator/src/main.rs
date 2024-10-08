use std::collections::{HashMap, HashSet};

type Grammar = HashMap<String, Vec<Vec<String>>>;

#[derive(Debug, Clone)]
enum Node {
    Leaf(String),
    Internal(String, Vec<Node>),
}

fn tree_to_string(indent: &str, tree: &Node) -> String {
    match tree {
        Node::Leaf(s) => format!("{}{}\n", indent, s),
        Node::Internal(label, children) => {
            let mut result = format!("{}{}\n", indent, label);
            for child in children {
                result.push_str(&tree_to_string(&format!("{indent}  "), child));
            }
            result
        }
    }
}

fn tree_to_typst(indent: &str, tree: &Node) -> String {
    match tree {
        Node::Leaf(s) => format!("{indent}tree(\"{}\")", s),
        Node::Internal(label, children) => {
            let new_indent = format!("{indent}  ");
            let children_typst: Vec<String> = children.iter().map(|child| tree_to_typst(&new_indent, child)).collect();
            let children_str = children_typst.join(",\n");
            format!("{indent}tree(\"{}\",\n{}\n{indent})", label, children_str)
        }
    }
}

// fn possible_splits(i: usize, j: usize, n: usize) -> Vec<Vec<usize>> {
//     if n == 1 {
//         if i < j {
//             vec![vec![]]
//         } else {
//             vec![]
//         }
//     } else {
//         let positions: Vec<usize> = (i + 1..j).collect();
//         fn combinations(acc: Vec<usize>, k: usize, positions: &[usize]) -> Vec<Vec<usize>> {
//             if k == 0 {
//                 vec![acc]
//             } else {
//                 positions.iter().flat_map(|&pos| {
//                     let mut new_acc = acc.clone();
//                     new_acc.push(pos);
//                     combinations(new_acc, k - 1, positions)
//                 }).collect()
//             }
//         }
//         combinations(vec![], n - 1, &positions)
//     }
// }

fn possible_splits(i: usize, j: usize, n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        if i < j {
            vec![vec![]]
        } else {
            vec![]
        }
    } else {
        let positions: Vec<usize> = (i + 1..j).collect();
        fn combinations(acc: Vec<usize>, remaining_positions: &[usize], k: usize) -> Vec<Vec<usize>> {
            if k == 0 {
                vec![acc]
            } else {
                remaining_positions.iter().enumerate().flat_map(|(idx, &pos)| {
                    let new_positions: Vec<usize> = remaining_positions.iter().enumerate()
                        .filter_map(|(i, &p)| if i > idx { Some(p) } else { None })
                        .collect();
                    let mut new_acc = acc.clone();
                    new_acc.push(pos);
                    combinations(new_acc, &new_positions, k - 1)
                }).collect()
            }
        }
        combinations(vec![], &positions, n - 1)
    }
}

fn lookup_rules(grammar: &Grammar, nt: &str) -> Vec<Vec<String>> {
    grammar.get(nt).cloned().unwrap_or_else(Vec::new)
}

fn remove_epsilons(grammar: &Grammar) -> (Grammar, Vec<(String, Vec<String>)>) {
    fn remove_eps_from_rhs(rhs_list: &[String], eps_nonterms: &HashSet<String>) -> Vec<Vec<String>> {
        match rhs_list.split_first() {
            None => vec![vec![]],
            Some((hd, tl)) => {
                let rest = remove_eps_from_rhs(tl, eps_nonterms);
                if eps_nonterms.contains(hd) {
                    let mut new_rest = rest.clone();
                    for r in &rest {
                        let mut new_r = vec![hd.clone()];
                        new_r.extend(r.clone());
                        new_rest.push(new_r);
                    }
                    new_rest
                } else {
                    rest.into_iter().map(|r| {
                        let mut new_r = vec![hd.clone()];
                        new_r.extend(r);
                        new_r
                    }).collect()
                }
            }
        }
    }

    fn find_eps_nonterms(grammar: &Grammar) -> HashSet<String> {
        grammar.iter().fold(HashSet::new(), |mut acc, (lhs, rhs_list)| {
            if rhs_list.iter().any(|rhs| rhs == &vec!["ε".to_string()]) {
                acc.insert(lhs.clone());
            }
            acc
        })
    }

    fn update_eps_nonterms(eps_nonterms: &HashSet<String>, grammar: &Grammar) -> HashSet<String> {
        let mut new_eps_nonterms = eps_nonterms.clone();
        for (lhs, rhs_list) in grammar {
            if rhs_list.iter().any(|rhs| rhs.iter().all(|sym| eps_nonterms.contains(sym))) {
                new_eps_nonterms.insert(lhs.clone());
            }
        }
        if new_eps_nonterms.len() > eps_nonterms.len() {
            update_eps_nonterms(&new_eps_nonterms, grammar)
        } else {
            new_eps_nonterms
        }
    }

    let eps_nonterms = update_eps_nonterms(&find_eps_nonterms(grammar), grammar);

    let new_grammar = grammar.iter().fold(HashMap::new(), |mut acc, (lhs, rhs_list)| {
        let new_rhs_list = rhs_list.iter().fold(vec![], |mut acc_rhs, rhs| {
            if rhs == &vec!["ε".to_string()] {
                acc_rhs
            } else {
                acc_rhs.extend(remove_eps_from_rhs(rhs, &eps_nonterms));
                acc_rhs
            }
        });
        acc.insert(lhs.clone(), new_rhs_list);
        acc
    });

    fn generate_new_productions(grammar: &Grammar, eps_nonterms: &HashSet<String>, acc: Vec<(String, Vec<String>)>) -> Vec<(String, Vec<String>)> {
        grammar.iter().fold(acc, |mut acc, (lhs, rhs_list)| {
            for rhs in rhs_list {
                if rhs.iter().any(|sym| eps_nonterms.contains(sym)) {
                    let new_rhs: Vec<String> = rhs.iter().filter(|sym| !eps_nonterms.contains(*sym)).cloned().collect();
                    if !new_rhs.is_empty() && !acc.contains(&(lhs.clone(), new_rhs.clone())) {
                        acc.push((lhs.clone(), new_rhs));
                    }
                }
            }
            acc
        })
    }

    let new_productions = generate_new_productions(&new_grammar, &eps_nonterms, vec![]);

    (new_grammar, new_productions)
}

fn parse(
    grammar: &Grammar,
    tokens: &[&str],
    non_terminals: &HashSet<String>,
    terminals: &HashSet<String>,
    memo: &mut HashMap<(String, usize, usize), Vec<Node>>,
    nt: &str,
    i: usize,
    j: usize,
) -> Vec<Node> {
    let key = (nt.to_owned(), i, j);
    if let Some(result) = memo.get(&key) {
        return result.clone();
    }

    let mut results = Vec::new();
    if i >= j {
        // No results
    } else if terminals.contains(nt) {
        if i + 1 == j && tokens[i] == nt {
            results.push(Node::Leaf(nt.to_owned()));
        }
    } else if non_terminals.contains(nt) {
        let rules = lookup_rules(grammar, nt);
        for production in rules {
            let n = production.len();
            if n == 1 {
                let symbol = &production[0];
                if terminals.contains(symbol) {
                    if i + 1 == j && tokens[i] == symbol {
                        results.push(Node::Internal(nt.to_owned(), vec![Node::Leaf(symbol.to_owned())]));
                    }
                } else {
                    let sub_trees = parse(grammar, tokens, non_terminals, terminals, memo, symbol, i, j);
                    for sub_tree in sub_trees {
                        results.push(Node::Internal(nt.to_owned(), vec![sub_tree]));
                    }
                }
            } else {
                for splits in possible_splits(i, j, n) {
                    let positions = std::iter::once(&i).chain(splits.iter()).chain(std::iter::once(&j)).cloned().collect::<Vec<_>>();
                    let mut children = Vec::new();
                    let mut failed = false;
                    for (idx, ai) in production.iter().enumerate() {
                        let start = positions[idx];
                        let end = positions[idx + 1];
                        let sub_trees = parse(grammar, tokens, non_terminals, terminals, memo, ai, start, end);
                        if sub_trees.is_empty() {
                            failed = true;
                            break;
                        }
                        children.push(sub_trees);
                    }
                    if !failed {
                        let combinations = children.iter().fold(vec![vec![]], |acc, sub_trees| {
                            acc.into_iter().flat_map(|acc_subtree| {
                                sub_trees.iter().map(move |t| {
                                    let mut new_acc = acc_subtree.clone();
                                    new_acc.push(t.clone());
                                    new_acc
                                })
                            }).collect::<Vec<_>>()
                        });
                        for combination in combinations {
                            results.push(Node::Internal(nt.to_owned(), combination));
                        }
                    }
                }
            }
        }
    }

    memo.insert(key, results.clone());
    results
}

fn main() {
    // Define the grammar
    let grammar: Grammar = [
        ("S".to_owned(), vec![vec!["NP".to_owned(), "VP".to_owned()]]),
        ("NP".to_owned(), vec![vec!["Det".to_owned(), "N".to_owned()], vec!["NP".to_owned(), "PP".to_owned()]]),
        ("VP".to_owned(), vec![vec!["V".to_owned(), "NP".to_owned()], vec!["VP".to_owned(), "PP".to_owned()]]),
        ("PP".to_owned(), vec![vec!["P".to_owned(), "NP".to_owned()]]),
        ("Det".to_owned(), vec![vec!["the".to_owned()], vec!["a".to_owned()]]),
        ("N".to_owned(), vec![vec!["cat".to_owned()], vec!["dog".to_owned()], vec!["telescope".to_owned()], vec!["park".to_owned()]]),
        ("V".to_owned(), vec![vec!["saw".to_owned()], vec!["walked".to_owned()]]),
        ("P".to_owned(), vec![vec!["in".to_owned()], vec!["with".to_owned()]]),
    ].iter().cloned().collect();

    // Eliminate epsilon productions
    let (grammar, new_productions) = remove_epsilons(&grammar);

    // Collect non-terminals and terminals
    let non_terminals: HashSet<String> = grammar.keys().cloned().collect();
    let terminals: HashSet<String> = grammar.values().flat_map(|prods| {
        prods.iter().flat_map(|prod| {
            prod.iter().filter(|sym| !non_terminals.contains(*sym)).cloned()
        })
    }).collect();

    // Input sentence
    let sentence = "the dog saw a cat in the park";
    let tokens: Vec<&str> = sentence.split_whitespace().collect();

    // Parse the sentence starting from 'S'
    let mut memo = HashMap::new();
    let trees = parse(&grammar, &tokens, &non_terminals, &terminals, &mut memo, "S", 0, tokens.len());

    // Print all possible parse trees
    for (idx, tree) in trees.iter().enumerate() {
        println!("Parse tree {}:", idx + 1);
        print!("{}", tree_to_string("", tree));
        println!("Typst tree code {}:", idx + 1);
        println!("#{}", tree_to_typst("", tree));
    }
}

// use std::collections::{HashMap, HashSet};

// type Grammar = HashMap<&'static str, Vec<Vec<&'static str>>>;

// #[derive(Debug, Clone)]
// enum Node {
//     Leaf(String),
//     Internal(String, Vec<Node>),
// }

// fn tree_to_string(indent: &str, tree: &Node) -> String {
//     match tree {
//         Node::Leaf(s) => format!("{}{}\n", indent, s),
//         Node::Internal(label, children) => {
//             let mut result = format!("{}{}\n", indent, label);
//             for child in children {
//                 result.push_str(&tree_to_string(&(indent.to_owned() + "  "), child));
//             }
//             result
//         }
//     }
// }

// fn possible_splits(i: usize, j: usize, n: usize) -> Vec<Vec<usize>> {
//     if n == 1 {
//         if i < j {
//             vec![vec![]]
//         } else {
//             vec![]
//         }
//     } else {
//         let positions: Vec<usize> = (i + 1..j).collect();
//         fn combinations(acc: Vec<usize>, remaining_positions: &[usize], k: usize) -> Vec<Vec<usize>> {
//             if k == 0 {
//                 vec![acc]
//             } else {
//                 remaining_positions.iter().enumerate().flat_map(|(idx, &pos)| {
//                     let new_positions: Vec<usize> = remaining_positions.iter().enumerate()
//                         .filter_map(|(i, &p)| if i != idx { Some(p) } else { None })
//                         .collect();
//                     let mut new_acc = acc.clone();
//                     new_acc.push(pos);
//                     if acc.is_empty() || pos > *acc.last().unwrap() {
//                         combinations(new_acc, &new_positions, k - 1)
//                     } else {
//                         vec![]
//                     }
//                 }).collect()
//             }
//         }
//         combinations(vec![], &positions, n - 1)
//     }
// }

// fn lookup_rules(grammar: &Grammar, nt: &str) -> Vec<Vec<&'static str>> {
//     grammar.get(nt).cloned().unwrap_or_else(Vec::new)
// }

// fn parse<'a>(
//     grammar: &'a Grammar,
//     tokens: &[&str],
//     non_terminals: &HashSet<&'static str>,
//     terminals: &HashSet<&'static str>,
//     memo: &mut HashMap<(String, usize, usize), Vec<Node>>,
//     nt: &str,
//     i: usize,
//     j: usize,
// ) -> Vec<Node> {
//     let key = (nt.to_owned(), i, j);
//     if let Some(result) = memo.get(&key) {
//         return result.clone();
//     }

//     let mut results = Vec::new();
//     if i >= j {
//         // No results
//     } else if terminals.contains(nt) {
//         if i + 1 == j && tokens[i] == nt {
//             results.push(Node::Leaf(nt.to_owned()));
//         }
//     } else if non_terminals.contains(nt) {
//         let rules = lookup_rules(grammar, nt);
//         for production in rules {
//             let n = production.len();
//             if n == 1 {
//                 let symbol = &production[0];
//                 if terminals.contains(symbol) {
//                     if i + 1 == j && tokens[i] == *symbol {
//                         results.push(Node::Internal(nt.to_owned(), vec![Node::Leaf(symbol.to_string())]));
//                     }
//                 } else {
//                     let sub_trees = parse(grammar, tokens, non_terminals, terminals, memo, symbol, i, j);
//                     for sub_tree in sub_trees {
//                         results.push(Node::Internal(nt.to_owned(), vec![sub_tree]));
//                     }
//                 }
//             } else {
//                 for splits in possible_splits(i, j, n) {
//                     let positions = std::iter::once(&i).chain(splits.iter()).chain(std::iter::once(&j)).cloned().collect::<Vec<_>>();
//                     let mut children = Vec::new();
//                     let mut failed = false;
//                     for (idx, ai) in production.iter().enumerate() {
//                         let start = positions[idx];
//                         let end = positions[idx + 1];
//                         let sub_trees = parse(grammar, tokens, non_terminals, terminals, memo, ai, start, end);
//                         if sub_trees.is_empty() {
//                             failed = true;
//                             break;
//                         }
//                         children.push(sub_trees);
//                     }
//                     if !failed {
//                         let combinations = children.iter().fold(vec![vec![]], |acc, sub_trees| {
//                             acc.into_iter().flat_map(|acc_subtree| {
//                                 sub_trees.iter().map(move |t| {
//                                     let mut new_acc = acc_subtree.clone();
//                                     new_acc.push(t.clone());
//                                     new_acc
//                                 })
//                             }).collect::<Vec<_>>()
//                         });
//                         for combination in combinations {
//                             results.push(Node::Internal(nt.to_owned(), combination));
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     memo.insert(key, results.clone());
//     results
// }

// fn main() {
//     // Define the grammar
//     let grammar: Grammar = [
//         ("S", vec![vec!["NP", "VP"]]),
//         ("NP", vec![vec!["Det", "N"], vec!["NP", "PP"]]),
//         ("VP", vec![vec!["V", "NP"], vec!["VP", "PP"]]),
//         ("PP", vec![vec!["P", "NP"]]),
//         ("Det", vec![vec!["the"], vec!["a"]]),
//         ("N", vec![vec!["cat"], vec!["dog"], vec!["telescope"], vec!["park"]]),
//         ("V", vec![vec!["saw"], vec!["walked"]]),
//         ("P", vec![vec!["in"], vec!["with"]]),
//     ].iter().cloned().collect();

//     // Collect non-terminals and terminals
//     let non_terminals: HashSet<&'static str> = grammar.keys().cloned().collect();
//     let terminals: HashSet<&'static str> = grammar.values().flat_map(|prods| {
//         prods.iter().flat_map(|prod| {
//             prod.iter().filter(|sym| !non_terminals.contains(*sym)).cloned()
//         })
//     }).collect();

//     // Input sentence
//     let sentence = "the dog saw a cat in the park";
//     let tokens: Vec<&str> = sentence.split_whitespace().collect();

//     // Parse the sentence starting from 'S'
//     let mut memo = HashMap::new();
//     let trees = parse(&grammar, &tokens, &non_terminals, &terminals, &mut memo, "S", 0, tokens.len());

//     // Print all possible parse trees
//     for (idx, tree) in trees.iter().enumerate() {
//         println!("Parse tree {}:", idx + 1);
//         print!("{}", tree_to_string("", tree));
//     }
// }