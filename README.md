# Thrust-LSP: A High-Performance Language Server for JavaScript and TypeScript

<!--![Build Status](https://img.shields.io/github/workflow/status/yourusername/yourrepository/CI)-->
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Issues](https://img.shields.io/github/issues/paul0lden/thrust-lsp)
![Forks](https://img.shields.io/github/forks/paul0lden/thrust-lsp)
![Stars](https://img.shields.io/github/stars/paul0lden/thrust-lsp)


<p align="center">
  <img src="https://github.com/paul0lden/thrust-lsp/assets/55639625/7803c1de-4dc4-4b5f-9de6-e19bdffb7a01" alt="Project Logo" width="200"/>
</p>


## Overview
Thrust-LSP is an open-source language server protocol (LSP) implementation designed to provide lightning-fast performance and extensive feature support for JavaScript, TypeScript, JSX, and TSX. Built with Rust, Thrust-LSP aims to deliver a seamless development experience with minimal latency and robust integrations.

## Key Features
- **Speed**: Leveraging the performance advantages of Rust, Thrust-LSP ensures rapid response times and efficient memory usage, making it one of the fastest LSP servers available.
- **Comprehensive Language Support**: Full support for JavaScript, TypeScript, JSX, and TSX, including advanced features such as intelligent code completion, real-time error checking, and refactoring tools.
- **Editor Integrations**: Out-of-the-box integrations with popular editors such as Vim/Neovim and Visual Studio Code, providing a smooth and consistent development experience across different platforms.
- **Scalability**: Designed to handle projects of any size, from small scripts to large codebases, without compromising on performance.
- **Extensibility**: Modular architecture allows for easy extension and customization, enabling developers to tailor the server to their specific needs.

## Installation

### Prerequisites
- Rust (latest stable version)
- Node.js (for editor integration)

### Building from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/Thrust-LSP.git
   cd Thrust-LSP
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the LSP server:
   ```bash
   cargo install --path .
   ```

### Editor Integration

#### Neovim
1. Install the LSP client plugin (e.g., `nvim-lspconfig`):
   ```bash
   :Plug 'neovim/nvim-lspconfig'
   ```

2. Configure Thrust-LSP in your Neovim configuration file (`init.vim` or `init.lua`):
   ```lua
   require'lspconfig'.thrustlsp.setup{}
   ```

#### Visual Studio Code
1. Install the Thrust-LSP extension from the Visual Studio Code marketplace.

2. Configure Thrust-LSP in your `settings.json`:
   ```json
   {
     "thrustlsp.serverPath": "/path/to/thrustlsp"
   }
   ```

## Contributing
Contributions are welcome! Please read our [contributing guide](CONTRIBUTING.md) to get started. Whether it's reporting a bug, suggesting a feature, or submitting a pull request, your input helps make Thrust-LSP better.

## License
Thrust-LSP is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact
For questions, feedback, or support, please open an issue on GitHub.
