%{
#include "mini.tab.h"
#include <stdio.h>
#include <stdlib.h>

int colno = 1;
%}

%option noinput nounput yylineno

%%

"//".*               { colno = 1; }

[0-9]+                  { colno += yyleng;yylval.num = atoi(yytext);return NUM; }


"="                      { colno += yyleng;yylval.id = strdup(yytext);return ASSIGN; }
"else"                   { colno += yyleng;return ELSE; }
"if"                     { colno += yyleng;return IF; }
"while"                  { colno += yyleng;return WHILE; }
"&&"                     { colno += yyleng;yylval.id = strdup(yytext);return LogicalAnd; }
"||"                     { colno += yyleng;yylval.id = strdup(yytext);return LogicalOr; }
"!"                      { colno += yyleng;yylval.id = strdup(yytext);return LogicalNot; }
"=="                     { colno += yyleng;yylval.id = strdup(yytext);return EQ; }
"!="                     { colno += yyleng;yylval.id = strdup(yytext);return NEQ; }
">"                      { colno += yyleng;yylval.id = strdup(yytext);return GT; }
"<"                      { colno += yyleng;yylval.id = strdup(yytext);return LT; }
">="                     { colno += yyleng;yylval.id = strdup(yytext);return GEQ; }
"<="                     { colno += yyleng;yylval.id = strdup(yytext);return LEQ; }

"+"                      { colno += yyleng;yylval.id = strdup(yytext);return PLUS; }
"-"                      { colno += yyleng;yylval.id = strdup(yytext);return MINUS; }
"*"                      { colno += yyleng;yylval.id = strdup(yytext);return TIMES; }
"/"                      { colno += yyleng;yylval.id = strdup(yytext);return DIVIDE; }
"%"                      { colno += yyleng;yylval.id = strdup(yytext);return MOD; }

";"                      { colno += yyleng;return SEMICOLON; }
"("                      { colno += yyleng;return LPAREN; }
")"                      { colno += yyleng;return RPAREN; }
"{"                      { colno += yyleng;return LBRACE; }
"}"                      { colno += yyleng;return RBRACE; }
[a-zA-Z_][a-zA-Z0-9_]*   { colno += yyleng;yylval.id = strdup(yytext);return IDENTIFIER; }

[ \t]                   { colno += yyleng; }
\n                      { colno = 1; }

.                        { fprintf(stderr, "Invalid character: %s line:%d column:%d\n", yytext, yylineno, colno); colno += yyleng; }

%%

int yywrap() {
    return 1;
}
