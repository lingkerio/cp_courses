(* Function to eliminate epsilon productions *)
let remove_epsilons grammar =
  let rec remove_eps_from_rhs rhs_list eps_nonterms =
    match rhs_list with
    | [] -> [[]]
    | hd :: tl ->
      let rest = remove_eps_from_rhs tl eps_nonterms in
      if List.mem hd eps_nonterms then
        let new_rest = rest @ (List.map (fun r -> hd :: r) rest) in
        new_rest
      else
        List.map (fun r -> hd :: r) rest
  in

  let rec find_eps_nonterms grammar =
    grammar
    |> List.fold_left (fun acc (lhs, rhs_list) ->
        if List.exists ((=) ["ε"]) rhs_list then lhs :: acc else acc
      ) []
  in

  let rec update_eps_nonterms eps_nonterms grammar =
    let new_eps_nonterms =
      grammar
      |> List.fold_left (fun acc (lhs, rhs_list) ->
          if List.exists (fun rhs -> List.for_all (fun sym -> List.mem sym eps_nonterms) rhs) rhs_list then
            lhs :: acc
          else
            acc
        ) eps_nonterms
    in
    if List.length new_eps_nonterms > List.length eps_nonterms then
      update_eps_nonterms new_eps_nonterms grammar
    else
      new_eps_nonterms
  in

  let eps_nonterms = update_eps_nonterms (find_eps_nonterms grammar) grammar in

  let new_grammar =
    grammar
    |> List.fold_left (fun acc (lhs, rhs_list) ->
        let new_rhs_list =
          rhs_list
          |> List.fold_left (fun acc_rhs rhs ->
              if rhs = ["ε"] then acc_rhs
              else acc_rhs @ (remove_eps_from_rhs rhs eps_nonterms)
            ) []
        in
        (lhs, new_rhs_list) :: acc
      ) []
  in

  let rec generate_new_productions grammar eps_nonterms acc =
    match grammar with
    | [] -> acc
    | (lhs, rhs_list) :: tl ->
      let new_acc =
        List.fold_left (fun acc rhs ->
          if List.exists (fun sym -> List.mem sym eps_nonterms) rhs then
            let new_rhs = List.filter (fun sym -> not (List.mem sym eps_nonterms)) rhs in
            if new_rhs <> [] && not (List.mem (lhs, new_rhs) acc) then
              (lhs, new_rhs) :: acc
            else
              acc
          else
            acc
        ) acc rhs_list
      in
      generate_new_productions tl eps_nonterms new_acc
  in

  let new_productions = generate_new_productions new_grammar eps_nonterms [] in

  (new_grammar, new_productions)

(* Example grammar *)
let grammar = [
  ("E", [["L"]; ["a"]]);
  ("L", [["("; "S"; ")"]]);
  ("S", [["E"; "A"]]);
  ("A", [["E"; "A"]; ["ε"]])
]

(* Grammar and epsilon non-terminals after eliminating epsilon productions *)
let grammar, new_productions = remove_epsilons grammar

(* Output the new grammar *)
let () =
  Printf.printf "New grammar:\n";
  List.iter (fun (lhs, rhs_list) ->
    Printf.printf "%s -> %s\n" lhs (String.concat " | " (List.map (fun rhs -> String.concat " " rhs) rhs_list))
  ) grammar;
  Printf.printf "New productions:\n";
  List.iter (fun (lhs, rhs) ->
    Printf.printf "%s -> %s\n" lhs (String.concat " " rhs)
  ) new_productions 

