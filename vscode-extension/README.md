# RustLeaf VS Code Extension

Basic syntax highlighting for the RustLeaf programming language.

## Features

- Syntax highlighting for RustLeaf files (`.rustleaf`)
- Support for:
  - Keywords (`var`, `fn`, `class`, `if`, `while`, etc.)
  - Comments (single-line `//`, multi-line `/* */`, documentation `///` and `/** */`)
  - String literals with interpolation support (`"Hello ${name}"`)
  - Raw string literals (`r"..."`)
  - Numeric literals (decimal, hex, octal, binary, floats)
  - Operators and punctuation

## Installation

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Click "Install from VSIX..."
4. Select the `.vsix` file

## Development

To package the extension:

```bash
cd vscode-extension
npm install -g vsce
vsce package
```

This will create a `.vsix` file that can be installed in VS Code.