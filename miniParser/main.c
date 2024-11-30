#include <stdio.h>

extern int yyparse(void);
extern FILE* yyin;

int main(int argc, char* argv[])
{
    if (argc > 1) {
        FILE* file = fopen(argv[1], "r");
        if (file) {
            yyin = file;
            if (yyparse() == 0) {
                printf("Parsing successful\n");
            } else {
                printf("Parsing failed\n");
            }
            fclose(file);
        } else {
            perror("Unable to open file");
        }
    } else {
        if (yyparse() == 0) {
            printf("Parsing successful\n");
        } else {
            printf("Parsing failed\n");
        }
    }
    return 0;
}
