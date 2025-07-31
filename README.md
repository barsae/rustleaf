# RustLeaf 🍂

A lightweight, dynamically-typed scripting language with Rust-inspired syntax and functional programming features.

<!-- TODO: Before open sourcing
- [ ] Verify all code examples compile and run
- [ ] Update installation instructions after crates.io publish  
- [ ] Add CI badges once GitHub Actions are set up
- [ ] Update GitHub username in clone URL
-->

## Overview

RustLeaf is designed to be an embeddable scripting language that combines the elegance of Rust's syntax with the flexibility of dynamic typing. It features expression-oriented semantics, pattern matching, and a powerful pipeline operator for functional programming.

## Features

- **Rust-inspired syntax** - Familiar syntax for Rust developers
- **Dynamic typing** - Flexible type system with runtime type checking
- **Expression-oriented** - Everything is an expression that returns a value
- **Pattern matching** - Powerful `match` expressions with destructuring
- **Pipeline operator** - Compose operations with `|` for readable data transformations
- **First-class functions** - Functions and lambdas as values
- **Iterator protocol** - Built-in support for ranges and custom iterators
- **String interpolation** - Embed expressions in strings with `${...}`
- **Comprehensive standard library** - Rich set of built-in functions

## Quick Start

### Installation

<!-- TODO: Update once published to crates.io -->
```bash
# Clone the repository
git clone https://github.com/yourusername/rustleaf.git
cd rustleaf

# Build and install
cargo install --path .
```

### Hello World

```rustleaf
print("Hello, World!");
```

### Basic Examples

```rustleaf
// Variables and functions
var x = 42;
fn greet(name) {
    print("Hello, ${name}!");
}

// Pattern matching
var result = match x {
    0 => "zero",
    1..10 => "single digit",
    _ => "large number"
};

// Pipeline operations
var sum = range(1, 11)
    | filter(|n| n % 2 == 0)
    | map(|n| n * n)
    | sum();

// Classes
// TODO: Verify class syntax - may need 'var' keyword for fields
class Point {
    var x = 0;
    var y = 0;
    
    fn distance() {
        sqrt(self.x * self.x + self.y * self.y)
    }
}
```

## Language Guide

### Basic Types

- **Numbers**: `42`, `3.14`, `0xFF`, `0b1010`
- **Strings**: `"hello"`, `'world'`, `` `multiline` ``
- **Booleans**: `true`, `false`
- **Unit**: `()` (empty value)
- **Lists**: `[1, 2, 3]`
- **Dictionaries**: `{name: "Alice", age: 30}`
- **Ranges**: `1..10` (exclusive), `1..=10` (inclusive)

### Control Flow

```rustleaf
// If expressions
var max = if a > b { a } else { b };

// While loops
while condition {
    // ...
}

// For loops
for item in collection {
    print(item);
}

// Loop with break
loop {
    if done { break; }
}
```

### Functions and Lambdas

```rustleaf
// Function declaration
fn add(a, b = 0) {
    a + b
}

// Lambda expressions
var double = |x| x * 2;
var sum = |a, b| { a + b };

// Rest parameters
fn sum_all(...numbers) {
    numbers | sum()
}
```

### Pattern Matching

```rustleaf
// Match expressions
// TODO: Verify pattern syntax, especially enum patterns
match value {
    Pattern::Variant(x) => x * 2,
    [first, ...rest] => process(first, rest),
    {x, y} => Point { x, y },
    _ => default_value
}

// Destructuring  
// TODO: Verify destructuring assignment syntax
var [head, ...tail] = list;
var {x, y} = point;
```

### Error Handling

```rustleaf
// Raise errors
if invalid {
    raise "Invalid input";
}

// Assert conditions
assert(x > 0, "x must be positive");
```

## Development

### Building from Source

```bash
# Run tests
just test

# Build release version
cargo build --release

# Run with tracing
cargo run --features parser-tracing -- your_script.rustleaf
```

### Project Structure

```
rustleaf/
├── src/
│   ├── lexer/          # Tokenization
│   ├── parser/         # AST generation  
│   ├── eval/           # Interpreter
│   └── core/           # Core types and traits
├── specs/              # Language specification
├── tests/              # Integration tests
├── project-euler/      # Example solutions
├── rustleaf-macros/    # Proc macros
└── vscode-extension/   # VS Code syntax highlighting
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

RustLeaf is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in RustLeaf by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.