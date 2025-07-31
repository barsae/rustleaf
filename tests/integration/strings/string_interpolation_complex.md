# Program
Status: 🔴
Assertions: 0

```rustleaf
var x = 10;
var y = 5;
var result = "Value: ${x + y * 2}";
assert(result == "Value: 20");
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(StringPart, "Value: "),
        Token(InterpolationStart),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(Star),
        Token(Int, "2"),
        Token(InterpolationEnd),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "Value: 20"),
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