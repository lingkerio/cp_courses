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

let read_symbols prompt =
  Printf.printf "%s (用空格分隔): " prompt;
  let line = read_line () in
  List.map (fun s -> if String.length s > 0 && s.[0] >= 'A' && s.[0] <= 'Z' then NonTerminal s else Terminal s) (String.split_on_char ' ' line)

let read_productions () =
  let rec read_productions_aux acc =
    Printf.printf "输入产生式 (格式: LHS -> RHS，用空格分隔，输入空行结束): ";
    let line = read_line () in
    if line = "" then List.rev acc
    else
      let parts = String.split_on_char '-' line in
      if List.length parts <> 2 || String.length (List.nth parts 1) < 2 || (List.nth parts 1).[0] <> '>' then
        (Printf.printf "格式错误，请重新输入。\n"; read_productions_aux acc)
      else
        let lhs = String.split_on_char ' ' (String.trim (List.nth parts 0)) in
        let rhs = String.split_on_char ' ' (String.trim (String.sub (List.nth parts 1) 1 (String.length (List.nth parts 1) - 1))) in
        read_productions_aux ((lhs, rhs) :: acc)
  in
  read_productions_aux []

let () =
  let non_terminals = read_symbols "输入非终结符" in
  let terminals = read_symbols "输入终结符" in
  let productions = read_productions () in
  Printf.printf "输入开始符号: ";
  let start_symbol = match read_line () with
    | s when List.exists (function NonTerminal x -> x = s | _ -> false) non_terminals -> NonTerminal s
    | s -> failwith ("Unknown start symbol: " ^ s)
  in

  let grammar_type = classify_grammar non_terminals terminals productions start_symbol in
  Printf.printf "文法类型: %d\n" grammar_type
