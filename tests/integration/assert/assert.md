# Program
Status: 🔴
Assertions: 0

```rustleaf
assert(true);
assert(1 == 1);
assert(10 + 5 == 15, "Math should work");
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
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Plus),
        Token(Int, "5"),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(Comma),
        Token(String, "Math should work"),
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