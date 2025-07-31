# Program
Status: 🔴
Assertions: 0

```rustleaf
assert(sqrt(4) == 2.0);
assert(abs(-5) == 5);
assert(floor(3.7) == 3);
assert(ceil(3.2) == 4);
assert(round(3.5) == 4);
assert(min(5, 3) == 3);
assert(max(5, 3) == 5);
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
        Token(Ident, "sqrt"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Float, "2.0"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "abs"),
        Token(LeftParen),
        Token(Minus),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "floor"),
        Token(LeftParen),
        Token(Float, "3.7"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "ceil"),
        Token(LeftParen),
        Token(Float, "3.2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "round"),
        Token(LeftParen),
        Token(Float, "3.5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "4"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "min"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "max"),
        Token(LeftParen),
        Token(Int, "5"),
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
    "Expected Hash, found Ident",
)
```

# Eval
```rust
Skipped due to parse error
```