# Wasp Language Extension for Zed

This is a language extension for [Zed](https://zed.dev) that provides support for the [Wasp](https://wasp-lang.dev) language.

## Features

- **Syntax highlighting** for `.wasp` files using Tree-sitter grammar
- **Language server integration** with the Wasp language server (`waspls`)
- **Code snippets** for common Wasp declarations (page, route, query, action, etc.)
- **Autocompletion** and error reporting via the language server
- **Code outline** and structure navigation

## Requirements

- [Zed editor](https://zed.dev)
- [Wasp](https://wasp-lang.dev) version 0.14.0 or later installed and available in your PATH

If you don't have Wasp installed, follow the [installation instructions](https://wasp-lang.dev/docs).

## Installation

### As a Dev Extension (for development/testing)

1. Clone this repository:

   ```bash
   git clone https://github.com/dmackinn/zed-wasp.git
   cd zed-wasp
   ```

2. Install Rust if you haven't already:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Open Zed and install the extension as a dev extension:
   - Open the command palette
   - Run "zed: install dev extension"
   - Select the directory containing this extension

### From the Zed Extension Registry (coming soon)

Once published to the Zed extension registry, you'll be able to install it directly from Zed's extensions panel.

## Usage

Once installed, the extension will automatically activate when you open `.wasp` files. You'll get:

- Syntax highlighting
- Language server features
- Code snippets (type snippet prefixes like `page`, `route`, `query`, etc.)

## Available Snippets

- `page` - Create a new page and route
- `route` - Create a new route
- `query` - Create a new query
- `action` - Create a new action
- `job` - Create a new job
- `api` - Create a new API route
- `apiNamespace` - Create a new API namespace
- `crud` - Create a new CRUD operations
- `app` - Create a new app declaration

## Development

This extension is based on the [VS Code Wasp extension](https://github.com/wasp-lang/vscode-wasp) and follows the [Zed extension development guidelines](https://zed.dev/docs/extensions/developing-extensions).

### Building

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Testing

Install as a dev extension in Zed and test with Wasp files.

## License

MIT License - see [LICENSE](LICENSE) file for details.
