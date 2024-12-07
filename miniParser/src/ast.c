#include "ast.h"
#include "symbolTable.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

ASTNode* create_program(ASTNode* statements)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_PROGRAM;
    node->data.program.statements = statements;
    return node;
}

ASTNode* create_statements(ASTNode* statement, ASTNode* statements)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_STATEMENTS;
    node->data.statements.statement = statement;
    node->data.statements.statements = statements;
    return node;
}

ASTNode* create_assignment(char* id, ASTNode* expr)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_ASSIGNMENT;
    node->data.assignment.id = strdup(id);
    node->data.assignment.expr = expr;
    return node;
}

ASTNode* create_if_statement(ASTNode* condition, ASTNode* then_block, ASTNode* else_block)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_IF;
    node->data.if_stmt.condition = condition;
    node->data.if_stmt.then_block = then_block;
    node->data.if_stmt.else_block = else_block;
    return node;
}

ASTNode* create_while_statement(ASTNode* condition, ASTNode* body)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_WHILE;
    node->data.while_stmt.condition = condition;
    node->data.while_stmt.body = body;
    return node;
}

ASTNode* create_expression(ASTNode* left, ASTNode* right, char op)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_EXPRESSION;
    node->data.expression.left = left;
    node->data.expression.right = right;
    node->data.expression.op = op;
    return node;
}

ASTNode* create_logical_expr(ASTNode* left, ASTNode* right, char* op)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_LOGICAL_EXPR;
    node->data.logical_expr.left = left;
    node->data.logical_expr.right = right;
    node->data.logical_expr.op = strdup(op);
    return node;
}

ASTNode* create_term(ASTNode* left, ASTNode* right, char op)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_TERM;
    node->data.term.left = left;
    node->data.term.right = right;
    node->data.term.op = op;
    return node;
}

ASTNode* create_factor(int data, char* id)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_FACTOR;
    node->data.factor.data = data;
    node->data.factor.id = id ? strdup(id) : NULL;
    return node;
}

ASTNode* create_block(ASTNode* statements)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = NODE_BLOCK;
    node->data.block.statements = statements;
    return node;
}

void destroy_ast(ASTNode* node)
{
    if (node == NULL)
        return;

    switch (node->type) {
    case NODE_PROGRAM:
        destroy_ast(node->data.program.statements);
        break;
    case NODE_STATEMENTS:
        destroy_ast(node->data.statements.statement);
        destroy_ast(node->data.statements.statements);
        break;
    case NODE_ASSIGNMENT:
        destroy_ast(node->data.assignment.expr);
        free(node->data.assignment.id);
        break;
    case NODE_IF:
        destroy_ast(node->data.if_stmt.condition);
        destroy_ast(node->data.if_stmt.then_block);
        destroy_ast(node->data.if_stmt.else_block);
        break;
    case NODE_WHILE:
        destroy_ast(node->data.while_stmt.condition);
        destroy_ast(node->data.while_stmt.body);
        break;
    case NODE_EXPRESSION:
        destroy_ast(node->data.expression.left);
        destroy_ast(node->data.expression.right);
        break;
    case NODE_LOGICAL_EXPR:
        destroy_ast(node->data.logical_expr.left);
        destroy_ast(node->data.logical_expr.right);
        free(node->data.logical_expr.op);
        break;
    case NODE_TERM:
        destroy_ast(node->data.term.left);
        destroy_ast(node->data.term.right);
        break;
    case NODE_FACTOR:
        free(node->data.factor.id);
        break;
    case NODE_BLOCK:
        destroy_ast(node->data.block.statements);
        break;
    default:
        break;
    }

    free(node);
}

void print_ast(ASTNode* node, int level)
{
    if (node == NULL)
        return;

    for (int i = 0; i < level; i++)
        printf("  ");

    switch (node->type) {
    case NODE_PROGRAM:
        printf("Program\n");
        print_ast(node->data.program.statements, level + 1);
        break;
    case NODE_STATEMENTS:
        printf("Statements\n");
        print_ast(node->data.statements.statement, level + 1);
        print_ast(node->data.statements.statements, level + 1);
        break;
    case NODE_ASSIGNMENT:
        printf("Assignment: %s\n", node->data.assignment.id);
        print_ast(node->data.assignment.expr, level + 1);
        break;
    case NODE_IF:
        printf("If\n");
        print_ast(node->data.if_stmt.condition, level + 1);
        print_ast(node->data.if_stmt.then_block, level + 1);
        print_ast(node->data.if_stmt.else_block, level + 1);
        break;
    case NODE_WHILE:
        printf("While\n");
        print_ast(node->data.while_stmt.condition, level + 1);
        print_ast(node->data.while_stmt.body, level + 1);
        break;
    case NODE_EXPRESSION:
        printf("Expression: %c\n", node->data.expression.op);
        print_ast(node->data.expression.left, level + 1);
        print_ast(node->data.expression.right, level + 1);
        break;
    case NODE_LOGICAL_EXPR:
        printf("Logical Expression: %s\n", node->data.logical_expr.op);
        print_ast(node->data.logical_expr.left, level + 1);
        print_ast(node->data.logical_expr.right, level + 1);
        break;
    case NODE_TERM:
        printf("Term: %c\n", node->data.term.op);
        print_ast(node->data.term.left, level + 1);
        print_ast(node->data.term.right, level + 1);
        break;
    case NODE_FACTOR:
        if (node->data.factor.id)
            printf("Factor: %s\n", node->data.factor.id);
        else
            printf("Factor: %d\n", node->data.factor.data);
        break;
    case NODE_BLOCK:
        printf("Block\n");
        print_ast(node->data.block.statements, level + 1);
        break;
    }
}

