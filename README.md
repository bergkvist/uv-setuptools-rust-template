# uv-setuptools-rust-template

This is a template for using custom PyO3 extensions from Python using the uv package manager.

## Requirements
Make sure you have [rustup](https://rustup.rs/) and [uv](https://docs.astral.sh/uv/getting-started/installation/) installed:
```sh
# Install rustup for compiling Rust code
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install uv package manager for Python
curl --proto '=https' --tlsv1.2 -LsSf https://astral.sh/uv/install.sh | sh
```

## Usage
Automatically builds the Rust extension, and runs some Python code that uses it:
```sh
uv run main
```
