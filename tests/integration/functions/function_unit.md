# Program
Status: 🔴
Assertions: 0

```rustleaf
var z = 0;
fn add(x, y) {
    z += 1;
    x + y;
}
assert(is_unit(add(2, 3)));
assert(z == 1);
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
        Token(Ident, "z"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "z"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
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