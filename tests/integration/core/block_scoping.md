# Program
Status: 🔴
Assertions: 0

```rustleaf
var x = 1;
var inner_x;
var outer_x;
{
    var x = 2;
    inner_x = x;
}
outer_x = x;
assert(inner_x == 2);
assert(outer_x == 1);
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
        Token(Int, "1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "inner_x"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "outer_x"),
        Token(Semicolon),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Ident, "inner_x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "outer_x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "inner_x"),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "outer_x"),
        Token(EqualEqual),
        Token(Int, "1"),
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