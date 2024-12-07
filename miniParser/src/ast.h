#ifndef AST_H
#define AST_H

typedef struct ASTNode {
    enum {
        NODE_PROGRAM,
        NODE_STATEMENTS,
        NODE_STATEMENT,
        NODE_ASSIGNMENT,
        NODE_IF,
        NODE_WHILE,
        NODE_EXPRESSION,
        NODE_LOGICAL_EXPR,
        NODE_TERM,
        NODE_FACTOR,
        NODE_BLOCK
    } type;

    union {
        struct {
            struct ASTNode* statements;
        } program;
        struct {
            struct ASTNode* statement;
            struct ASTNode* statements;
        } statements;
        struct {
            struct ASTNode* statement;
        } statement;
        struct {
            struct ASTNode* statements;
        } block;
        struct {
            struct ASTNode* condition;
            struct ASTNode* then_block;
            struct ASTNode* else_block;
        } if_stmt;
        struct {
            struct ASTNode* condition;
            struct ASTNode* body;
        } while_stmt;
        struct {
            char* id;
            struct ASTNode* expr;
        } assignment;
        struct {
            struct ASTNode* left;
            struct ASTNode* right;
            char* op;
        } logical_expr;
        struct {
            struct ASTNode* left;
            struct ASTNode* right;
            char op;
        } expression;
        struct {
            struct ASTNode* left;
            struct ASTNode* right;
            char op;
        } term;
        struct {
            int data;
            char* id;
        } factor;
    } data;
} ASTNode;

ASTNode* create_program(ASTNode* statements);
ASTNode* create_statements(ASTNode* statement, ASTNode* statements);
ASTNode* create_statement(ASTNode* statement);
ASTNode* create_assignment(char* id, ASTNode* expr);
ASTNode* create_if_statement(ASTNode* condition, ASTNode* then_block, ASTNode* else_block);
ASTNode* create_while_statement(ASTNode* condition, ASTNode* body);
ASTNode* create_expression(ASTNode* left, ASTNode* right, char op);
ASTNode* create_logical_expr(ASTNode* left, ASTNode* right, char* op);
ASTNode* create_term(ASTNode* left, ASTNode* right, char op);
ASTNode* create_factor(int data, char* id);
ASTNode* create_block(ASTNode* program);
void destroy_ast(ASTNode* node);
void print_ast(ASTNode* node, int level);
void run_program();

#endif // AST_H
