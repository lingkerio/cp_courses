CC = gcc
FLEX = flex
BISON = bison

CFLAGS = -Wall -g

SRC_DIR = src
BUILD_DIR = build
TEST_DIR = test

TARGET = $(BUILD_DIR)/mini_parser

LEX_OUTPUT = $(BUILD_DIR)/lex.yy.c
BISON_OUTPUT_C = $(BUILD_DIR)/mini.tab.c
BISON_OUTPUT_H = $(BUILD_DIR)/mini.tab.h
MAIN = $(SRC_DIR)/main.c
LEX_SRC = $(SRC_DIR)/mini.l
BISON_SRC = $(SRC_DIR)/mini.y
INPUT_FILE = $(TEST_DIR)/input.mini

all: $(TARGET)

$(BISON_OUTPUT_C) $(BISON_OUTPUT_H): $(BISON_SRC)
	$(BISON) -d $(BISON_SRC) -o $(BISON_OUTPUT_C)

$(LEX_OUTPUT): $(LEX_SRC)
	$(FLEX) -o $(LEX_OUTPUT) $(LEX_SRC)

$(TARGET): $(LEX_OUTPUT) $(BISON_OUTPUT_C) $(MAIN)
	$(CC) $(CFLAGS) $(MAIN) $(LEX_OUTPUT) $(BISON_OUTPUT_C) -o $(TARGET) -lfl

clean:
	rm -f $(BUILD_DIR)/* $(LEX_OUTPUT) $(BISON_OUTPUT_C) $(BISON_OUTPUT_H)

test: $(TARGET)
	./$(TARGET) $(INPUT_FILE)

run: $(TARGET)
	./$(TARGET)