# Program
Status: 🔴
Assertions: 0

```rustleaf
var y;
var z;
y = 100;
z = "assigned later";
assert(y == 100);
assert(z == "assigned later");
assert(y + 23 == 123);
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
        Token(Ident, "y"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "z"),
        Token(Semicolon),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "100"),
        Token(Semicolon),
        Token(Ident, "z"),
        Token(Equal),
        Token(String, "assigned later"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(EqualEqual),
        Token(Int, "100"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
        Token(EqualEqual),
        Token(String, "assigned later"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(Plus),
        Token(Int, "23"),
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