#ifndef SYMBOLTABLE_H
#define SYMBOLTABLE_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ast.h"


// 符号类型定义
typedef enum {
    SYMBOL_VAR // 变量
} SymbolType;

// 符号表项
typedef struct SymbolEntry {
    char* name; // 符号名称
    int data;
    struct SymbolEntry* next; // 链表指针
} SymbolEntry;

// 符号表
typedef struct SymbolTable {
    SymbolEntry* entries; // 符号表的条目
    struct SymbolTable* next_scope; // 下一个作用域符号表（链式存储）
} SymbolTable;

// 符号表操作函数
SymbolTable* create_symbol_table();
void destroy_symbol_table(SymbolTable* table);
void insert_variable(SymbolTable* table, const char* name, int data);
SymbolEntry* find_variable(SymbolTable* table, const char* name);
void enter_scope(SymbolTable** table);
void exit_scope(SymbolTable** table);
void print_symbol_table(SymbolTable* table);

#endif // SYMBOLTABLE_H