(* Helper function to add an element to a list if it's not already present *)
let add_unique elem lst =
  if List.mem elem lst then lst else elem :: lst

(* Collect non-terminals *)
let non_terminals = 
  List.fold_left (fun acc (lhs, _) -> add_unique lhs acc) [] grammar

(* Collect terminals *)
let terminals = 
  List.fold_left 
    (fun acc (_, prods) -> 
      List.fold_left 
        (fun acc prod -> 
          List.fold_left 
            (fun acc sym -> 
              if List.mem sym non_terminals then acc else add_unique sym acc) 
            acc prod) 
        acc prods) 
    [] grammar

(* Input sentence *)
let sentence = "( a a a )"
let tokens = String.split_on_char ' ' sentence

(* ParseTree type to represent parse trees *)
type parse_tree = 
  | Node of string * parse_tree list
  | Leaf of string

(* Utility function to convert a parse tree to a string for printing *)
let rec tree_to_string indent = function
  | Leaf s -> indent ^ s ^ "\n"
  | Node (label, children) ->
      let result = indent ^ label ^ "\n" in
      List.fold_left 
        (fun acc child -> acc ^ (tree_to_string (indent ^ "  ") child)) 
        result children

(* Function to convert a parse tree to typst_tree_code *)
let rec tree_to_typst indent = function
  | Leaf s -> Printf.sprintf "%stree(\"%s\")" indent s
  | Node (label, children) ->
      let new_indent = indent ^ "  " in
      let children_typst = List.map (tree_to_typst new_indent) children in
      let children_str = String.concat ",\n" children_typst in
      Printf.sprintf "%stree(\"%s\",\n%s\n%s)" indent label children_str indent

let rec possible_splits i j n =
  if n = 1 then
    if i < j then [[]] else []
  else
    let positions = List.init (j - i - 1) (fun k -> k + i + 1) in
    let rec combinations acc remaining_positions k =
      match k with
      | 0 -> [List.rev acc]
      | _ -> 
          List.flatten (
            List.mapi (fun idx pos ->
              let new_positions = List.filteri (fun i _ -> i > idx) remaining_positions in
              let new_acc = pos :: acc in
              combinations new_acc new_positions (k-1)
            ) remaining_positions
          )
    in
    combinations [] positions (n - 1)

(* Lookup grammar rules for a non-terminal *)
let lookup_rules nt = 
  try List.assoc nt grammar 
  with Not_found -> []

(* Memoization using purely functional approach *)
let rec parse memo nt i j =
  let key = (nt, i, j) in
  if List.mem_assoc key memo then
    List.assoc key memo, memo
  else
    let results, new_memo = 
      if i >= j then ([], memo)
      else if List.mem nt terminals then
        if i + 1 = j && List.nth tokens i = nt then
          ([Leaf nt], memo)
        else ([], memo)
      else if List.mem nt non_terminals then
        let rules = lookup_rules nt in
        List.fold_left (fun (acc_results, acc_memo) production ->
          let n = List.length production in
          if n = 1 then
            let symbol = List.hd production in
            if List.mem symbol terminals then
              if i + 1 = j && List.nth tokens i = symbol then
                (Node (nt, [Leaf symbol]) :: acc_results, acc_memo)
              else (acc_results, acc_memo)
            else
              let sub_trees, updated_memo = parse acc_memo symbol i j in
              (List.map (fun sub_tree -> Node (nt, [sub_tree])) sub_trees @ acc_results, updated_memo)
          else
            List.fold_left (fun (acc_res, acc_m) splits ->
              let positions = i :: splits @ [j] in
              let children, new_acc_memo, failed =
                List.fold_left (fun (children_acc, curr_memo, fail) idx ->
                  let ai = List.nth production idx in
                  let start = List.nth positions idx in
                  let end_ = List.nth positions (idx + 1) in
                  if fail then (children_acc, curr_memo, true)
                  else
                    let sub_trees, updated_memo = parse curr_memo ai start end_ in
                    if sub_trees = [] then (children_acc, updated_memo, true)
                    else (sub_trees :: children_acc, updated_memo, false))
                  ([], acc_m, false) (List.init n (fun x -> x))
              in
              if failed then (acc_res, acc_m)
              else
                let combinations = List.fold_left 
                  (fun acc sub_trees -> 
                    List.flatten (List.map (fun acc_subtree -> List.map (fun t -> acc_subtree @ [t]) sub_trees) acc)) 
                  [[]] (List.rev children) in
                let new_results = List.map (fun children -> Node (nt, children)) combinations in
                (new_results @ acc_res, new_acc_memo)
            ) (acc_results, acc_memo) (possible_splits i j n)
        ) ([], memo) rules
      else ([], memo)
    in
    (results, (key, results) :: new_memo)

(* Optimized version: Avoid repeated lookups and exception handling overhead *)
let rec parse memo nt i j =
  let key = (nt, i, j) in
  try
    let value = List.assoc key memo in
    value, memo
  with Not_found ->
    let results, new_memo = 
      if i >= j then ([], memo)
      else if List.mem nt terminals then
        if i + 1 = j && List.nth tokens i = nt then
          ([Leaf nt], memo)
        else ([], memo)
      else if List.mem nt non_terminals then
        let rules = lookup_rules nt in
        List.fold_left (fun (acc_results, acc_memo) production ->
          let n = List.length production in
          if n = 1 then
            let symbol = List.hd production in
            if List.mem symbol terminals then
              if i + 1 = j && List.nth tokens i = symbol then
                (Node (nt, [Leaf symbol]) :: acc_results, acc_memo)
              else (acc_results, acc_memo)
            else
              let sub_trees, updated_memo = parse acc_memo symbol i j in
              (List.map (fun sub_tree -> Node (nt, [sub_tree])) sub_trees @ acc_results, updated_memo)
          else
            List.fold_left (fun (acc_res, acc_m) splits ->
              let positions = i :: splits @ [j] in
              let children, new_acc_memo, failed =
                List.fold_left (fun (children_acc, curr_memo, fail) idx ->
                  let ai = List.nth production idx in
                  let start = List.nth positions idx in
                  let end_ = List.nth positions (idx + 1) in
                  if fail then (children_acc, curr_memo, true)
                  else
                    let sub_trees, updated_memo = parse curr_memo ai start end_ in
                    if sub_trees = [] then (children_acc, updated_memo, true)
                    else (sub_trees :: children_acc, updated_memo, false))
                  ([], acc_m, false) (List.init n (fun x -> x))
              in
              if failed then (acc_res, acc_m)
              else
                let combinations = List.fold_left 
                  (fun acc sub_trees -> 
                    List.flatten (List.map (fun acc_subtree -> List.map (fun t -> acc_subtree @ [t]) sub_trees) acc)) 
                  [[]] (List.rev children) in
                let new_results = List.map (fun children -> Node (nt, children)) combinations in
                (new_results @ acc_res, new_acc_memo)
            ) (acc_results, acc_memo) (possible_splits i j n)
        ) ([], memo) rules
      else ([], memo)
    in
    (results, (key, results) :: new_memo)

(* Parse the sentence starting from 'S' *)
let trees, _ = parse [] "S" 0 (List.length tokens)

(* Optimized version using Hashtbl: Avoid repeated lookups and exception handling overhead *)
let rec parse memo nt i j =
  let key = (nt, i, j) in
  if Hashtbl.mem memo key then
    Hashtbl.find memo key, memo
  else
    let results, new_memo = 
      if i >= j then ([], memo)
      else if List.mem nt terminals then
        if i + 1 = j && List.nth tokens i = nt then
          ([Leaf nt], memo)
        else ([], memo)
      else if List.mem nt non_terminals then
        let rules = lookup_rules nt in
        List.fold_left (fun (acc_results, acc_memo) production ->
          let n = List.length production in
          if n = 1 then
            let symbol = List.hd production in
            if List.mem symbol terminals then
              if i + 1 = j && List.nth tokens i = symbol then
                (Node (nt, [Leaf symbol]) :: acc_results, acc_memo)
              else (acc_results, acc_memo)
            else
              let sub_trees, updated_memo = parse acc_memo symbol i j in
              (List.map (fun sub_tree -> Node (nt, [sub_tree])) sub_trees @ acc_results, updated_memo)
          else
            List.fold_left (fun (acc_res, acc_m) splits ->
              let positions = i :: splits @ [j] in
              let children, new_acc_memo, failed =
                List.fold_left (fun (children_acc, curr_memo, fail) idx ->
                  let ai = List.nth production idx in
                  let start = List.nth positions idx in
                  let end_ = List.nth positions (idx + 1) in
                  if fail then (children_acc, curr_memo, true)
                  else
                    let sub_trees, updated_memo = parse curr_memo ai start end_ in
                    if sub_trees = [] then (children_acc, updated_memo, true)
                    else (sub_trees :: children_acc, updated_memo, false))
                  ([], acc_m, false) (List.init n (fun x -> x))
              in
              if failed then (acc_res, acc_m)
              else
                let combinations = List.fold_left 
                  (fun acc sub_trees -> 
                    List.flatten (List.map (fun acc_subtree -> List.map (fun t -> acc_subtree @ [t]) sub_trees) acc)) 
                  [[]] (List.rev children) in
                let new_results = List.map (fun children -> Node (nt, children)) combinations in
                (new_results @ acc_res, new_acc_memo)
            ) (acc_results, acc_memo) (possible_splits i j n)
        ) ([], memo) rules
      else ([], memo)
    in
    Hashtbl.add new_memo key results;
    (results, new_memo)

(* Initialize memo *)
let memo = Hashtbl.create 100

(* Parse the sentence starting from 'S' *)
let trees, _ = parse memo "S" 0 (List.length tokens)

(* Traverse the parse tree and add the removed non-terminals *)
let rec add_epsilons_to_tree tree new_productions =
  match tree with
  | Leaf _ -> tree
  | Node (label, children) ->
      let new_children = List.map (fun child -> add_epsilons_to_tree child new_productions) children in
      let new_children_with_epsilons =
        List.fold_left (fun acc (lhs, rhs) ->
          if lhs = label && List.map (function Node (l, _) -> l | Leaf l -> l) new_children = rhs then
            Node (lhs, new_children @ [Node (lhs, [Leaf "ε"])]) :: acc
          else
            acc
        ) [Node (label, new_children)] new_productions
      in
      List.hd new_children_with_epsilons

(* Print all possible parse trees *)
let () = 
  List.iteri (fun idx tree ->
    let tree_with_epsilons = add_epsilons_to_tree tree new_productions in
    Printf.printf "Parse tree %d:\n" (idx + 1);
    print_string (tree_to_string "" tree_with_epsilons);
    Printf.printf "Typst tree code %d:\n" (idx + 1);
    print_string ("#" ^ tree_to_typst "" tree_with_epsilons)
  ) trees