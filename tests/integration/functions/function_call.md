# Program
Status: 🔴
Assertions: 0

```rustleaf
fn add(x, y) { x + y }
assert(add(2, 3) == 5);
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
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "5"),
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