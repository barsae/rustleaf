# Program
Status: 🔴
Assertions: 0

```rustleaf
fn deep_recursion(n) {
    if n <= 0 {
        return 0
    } else {
        return 1 + deep_recursion(n - 1)
    }
}

print(deep_recursion(5000))
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
        Token(Ident, "deep_recursion"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(RightParen),
        Token(LeftBrace),
        Token(If),
        Token(Ident, "n"),
        Token(LessEqual),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(Return),
        Token(Int, "0"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Return),
        Token(Int, "1"),
        Token(Plus),
        Token(Ident, "deep_recursion"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(Minus),
        Token(Int, "1"),
        Token(RightParen),
        Token(RightBrace),
        Token(RightBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "deep_recursion"),
        Token(LeftParen),
        Token(Int, "5000"),
        Token(RightParen),
        Token(RightParen),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected statement",
)
```

# Eval
```rust
Skipped due to parse error
```