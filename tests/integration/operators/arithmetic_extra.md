# Program
Status: 🔴
Assertions: 0

```rustleaf
// Test modulo operator
assert(7 % 3 == 1);
assert(8 % 4 == 0);
assert(7.5 % 2.5 == 0.0);
assert(7 % 2.0 == 1.0);

// Test power operator
assert(2 ** 3 == 8);
assert(2 ** 0 == 1);
assert(3.0 ** 2.0 == 9.0);
assert(2 ** 3.0 == 8.0);
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
        Token(Int, "7"),
        Token(Percent),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "8"),
        Token(Percent),
        Token(Int, "4"),
        Token(EqualEqual),
        Token(Int, "0"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Float, "7.5"),
        Token(Percent),
        Token(Float, "2.5"),
        Token(EqualEqual),
        Token(Float, "0.0"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "7"),
        Token(Percent),
        Token(Float, "2.0"),
        Token(EqualEqual),
        Token(Float, "1.0"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(StarStar),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "8"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(StarStar),
        Token(Int, "0"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Float, "3.0"),
        Token(StarStar),
        Token(Float, "2.0"),
        Token(EqualEqual),
        Token(Float, "9.0"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(StarStar),
        Token(Float, "3.0"),
        Token(EqualEqual),
        Token(Float, "8.0"),
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