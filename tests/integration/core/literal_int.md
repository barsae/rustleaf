# Program
Status: 🔴
Assertions: 0

```rustleaf
var num = 123;
var zero = 0;
var negative = -42;
assert(num == 123);
assert(zero == 0);
assert(negative == -42);
assert(num + zero == 123);
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
        Token(Ident, "num"),
        Token(Equal),
        Token(Int, "123"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "zero"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "negative"),
        Token(Equal),
        Token(Minus),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "num"),
        Token(EqualEqual),
        Token(Int, "123"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "zero"),
        Token(EqualEqual),
        Token(Int, "0"),
        Token(RightParen),
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
        Token(Ident, "num"),
        Token(Plus),
        Token(Ident, "zero"),
        Token(EqualEqual),
        Token(Int, "123"),
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