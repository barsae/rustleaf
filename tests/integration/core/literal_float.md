# Program
Status: 🔴
Assertions: 0

```rustleaf
var pi = 3.14;
var small = 0.1;
var negative = -2.5;
assert(pi == 3.14);
assert(small == 0.1);
assert(negative == -2.5);
assert(pi + small == 3.24);
assert(pi * 2.0 == 6.28);
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
        Token(Ident, "pi"),
        Token(Equal),
        Token(Float, "3.14"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "small"),
        Token(Equal),
        Token(Float, "0.1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "negative"),
        Token(Equal),
        Token(Minus),
        Token(Float, "2.5"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "pi"),
        Token(EqualEqual),
        Token(Float, "3.14"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "small"),
        Token(EqualEqual),
        Token(Float, "0.1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "negative"),
        Token(EqualEqual),
        Token(Minus),
        Token(Float, "2.5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "pi"),
        Token(Plus),
        Token(Ident, "small"),
        Token(EqualEqual),
        Token(Float, "3.24"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "pi"),
        Token(Star),
        Token(Float, "2.0"),
        Token(EqualEqual),
        Token(Float, "6.28"),
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