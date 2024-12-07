#include "symbolTable.h"
#include "ast.h"

// 创建符号表
SymbolTable* create_symbol_table(void)
{
    SymbolTable* table = (SymbolTable*)malloc(sizeof(SymbolTable));
    table->entries = NULL;
    table->next_scope = NULL;
    return table;
}

// 销毁符号表
void destroy_symbol_table(SymbolTable* table)
{
    if (table == NULL)
        return;

    SymbolEntry* entry = table->entries;
    while (entry != NULL) {
        SymbolEntry* temp = entry;
        entry = entry->next;
        free(temp->name);
        free(temp);
    }
    free(table);
}

// 插入变量
void insert_variable(SymbolTable* table, const char* name, int data)
{
    SymbolEntry* entry = find_variable(table, name);
    if (entry != NULL) {
        entry->data = data;
        return;
    }

    SymbolEntry* new_entry = (SymbolEntry*)malloc(sizeof(SymbolEntry));
    new_entry->name = strdup(name);
    new_entry->data = data;
    new_entry->next = table->entries;
    table->entries = new_entry;

    return; // 插入成功
}

// 查找变量
SymbolEntry* find_variable(SymbolTable* table, const char* name)
{
    SymbolEntry* entry = table->entries;
    while (entry != NULL) {
        if (strcmp(entry->name, name) == 0) {
            return entry;
        }
        entry = entry->next;
    }
    if (table->next_scope != NULL) {
        return find_variable(table->next_scope, name); // 父作用域查找
    }
    return NULL; // 未找到
}

// 进入新作用域
void enter_scope(SymbolTable** table)
{
    SymbolTable* new_scope = create_symbol_table();
    new_scope->next_scope = *table;
    *table = new_scope;
}

// 退出作用域
void exit_scope(SymbolTable** table)
{
    SymbolTable* old_scope = *table;
    *table = old_scope->next_scope;
    destroy_symbol_table(old_scope);
}

void print_symbol_table(SymbolTable *table)
{
    SymbolEntry *entry = table->entries;
    while (entry != NULL) {
        printf("%s: %d\n", entry->name, entry->data);
        entry = entry->next;
    }
    if (table->next_scope != NULL) {
        print_symbol_table(table->next_scope);
    }
}
