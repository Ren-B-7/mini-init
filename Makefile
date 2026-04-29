BINARY_NAME = mini
INSTALL_DIR = $(HOME)/.local/bin

.PHONY: all build install clean uninstall test

all: build

build:
	cargo build --release

install: build
	mkdir -p $(INSTALL_DIR)
	cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Installed $(BINARY_NAME) to $(INSTALL_DIR)"

uninstall:
	rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Uninstalled $(BINARY_NAME)"

clean:
	cargo clean

test:
	cargo test

.PHONY: lint lint-interactive run format

lint:
	cargo clippy

lint-interactive:
	bacon

run:
	cargo run

format:
	cargo fmt
