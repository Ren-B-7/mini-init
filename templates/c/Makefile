# Project Name (edit this)
TARGET_NAME = project
BUILD_DIR = build
BIN_DIR = bin
EXECUTABLE = $(BIN_DIR)/$(TARGET_NAME)

# Source files (edit these) - MUST be in src/ directory
SRCS = $(wildcard src/*.c)
# Header files (edit these)
HDRS = $(wildcard src/*.h)

# Object files (placed in build/ directory)
OBJS = $(SRCS:src/%.c=$(BUILD_DIR)/%.o)

# Compiler
CC = gcc

# Standard compilation flags
CFLAGS = -std=c99 -Wall -Wextra -Isrc

# Optimization
OPTFLAGS = -O2

# Linker flags
LDFLAGS = 

# Combine all flags
ALL_CFLAGS = $(CFLAGS) $(OPTFLAGS)

# Targets
.PHONY: all clean run format lint directories

all: directories $(EXECUTABLE)

# Create output directories if they don't exist
directories:
	@mkdir -p $(BIN_DIR) $(BUILD_DIR)

# Rule to compile .c files into .o files in the build/ directory
$(BUILD_DIR)/%.o: src/%.c
	@echo "Compiling $< ..."
	$(CC) $(ALL_CFLAGS) -c $< -o $@

# Rule to link the executable in the bin/ directory
$(EXECUTABLE): $(OBJS)
	@echo "Linking $@ ..."
	$(CC) $(ALL_CFLAGS) $(LDFLAGS) -o $(EXECUTABLE) $(OBJS) -lm

# Rule to clean up build artifacts
clean:
	@echo "Cleaning up build artifacts..."
	@rm -rf $(BUILD_DIR) $(BIN_DIR)

# Rule to run the executable
run: $(EXECUTABLE)
	@echo "Running $(EXECUTABLE) ..."
	./$(EXECUTABLE)

# Format code using clang-format
format:
	@echo "Formatting code..."
	@clang-format -style=file:./.clang-format -i $(SRCS) $(HDRS)
	mbake format --config ./.bake.toml Makefile

# Run static analysis with clang-tidy
CLANG_TIDY_CHECKS = -checks=-*,readability-*,performance-*
CLANG_TIDY_FLAGS = -std=c99 -Wall -Wextra -Isrc

lint:
	@echo "Running static analysis..."
	@clang-tidy $(CLANG_TIDY_CHECKS) $(SRCS) -- $(CLANG_TIDY_FLAGS)
	mbake validate --config ./.bake.toml Makefile