extern SymbolTable* symbolTable;
extern ASTNode* root;

void run_program();
void run_statements(ASTNode* statements);
void run_statement(ASTNode* statement);
void run_assignment(ASTNode* assignment);
void run_if_statement(ASTNode* if_stmt);
void run_while_statement(ASTNode* while_stmt);
int evaluate_expression(ASTNode* expression);
int evaluate_logical_expr(ASTNode* logical_expr);
int evaluate_term(ASTNode* term);
int evaluate_factor(ASTNode* factor);

void run_program()
{
    run_statements(root->data.program.statements);
}

void run_statements(ASTNode* statements)
{
    if (statements == NULL)
        return;

    if (statements->type == NODE_STATEMENTS) {
        run_statement(statements->data.statements.statement);
        run_statements(statements->data.statements.statements);
    } else {
        run_statement(statements);
    }
}

void run_statement(ASTNode* statement)
{
    if (statement == NULL)
        return;

    switch (statement->type) {
    case NODE_ASSIGNMENT:
        run_assignment(statement);
        break;
    case NODE_IF:
        run_if_statement(statement);
        break;
    case NODE_WHILE:
        run_while_statement(statement);
        break;
    default:
        break;
    }
}

void run_assignment(ASTNode* assignment)
{
    int value = evaluate_expression(assignment->data.assignment.expr);
    insert_variable(symbolTable, assignment->data.assignment.id, value);
}

void run_if_statement(ASTNode* if_stmt)
{
    int condition = evaluate_logical_expr(if_stmt->data.if_stmt.condition);
    if (condition)
        run_statements(if_stmt->data.if_stmt.then_block);
    else
        run_statements(if_stmt->data.if_stmt.else_block);
}

void run_while_statement(ASTNode* while_stmt)
{
    while (evaluate_logical_expr(while_stmt->data.while_stmt.condition))
        run_statements(while_stmt->data.while_stmt.body);
}

int evaluate_expression(ASTNode* expression)
{
    if (expression->type == NODE_FACTOR)
        return evaluate_factor(expression);

    int left = evaluate_expression(expression->data.expression.left);
    int right = evaluate_expression(expression->data.expression.right);

    switch (expression->data.expression.op) {
    case '+':
        return left + right;
    case '-':
        return left - right;
    case '*':
        return left * right;
    case '/':
        return left / right;
    default:
        return 0;
    }
}

int evaluate_logical_expr(ASTNode* logical_expr)
{
    if (logical_expr->data.logical_expr.right == NULL && strcmp(logical_expr->data.logical_expr.op, "!") == 0)
        return !evaluate_logical_expr(logical_expr->data.logical_expr.left);

    char* op = logical_expr->data.logical_expr.op;
    if (strcmp(op, "&&") == 0)
        return evaluate_logical_expr(logical_expr->data.logical_expr.left) && evaluate_logical_expr(logical_expr->data.logical_expr.right);
    else if (strcmp(op, "||") == 0)
        return evaluate_logical_expr(logical_expr->data.logical_expr.left) || evaluate_logical_expr(logical_expr->data.logical_expr.right);
    else {
        int left = evaluate_expression(logical_expr->data.logical_expr.left);
        int right = evaluate_expression(logical_expr->data.logical_expr.right);

        if (strcmp(op, "==") == 0)
            return left == right;
        else if (strcmp(op, "!=") == 0)
            return left != right;
        else if (strcmp(op, ">") == 0)
            return left > right;
        else if (strcmp(op, "<") == 0)
            return left < right;
        else if (strcmp(op, ">=") == 0)
            return left >= right;
        else if (strcmp(op, "<=") == 0)
            return left <= right;
        else
            return 0;
    }
}

int evaluate_term(ASTNode* term)
{
    if (term->type == NODE_FACTOR)
        return evaluate_factor(term);

    int left = evaluate_term(term->data.term.left);
    int right = evaluate_term(term->data.term.right);

    switch (term->data.term.op) {
    case '*':
        return left * right;
    case '/':
        return left / right;
    default:
        return 0;
    }
}

int evaluate_factor(ASTNode* factor)
{
    if (factor->data.factor.id) {
        SymbolEntry* entry = find_variable(symbolTable, factor->data.factor.id);
        if (entry == NULL)
            fprintf(stderr, "Undefined variable: %s\n", factor->data.factor.id),
            exit(EXIT_FAILURE);
        else
            return entry->data;
    } else
        return factor->data.factor.data;
}
