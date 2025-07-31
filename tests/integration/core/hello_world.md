# Program
Status: 🔴
Assertions: 0

```rustleaf
assert("hello, world" == "hello, world");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Ident(assert))
parse_statement: starting at position 0 (Ident(assert))
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello, world"),
        Token(EqualEqual),
        Token(String, "hello, world"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Ident",
)
```

# Eval
```rust
Skipped due to parse error
```