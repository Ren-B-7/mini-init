.PHONY: all run clean install

all: install

run:
	@if [ -d ".venv" ]; then \
		. .venv/bin/activate && python main.py; \
	elif command -v uv > /dev/null; then \
		uv run python main.py; \
	else \
		python3 main.py; \
	fi

install:
	@if command -v uv > /dev/null; then \
		uv sync; \
	else \
		python3 -m venv .venv; \
		. .venv/bin/activate && pip install -r requirements.txt; \
	fi

clean:
	rm -rf .venv __pycache__ .uv
