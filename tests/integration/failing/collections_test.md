# Program 🔴
```rustleaf
// #[fail_quietly]
// List creation test
let my_list = [1, 2, 3];
assert(true);  // Just test that it doesn't crash

// Dict creation test  
let my_dict = {"a": 1, "b": 2};
assert(true);  // Just test that it doesn't crash

// Mixed key types in dict
let mixed_dict = {1: "one", "two": 2, true: "yes"};
assert(true);  // Just test that it doesn't crash
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
        Token(Ident, "let"),
        Token(Ident, "my_list"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "let"),
        Token(Ident, "my_dict"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "a"),
        Token(Colon),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "b"),
        Token(Colon),
        Token(Int, "2"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "let"),
        Token(Ident, "mixed_dict"),
        Token(Equal),
        Token(LeftBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(String, "one"),
        Token(Comma),
        Token(String, "two"),
        Token(Colon),
        Token(Int, "2"),
        Token(Comma),
        Token(True),
        Token(Colon),
        Token(String, "yes"),
        Token(RightBrace),
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