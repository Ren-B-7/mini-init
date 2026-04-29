# Mini CLI Tool

`mini` is a CLI tool designed to initialize and manage projects based on various templates.

## Features

- **Portability**: All templates are embedded within the `mini` binary.
- **Multi-language Support**:
  - **C**: Standard C template.
  - **C-Strict**: C template with comprehensive compiler and linker flags for hardening and static analysis.
  - **Python**: Python template. Automatically runs `uv init` if `uv` is installed.
  - **Rust**: Rust template. Automatically runs `cargo init`.
- **Commands**:
  - `init <project_name> [--lang <lang>]`: Initializes a new project. Default language is `c`.
    - Supported languages: `c`, `c-strict`, `python`, `rust`.
  - `make`: Runs `make` in the current directory.
  - `clean`: Runs `make clean` in the current directory.
  - `remove <project_name>`: Deletes the specified project directory.

## Installation

### Prerequisites

- Rust and Cargo
- GCC and Make (for C templates)
- `uv` (optional, for enhanced Python support)

### Steps

1. Clone this repository.
2. Run `make install`. This will build the tool in release mode and copy the binary to `~/.local/bin/mini`.
   ```bash
   make install
   ```
3. Ensure `~/.local/bin` is in your `PATH`.

## Usage

### Initialize a C project
```bash
mini init my_c_project
```

### Initialize a Strict C project
```bash
mini init my_strict_project --lang c-strict
```

### Initialize a Python project
```bash
mini init my_python_project --lang python
```

### Build and Run
Each template includes a `Makefile` to standardize common operations:
```bash
mini make      # Build/Install dependencies
mini make run  # Execute the project
mini clean     # Cleanup build artifacts
```

## License

This tool and its templates are licensed under the MIT License.
