type symbol = Terminal of string | NonTerminal of string
type production = symbol list * symbol list

let is_terminal = function
  | Terminal _ -> true
  | NonTerminal _ -> false

let is_non_terminal = function
  | NonTerminal _ -> true
  | Terminal _ -> false

let parse_symbol s non_terminals terminals =
  if List.exists (function NonTerminal x -> x = s | _ -> false) non_terminals then
    NonTerminal s
  else if List.exists (function Terminal x -> x = s | _ -> false) terminals then
    Terminal s
  else
    failwith ("Unknown symbol: " ^ s)

let parse_production lhs rhs non_terminals terminals =
  let lhs_symbols = List.map (fun s -> parse_symbol s non_terminals terminals) lhs in
  let rhs_symbols = List.map (fun s -> parse_symbol s non_terminals terminals) rhs in
  (lhs_symbols, rhs_symbols)

let parse_productions productions non_terminals terminals =
  List.map (fun (lhs, rhs) -> parse_production lhs rhs non_terminals terminals) productions

let is_type_3 productions =
  List.for_all (fun (lhs_symbols, rhs_symbols) ->
    match lhs_symbols, rhs_symbols with
    | [NonTerminal _], [Terminal _] -> true
    | [NonTerminal _], [Terminal _; NonTerminal _] -> true
    | _ -> false
  ) productions

let is_type_2 productions =
  List.for_all (fun (lhs_symbols, _) ->
    match lhs_symbols with
    | [NonTerminal _] -> true
    | _ -> false
  ) productions

let is_type_1 productions =
  List.for_all (fun (lhs_symbols, rhs_symbols) ->
    List.length lhs_symbols >= 1 && List.length rhs_symbols >= 1 && List.length lhs_symbols <= List.length rhs_symbols
  ) productions

let classify_grammar non_terminals terminals productions start_symbol =
  let parsed_productions = parse_productions productions non_terminals terminals in
  if is_type_3 parsed_productions then 3
  else if is_type_2 parsed_productions then 2
  else if is_type_1 parsed_productions then 1
  else 0

(* 示例输入：上下文无关文法 *)
let non_terminals = [NonTerminal "S"; NonTerminal "A"; NonTerminal "B"]
let terminals = [Terminal "a"; Terminal "b"]
let productions = [
  (["S"], ["A"; "B"]);
  (["A"], ["a"]);
  (["B"], ["b"])
]
let start_symbol = NonTerminal "S"

(* 调用函数 *)
let grammar_type = classify_grammar non_terminals terminals productions start_symbol
let () = Printf.printf "文法类型: %d\n" grammar_type