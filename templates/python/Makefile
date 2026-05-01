INSTALL_DIR = $(HOME)/.local/bin
SCRIPT_NAME = main.py
TARGET_NAME = my_project

.PHONY: all run clean setup install uninstall

all: setup

run:
	@if [ -d ".venv" ]; then \
		. .venv/bin/activate && python $(SCRIPT_NAME); \
	elif command -v uv > /dev/null; then \
		uv run python $(SCRIPT_NAME); \
	else \
		python3 $(SCRIPT_NAME); \
	fi

setup:
	@if command -v uv > /dev/null; then \
		uv sync; \
	else \
		python3 -m venv .venv; \
		. .venv/bin/activate && pip install -r requirements.txt; \
	fi

install:
	@echo "Installing $(TARGET_NAME) to $(INSTALL_DIR) ..."
	@mkdir -p $(INSTALL_DIR)
	@echo '#!/bin/bash' > $(INSTALL_DIR)/$(TARGET_NAME)
	@echo 'DIR=$$(cd $$(dirname $$(readlink -f "$$0")) && pwd)' >> $(INSTALL_DIR)/$(TARGET_NAME)
	@echo 'cd $(CURDIR) && ./Makefile run' >> $(INSTALL_DIR)/$(TARGET_NAME) # This is a bit tricky for templates
	@# For a template, it's better to just install the script if it's standalone or use a wrapper
	@cp $(SCRIPT_NAME) $(INSTALL_DIR)/$(TARGET_NAME)
	@chmod +x $(INSTALL_DIR)/$(TARGET_NAME)
	@echo "Installed $(TARGET_NAME) to $(INSTALL_DIR)"

uninstall:
	@echo "Uninstalling $(TARGET_NAME) from $(INSTALL_DIR) ..."
	@rm -f $(INSTALL_DIR)/$(TARGET_NAME)
	@echo "Uninstalled $(TARGET_NAME)"

clean:
	rm -rf .venv __pycache__ .uv
