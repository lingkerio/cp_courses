(* 定义 Token 枚举类型 *)
type token =
  | Number of int
  | Operator of char
  | LParen
  | RParen

(* 定义 Lexer 结构体 *)
type lexer = {
  input_string : string;
  mutable position : int;
}

(* 创建新的 Lexer *)
let new_lexer input_string = {
  input_string;
  position = 0;
}

(* 获取下一个 Token *)
let rec next_token lexer =
  if lexer.position >= String.length lexer.input_string then
    None
  else
    let current_char = String.get lexer.input_string lexer.position in

    (* 跳过空白字符 *)
    if current_char = ' ' || current_char = '\t' || current_char = '\n' then (
      lexer.position <- lexer.position + 1;
      next_token lexer
    ) else if '0' <= current_char && current_char <= '9' then (
      (* 处理数字 *)
      let rec extract_number pos =
        if pos < String.length lexer.input_string && '0' <= String.get lexer.input_string pos && String.get lexer.input_string pos <= '9' then
          extract_number (pos + 1)
        else
          pos
        in
        let end_pos = extract_number lexer.position in
        let number_str = String.sub lexer.input_string lexer.position (end_pos - lexer.position) in
        lexer.position <- end_pos;
        Some (Number (int_of_string number_str))
    ) else if current_char = '+' || current_char = '-' || current_char = '*' || current_char = '/' || current_char = '(' || current_char = ')' then (
      (* 处理操作符和括号 *)
      lexer.position <- lexer.position + 1;
      match current_char with
      | '(' -> Some LParen
      | ')' -> Some RParen
      | _ -> Some (Operator current_char)
    ) else
      failwith ("Unexpected character: " ^ Char.escaped current_char)

(* 将输入字符串转换为 Token 列表 *)
let rec tokenize lexer =
  match next_token lexer with
  | Some token -> token :: tokenize lexer
  | None -> []

(* 定义 Expr 枚举类型 *)
type expr =
  | Number of int
  | Binary of token * expr * expr

(* 定义 Parser 结构体 *)
type parser = {
  tokens : token list;
  mutable position : int;
}

(* 创建新的 Parser *)
let new_parser tokens = {
  tokens;
  position = 0;
}

(* 获取当前 Token *)
let current_token parser =
  if parser.position < List.length parser.tokens then
    Some (List.nth parser.tokens parser.position)
  else
    None

(* 解析表达式 *)
let rec expression parser =
  let node = term parser in
  match current_token parser with
  | Some (Operator ('+' | '-')) as op ->
    parser.position <- parser.position + 1;
    let right_node = term parser in
    Binary (Option.get op, node, right_node)
  | _ -> node

(* 解析项 *)
and term parser =
  let node = factor parser in
  match current_token parser with
  | Some (Operator ('*' | '/')) as op ->
    parser.position <- parser.position + 1;
    let right_node = factor parser in
    Binary (Option.get op, node, right_node)
  | _ -> node

(* 解析因子 *)
and factor parser =
  match current_token parser with
  | Some (Number value) ->
    parser.position <- parser.position + 1;
    Number value
  | Some LParen ->
    parser.position <- parser.position + 1;
    let node = expression parser in
    (match current_token parser with
    | Some RParen -> parser.position <- parser.position + 1; node
    | _ -> failwith "Expected closing parenthesis")
  | _ -> failwith "Invalid syntax"

(* 计算表达式的值 *)
let rec evaluate = function
  | Number value -> value
  | Binary (Operator op, left, right) ->
    let left_val = evaluate left in
    let right_val = evaluate right in
    (match op with
    | '+' -> left_val + right_val
    | '-' -> left_val - right_val
    | '*' -> left_val * right_val
    | '/' -> left_val / right_val
    | _ -> failwith "Unexpected operator")
  | _ -> failwith "Unexpected expression"

(* 计算输入字符串的值 *)
let calculate input_string =
  let lexer = new_lexer input_string in
  let tokens = tokenize lexer in
  let parser = new_parser tokens in
  let ast = expression parser in
  evaluate ast

(* 主函数 *)
let () =
  let result = calculate "3 + 5 * (2 - 8)" in
  Printf.printf "%d\n" result