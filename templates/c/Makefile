# Project Name
TARGET_NAME = my_project
BUILD_DIR = build
BIN_DIR = bin
EXECUTABLE = $(BIN_DIR)/$(TARGET_NAME)

SRCS = src/main.c src/example.c
HDRS = src/include/example.h
OBJS = $(SRCS:src/%.c=$(BUILD_DIR)/%.o)

CC = gcc
CFLAGS = -Wall -Wextra -Isrc/include -O2
LDFLAGS = -lm

.PHONY: all clean run directories

all: directories $(EXECUTABLE)

directories:
	@mkdir -p $(BIN_DIR) $(BUILD_DIR)

$(BUILD_DIR)/%.o: src/%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(EXECUTABLE): $(OBJS)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $(EXECUTABLE) $(OBJS)

clean:
	rm -rf $(BUILD_DIR) $(BIN_DIR)

run: $(EXECUTABLE)
	./$(EXECUTABLE)
