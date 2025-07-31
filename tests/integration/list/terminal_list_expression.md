# Program
Status: 🔴
Assertions: 0

```rustleaf
fn f() {
    var j = [10, 20];
    for i in j {
        i;
    }
    [1, 2, 3]
}
// This used to parse as "for_loop[1, 2, 3]", an indexing operation
assert(f() == [1, 2, 3]);
```

# Output
None

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "j"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "10"),
        Token(Comma),
        Token(Int, "20"),
        Token(RightBracket),
        Token(Semicolon),
        Token(For),
        Token(Ident, "i"),
        Token(In),
        Token(Ident, "j"),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Semicolon),
        Token(RightBrace),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected expression, found Fn",
)
```

# Eval
```rust
Skipped due to parse error
```