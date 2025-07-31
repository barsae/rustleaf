# Program
Status: 🔴
Assertions: 0

```rustleaf
assert(1 + 2 == 3);
assert(5 - 3 == 2);
assert(4 * 3 == 12);
assert(10 / 2 == 5.0);
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
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Minus),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Star),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "12"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Slash),
        Token(Int, "2"),
        Token(EqualEqual),
        Token(Float, "5.0"),
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