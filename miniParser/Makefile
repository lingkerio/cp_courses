# Makefile for compiling Mini language parser using Bison and Flex

CC = gcc
FLEX = flex
BISON = bison

CFLAGS = -Wall -g

TARGET = mini_parser

LEX_OUTPUT = lex.yy.c
BISON_OUTPUT_C = mini.tab.c
BISON_OUTPUT_H = mini.tab.h
MAIN = main.c

all: $(TARGET)

$(BISON_OUTPUT_C) $(BISON_OUTPUT_H): mini.y
	$(BISON) -d mini.y

$(LEX_OUTPUT): mini.l
	$(FLEX) mini.l

$(TARGET): $(LEX_OUTPUT) $(BISON_OUTPUT_C)
	$(CC) $(CFLAGS) $(MAIN) $(LEX_OUTPUT) $(BISON_OUTPUT_C) -o $(TARGET) -lfl

clean:
	rm -f $(TARGET) $(LEX_OUTPUT) $(BISON_OUTPUT_C) $(BISON_OUTPUT_H)

run: $(TARGET)
	./$(TARGET) input.mini
