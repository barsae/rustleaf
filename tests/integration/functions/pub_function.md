# Program
Status: 🔴
Assertions: 0

```rustleaf
pub fn greet(name) {
    return "Hello, " + name;
}

var result = greet("World");
assert(result == "Hello, World");
assert(greet("Alice") == "Hello, Alice");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Pub)
parse_statement: starting at position 0 (Pub)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Pub),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Return),
        Token(String, "Hello, "),
        Token(Plus),
        Token(Ident, "name"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(String, "World"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "Hello, World"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(String, "Alice"),
        Token(RightParen),
        Token(EqualEqual),
        Token(String, "Hello, Alice"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Pub",
)
```

# Eval
```rust
Skipped due to parse error
```