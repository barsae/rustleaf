# Program
Status: 🔴
Assertions: 0

```rustleaf
fn test_return() {
    return 42;
}

var result = test_return();
assert(result == 42);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Return),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Fn",
)
```

# Eval
```rust
Skipped due to parse error
```