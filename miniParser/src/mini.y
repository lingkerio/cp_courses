%{
#include <stdio.h>
#include <stdlib.h>
#include "../src/symbolTable.h"
#include "../src/ast.h"

extern int yylex(void);
extern void yyerror(const char *s);

extern int yylineno;
extern int colno;

SymbolTable *symbolTable = NULL;  // 当前符号表
ASTNode *root = NULL;             // 语法树根节点
%}

%code requires {
    #include <stdio.h>
    #include <stdlib.h>
    #include "../src/symbolTable.h"
    #include "../src/ast.h"
}

%union {
    int num;
    char *id;
    ASTNode *node;
}

%token  <num> NUM
%token  <id> IDENTIFIER
%token ASSIGN
%token IF ELSE WHILE
%token <id> PLUS MINUS TIMES DIVIDE MOD LogicalNot LogicalAnd LogicalOr SEMICOLON LPAREN RPAREN LBRACE RBRACE GEQ LEQ GT LT EQ NEQ

%type <node> program statements statement assignment if_statement while_statement block expression term factor LogicalExpression

%start program
%precedence RPAREN
%precedence ELSE
%left LogicalOr
%left LogicalAnd
%right LogicalNot
%nonassoc GEQ LEQ GT LT EQ NEQ
%left PLUS MINUS
%left TIMES DIVIDE MOD

%%

program:
    {symbolTable = create_symbol_table();} statements {$$ = create_program($2); root = $$; run_program(); print_symbol_table(symbolTable);}
    ;

statements:
    statement {$$ = create_statements($1, NULL);}
    | statement statements {$$ = create_statements($1, $2);}
    ;

statement:
      assignment { $$ = $1; }
    | if_statement { $$ = $1; }
    | while_statement { $$ = $1; }
    | SEMICOLON { $$ = NULL; }
    | error SEMICOLON               { yyerrok; printf("语法错误，跳过错误的语句\n"); }
    | error RBRACE                  { yyerrok; printf("语法错误，跳过到下一个语句块\n"); }
    ;

assignment:
      IDENTIFIER ASSIGN expression SEMICOLON { $$ = create_assignment($1, $3); }
    ;

if_statement:
      IF LPAREN LogicalExpression RPAREN block { $$ = create_if_statement($3, $5, NULL); }
      | IF LPAREN LogicalExpression RPAREN block ELSE block { $$ = create_if_statement($3, $5, $7); }
      | IF LPAREN LogicalExpression RPAREN statement ELSE statement { $$ = create_if_statement($3, $5, $7); }
      | IF LPAREN LogicalExpression RPAREN block ELSE statement { $$ = create_if_statement($3, $5, $7); }
      | IF LPAREN LogicalExpression RPAREN statement ELSE block { $$ = create_if_statement($3, $5, $7); }
    ;

while_statement:
      WHILE LPAREN LogicalExpression RPAREN block { $$ = create_while_statement($3, $5); }
      | WHILE LPAREN LogicalExpression RPAREN statement { $$ = create_while_statement($3, $5); }
    ;

block:
      LBRACE {enter_scope(&symbolTable);} statements {exit_scope(&symbolTable);} RBRACE { $$ = $3; }
    ;

expression:
      expression PLUS term { $$ = create_expression($1, $3, *$2); }
    | expression MINUS term { $$ = create_expression($1, $3, *$2); }
    | term { $$ = $1; }
    ;

term:
      term TIMES factor { $$ = create_term($1, $3, *$2); }
    | term DIVIDE factor { $$ = create_term($1, $3, *$2); }
    | term MOD factor { $$ = create_term($1, $3, *$2); }
    | factor {$$ = $1;}
    ;

factor:
      NUM { $$ = create_factor($1, NULL); }
    | IDENTIFIER { SymbolEntry* tmp = find_variable(symbolTable, $1); $$ = create_factor(0, $1); }
    | LPAREN expression RPAREN { $$ = $2; }
    | error                           { yyerrok; printf("无效的因子，跳过...\n"); }
    ;

LogicalExpression:
      LogicalExpression LogicalAnd LogicalExpression {$$ = create_logical_expr($1, $3, $2);}
    | LogicalExpression LogicalOr LogicalExpression {$$ = create_logical_expr($1, $3, $2);}
    | expression EQ expression {$$ = create_logical_expr($1, $3, $2);}
    | expression NEQ expression{$$ = create_logical_expr($1, $3, $2);}
    | expression GT expression {$$ = create_logical_expr($1, $3, $2);}
    | expression LT expression {$$ = create_logical_expr($1, $3, $2);}
    | expression GEQ expression{$$ = create_logical_expr($1, $3, $2);}
    | expression LEQ expression {$$ = create_logical_expr($1, $3, $2);}
    | LogicalNot LogicalExpression {$$ = create_logical_expr($2, NULL, $1);}
    | LPAREN LogicalExpression RPAREN { $$ = $2; }
    | expression error                           { yyerrok; printf("逻辑表达式错误，跳过...\n"); }
    ;

%%

void yyerror(const char* s) {
    fprintf(stderr, "语法错误: %s line:%d column:%d\n", s, yylineno, colno);
}
