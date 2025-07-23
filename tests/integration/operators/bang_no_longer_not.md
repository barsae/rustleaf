# Program 🔴
```rustleaf
// Test that ! no longer works as unary not operator
assert(!true == false);
```

# Output
None

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
        Token(Bang),
        Token(True),
        Token(EqualEqual),
        Token(False),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected statement",
)
```

# Eval
```rust
Skipped due to parse error
```