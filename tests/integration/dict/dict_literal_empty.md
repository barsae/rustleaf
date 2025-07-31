# Program
Status: 🔴
Assertions: 0

```rustleaf
var x = {};
assert(x is Dict);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(LeftBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Is),
        Token(Ident, "Dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Var",
)
```

# Eval
```rust
Skipped due to parse error
```