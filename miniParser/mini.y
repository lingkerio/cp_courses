%{
#include <stdio.h>
#include <stdlib.h>

extern int yylex(void);
extern void yyerror(const char *s);

extern int yylineno;
extern int colno;
%}

%token  NUM
%token  IDENTIFIER
%token ASSIGN
%token IF ELSE WHILE
%token PLUS MINUS TIMES DIVIDE MOD LogicalNot LogicalAnd LogicalOr SEMICOLON LPAREN RPAREN LBRACE RBRACE GEQ LEQ GT LT EQ NEQ

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
    statement
    | program statement
    ;

statement:
      assignment
    | if_statement
    | while_statement
    | SEMICOLON
    | error SEMICOLON               { yyerrok; printf("语法错误，跳过错误的语句\n"); }
    ;

assignment:
      IDENTIFIER ASSIGN expression SEMICOLON
    ;

if_statement:
      IF LPAREN LogicalExpression RPAREN block
      | IF LPAREN LogicalExpression RPAREN block ELSE block
      | IF LPAREN LogicalExpression RPAREN statement ELSE statement
      | IF LPAREN LogicalExpression RPAREN block ELSE statement
      | IF LPAREN LogicalExpression RPAREN statement ELSE block
    ;

while_statement:
      WHILE LPAREN LogicalExpression RPAREN block
        | WHILE LPAREN LogicalExpression RPAREN statement
    ;

block:
      LBRACE program RBRACE
    ;

expression:
      expression PLUS term
    | expression MINUS term
    | term
    ;

term:
      term TIMES factor
    | term DIVIDE factor
    | term MOD factor
    | factor
    ;

factor:
      NUM
    | IDENTIFIER
    | LPAREN expression RPAREN
    | error                           { yyerrok; printf("无效的因子，跳过...\n"); }
    ;

LogicalExpression:
      LogicalExpression LogicalAnd LogicalExpression
    | LogicalExpression LogicalOr LogicalExpression
    | expression EQ expression
    | expression NEQ expression
    | expression GT expression
    | expression LT expression
    | expression GEQ expression
    | expression LEQ expression
    | LogicalNot LogicalExpression
    | LPAREN LogicalExpression RPAREN
    | expression error                           { yyerrok; printf("逻辑表达式错误，跳过...\n"); }
    ;

%%

void yyerror(const char* s) {
    fprintf(stderr, "语法错误: %s line:%d column:%d\n", s, yylineno, colno);
}
