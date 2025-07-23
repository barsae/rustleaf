# Program 🔴

```rustleaf
fn test_return() {
    return 42;
}

let result = test_return();
assert(result == 42);
```

# Output

```
Skipped due to parse error
```

# Result

```rust
Skipped due to parse error
```

# Lex

```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Return),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "let"),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "test_return"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
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
