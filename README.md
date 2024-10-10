# Rig WASM Compiler

A tool for easily compiling Rig projects to WebAssembly.

## Installation

```bash
cargo install --git https://github.com/yourusername/rig-wasm-compiler.git
```

## Usage

In your Rig project directory:

```bash
rig-wasm-compiler --project-path . --output-path ./wasm-output
```

## Configuration

You can create a `rig-wasm.toml` file in your project root to configure the compiler:

```toml
wasm_pack_args = ["--target", "web"]
output_path = "./wasm-output"
```

Then run the compiler with:

```bash
rig-wasm-compiler --project-path . --config rig-wasm.toml
```

## Options

- `--project-path`: Path to the Rig project (required)
- `--output-path`: Path for the WebAssembly output
- `--release`: Compile in release mode
- `--config`: Path to a configuration file

## License

This project is licensed under the MIT License.