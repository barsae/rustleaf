# Program 🔴
```rustleaf
// #[fail_quietly]
// Test expressions in collections
let list_with_expr = [1 + 2, 3 * 4];
assert(true);  // Just test that it doesn't crash

// Test expressions in dict
let dict_with_expr = {"sum": 1 + 2, "product": 3 * 4};
assert(true);  // Just test that it doesn't crash

// Test variable in collections
let x = 5;
let list_with_var = [x, x + 1];
assert(true);  // Just test that it doesn't crash
```

# Output
```
None
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Ident, "let"),
        Token(Ident, "list_with_expr"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Star),
        Token(Int, "4"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "let"),
        Token(Ident, "dict_with_expr"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "sum"),
        Token(Colon),
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
        Token(Comma),
        Token(String, "product"),
        Token(Colon),
        Token(Int, "3"),
        Token(Star),
        Token(Int, "4"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "let"),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Ident, "let"),
        Token(Ident, "list_with_var"),
        Token(Equal),
        Token(LeftBracket),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(True),
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