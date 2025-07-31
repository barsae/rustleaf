# Program
Status: 🔴
Assertions: 0

```rustleaf
var positive = 42;
var negative = -positive;
var double_neg = -negative;
assert(negative == -42);
assert(double_neg == 42);
assert(-100 == -100);
assert(-(5 + 3) == -8);
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
        Token(Ident, "positive"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "negative"),
        Token(Equal),
        Token(Minus),
        Token(Ident, "positive"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "double_neg"),
        Token(Equal),
        Token(Minus),
        Token(Ident, "negative"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "negative"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "double_neg"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Minus),
        Token(Int, "100"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "100"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Minus),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Plus),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "8"),
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