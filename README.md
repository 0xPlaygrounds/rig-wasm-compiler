# Rig WASM Compiler

<p align="center">
  <img src="path_to_logo.png" alt="Rig WASM Compiler Logo" width="200"/>
</p>

<p align="center">
  <a href="https://github.com/yourusername/rig-wasm-compiler/actions"><img alt="Build Status" src="https://github.com/yourusername/rig-wasm-compiler/workflows/CI/badge.svg"></a>
  <a href="https://crates.io/crates/rig-wasm-compiler"><img alt="Crate Info" src="https://img.shields.io/crates/v/rig-wasm-compiler.svg"></a>
  <a href="https://docs.rs/rig-wasm-compiler"><img alt="API Docs" src="https://docs.rs/rig-wasm-compiler/badge.svg"></a>
  <a href="https://github.com/yourusername/rig-wasm-compiler/blob/main/LICENSE"><img alt="License Info" src="https://img.shields.io/github/license/yourusername/rig-wasm-compiler"></a>
</p>

Rig WASM Compiler is a powerful tool that enables the compilation of Rig applications to WebAssembly (WASM), allowing Rig-based LLM applications to run seamlessly in web browsers. This project bridges the gap between Rig's robust AI capabilities and the web platform, opening up new possibilities for AI-powered web applications.

## Features

- Compile Rig applications to WASM
- Run Rig Agents and RAG Agents in web browsers
- WASM-compatible adapters for HTTP clients and vector stores
- Easy integration with existing web projects
- Optimized for performance in browser environments

## Quick Start

### Project structure:
```
rig-wasm-compiler/
├── Cargo.toml
├── .gitignore
├── README.md
├── build.rs
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── wasm_bindings/
│   │   ├── mod.rs
│   │   ├── agent.rs
│   │   └── rag.rs
│   ├── adapters/
│   │   ├── mod.rs
│   │   ├── http_client.rs
│   │   └── vector_store.rs
│   └── utils/
│       ├── mod.rs
│       └── wasm_utils.rs
├── examples/
│   ├── simple_agent.rs
│   └── rag_agent.rs
├── tests/
│   ├── integration_tests.rs
│   └── wasm_tests.rs
└── www/
    ├── index.html
    ├── index.js
    └── webpack.config.js

```

### Installation

To install the Rig WASM Compiler, ensure you have Rust and Cargo installed, then run:

```bash
cargo install rig-wasm-compiler
```

### Usage

1. Create a Rig application (e.g., `my_rig_app.rs`).
2. Compile it to WASM:

```bash
rig-wasm-compiler my_rig_app.rs --output dist
```

3. Include the generated WASM module in your web project:

```html
<script type="module">
  import init, { WasmAgent } from './dist/my_rig_app.js';

  async function run() {
    await init();
    const agent = new WasmAgent();
    const result = await agent.process("Hello, Rig!");
    console.log(result);
  }

  run();
</script>
```

## Documentation

For detailed documentation, including API reference and advanced usage, visit our [documentation page](https://docs.rs/rig-wasm-compiler).

## Examples

Check out the `examples/` directory for sample Rig applications compiled to WASM:

- `simple_agent.rs`: A basic Rig agent running in the browser
- `rag_agent.rs`: A RAG (Retrieval-Augmented Generation) agent with WASM-compatible vector store

## Development

To set up the development environment:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rig-wasm-compiler.git
   cd rig-wasm-compiler
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for more details on how to get started.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Rig team for their excellent AI framework
- The Rust and WebAssembly working group for their tools and documentation

## Contact

For questions, suggestions, or discussions, please open an issue on GitHub or contact us at [your-email@example.com](mailto:your-email@example.com).

---

<p align="center">Made with ❤️ by the Rig community</p>
