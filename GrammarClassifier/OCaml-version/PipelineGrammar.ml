type symbol = Terminal of string | NonTerminal of string
type production = symbol list * symbol list

let parse_symbol s non_terminals terminals =
  if List.exists (function NonTerminal x -> x = s | _ -> false) non_terminals then
    NonTerminal s
  else if List.exists (function Terminal x -> x = s | _ -> false) terminals then
    Terminal s
  else
    failwith ("Unknown symbol: " ^ s)

let parse_production (lhs, rhs) non_terminals terminals =
  let lhs_symbols = List.map (fun s -> parse_symbol s non_terminals terminals) lhs in
  let rhs_symbols = List.map (fun s -> parse_symbol s non_terminals terminals) rhs in
  (lhs_symbols, rhs_symbols)

let parse_productions productions non_terminals terminals =
  List.map (fun production -> parse_production production non_terminals terminals) productions

let is_left_linear productions =
  List.for_all (fun (lhs_symbols, rhs_symbols) ->
    match lhs_symbols, rhs_symbols with
    | [NonTerminal _], [Terminal _] -> true
    | [NonTerminal _], (Terminal _ :: non_terminals) when List.for_all (function NonTerminal _ -> true | _ -> false) non_terminals -> true
    | _ -> false
  ) productions

let is_right_linear productions =
  List.for_all (fun (lhs_symbols, rhs_symbols) ->
    match lhs_symbols, rhs_symbols with
    | [NonTerminal _], [Terminal _] -> true
    | [NonTerminal _], (Terminal _ :: non_terminals) when List.for_all (function NonTerminal _ -> true | _ -> false) non_terminals -> true
    | _ -> false
  ) productions

let is_regular_grammar productions =
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

let classify_grammar non_terminals terminals productions =
  let parsed_productions = parse_productions productions non_terminals terminals in
  if is_regular_grammar parsed_productions then "正则文法"
  else if is_right_linear parsed_productions then "右线性文法"
  else if is_left_linear parsed_productions then "左线性文法"
  else if is_type_1 parsed_productions then "上下文有关文法"
  else if is_type_2 parsed_productions then "上下文无关文法"
  else "未知文法"

let read_symbols prompt =
  Printf.printf "%s (用空格分隔): " prompt;
  let line = read_line () in
  String.split_on_char ' ' line
  |> List.map (fun s -> 
    if String.length s > 0 && s.[0] >= 'A' && s.[0] <= 'Z' then NonTerminal s else Terminal s
  )

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
  let start_symbol = 
    read_line () |> (fun s ->
      match List.find_opt (function NonTerminal x -> x = s | _ -> false) non_terminals with
      | Some (NonTerminal _) -> NonTerminal s
      | _ -> failwith ("Unknown start symbol: " ^ s))
  in

  let grammar_type = classify_grammar non_terminals terminals productions in
  Printf.printf "文法类型: %s\n" grammar_type