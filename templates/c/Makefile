# Project Name
TARGET_NAME = my_project
BUILD_DIR = build
BIN_DIR = bin
EXECUTABLE = $(BIN_DIR)/$(TARGET_NAME)
INSTALL_DIR = $(HOME)/.local/bin

SRCS = src/main.c src/example.c
HDRS = src/include/example.h
OBJS = $(SRCS:src/%.c=$(BUILD_DIR)/%.o)

CC = gcc
CFLAGS = -Wall -Wextra -Isrc/include -O3
LDFLAGS = -lm

.PHONY: all clean run directories install uninstall

all: directories $(EXECUTABLE)

directories:
	@mkdir -p $(BIN_DIR) $(BUILD_DIR)

$(BUILD_DIR)/%.o: src/%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(EXECUTABLE): $(OBJS)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $(EXECUTABLE) $(OBJS)

install: all
	mkdir -p $(INSTALL_DIR)
	install -m 755 $(EXECUTABLE) $(INSTALL_DIR)
	@echo "Installed $(TARGET_NAME) to $(INSTALL_DIR)"

uninstall:
	rm -f $(INSTALL_DIR)/$(TARGET_NAME)
	@echo "Uninstalled $(TARGET_NAME)"

clean:
	rm -rf $(BUILD_DIR) $(BIN_DIR)

run: $(EXECUTABLE)
	./$(EXECUTABLE)
