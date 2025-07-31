# Program
Status: 🔴
Assertions: 0

```rustleaf
var i;
try {
    raise(42);
} catch e {
    i = e;
}
assert(i == 42);
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
        Token(Ident, "i"),
        Token(Semicolon),
        Token(Try),
        Token(LeftBrace),
        Token(Ident, "raise"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(Catch),
        Token(Ident, "e"),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Equal),
        Token(Ident, "e"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "i"),
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
    "Expected Hash, found Var",
)
```

# Eval
```rust
Skipped due to parse error
```