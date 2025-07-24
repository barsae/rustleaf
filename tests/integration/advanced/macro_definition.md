# Program
Status: 🟢
Assertions: 0

```rustleaf
#[macro]
fn test_macro(ast_node) {
    ast_node
}
```

# Output
None

# Result
```rust
Ok(
    Unit,
)
```

# Lex
```rust
Ok(
    [
        Token(Hash),
        Token(LeftBracket),
        Token(Macro),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "test_macro"),
        Token(LeftParen),
        Token(Ident, "ast_node"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "ast_node"),
        Token(RightBrace),
        Token(Eof),
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [],
        None,
    ),
)
```