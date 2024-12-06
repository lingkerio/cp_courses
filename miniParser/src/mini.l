%{
#include "mini.tab.h"
#include <stdio.h>
#include <stdlib.h>

int colno = 1;
%}

%option noinput nounput yylineno

%%

"//".*               { colno = 1; }

[0-9]+                  { colno += yyleng;return NUM; }


"="                      { colno += yyleng;return ASSIGN; }
"if"                     { colno += yyleng;return IF; }
"else"                   { colno += yyleng;return ELSE; }
"while"                  { colno += yyleng;return WHILE; }
"&&"                     { colno += yyleng;return LogicalAnd; }
"||"                     { colno += yyleng;return LogicalOr; }
"!"                      { colno += yyleng;return LogicalNot; }
"=="                     { colno += yyleng;return EQ; }
"!="                     { colno += yyleng;return NEQ; }
">"                      { colno += yyleng;return GT; }
"<"                      { colno += yyleng;return LT; }
">="                     { colno += yyleng;return GEQ; }
"<="                     { colno += yyleng;return LEQ; }

"+"                      { colno += yyleng;return PLUS; }
"-"                      { colno += yyleng;return MINUS; }
"*"                      { colno += yyleng;return TIMES; }
"/"                      { colno += yyleng;return DIVIDE; }
"%"                      { colno += yyleng;return MOD; }

";"                      { colno += yyleng;return SEMICOLON; }
"("                      { colno += yyleng;return LPAREN; }
")"                      { colno += yyleng;return RPAREN; }
"{"                      { colno += yyleng;return LBRACE; }
"}"                      { colno += yyleng;return RBRACE; }
[a-zA-Z_][a-zA-Z0-9_]*   { colno += yyleng;return IDENTIFIER; }

[ \t]                   { colno += yyleng; }
\n                      { colno = 1; }

.                        { fprintf(stderr, "Invalid character: %s line:%d column:%d\n", yytext, yylineno, colno); colno += yyleng; }

%%

int yywrap() {
    return 1;
}
