BINARY_NAME = my_project
INSTALL_DIR = $(HOME)/.local/bin

.PHONY: all build run clean test install uninstall

all: build

build:
	cargo build --release

run:
	cargo run

test:
	cargo test

install: build
	mkdir -p $(INSTALL_DIR)
	install -m 755 target/release/$(BINARY_NAME) $(INSTALL_DIR)
	@echo "Installed $(BINARY_NAME) to $(INSTALL_DIR)"

uninstall:
	rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Uninstalled $(BINARY_NAME)"

clean:
	cargo clean
