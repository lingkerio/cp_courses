#include <stdio.h>

extern int yyparse(void);
extern FILE* yyin;

int main(int argc, char* argv[])
{
    if (argc > 1) {
        FILE* file = fopen(argv[1], "r");
        if (file) {
            yyin = file;
            yyparse();
            fclose(file);
        } else {
            perror("Unable to open file");
        }
    } else {
        yyparse();
    }
    return 0;
}
