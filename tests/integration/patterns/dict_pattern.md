# Program
Status: 🔴
Assertions: 0

```rustleaf
var user = {"name": "Alice", "age": 30};
var {name, age: user_age} = user;
assert(name == "Alice");
assert(user_age == 30);
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
        Token(Ident, "user"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "name"),
        Token(Colon),
        Token(String, "Alice"),
        Token(Comma),
        Token(String, "age"),
        Token(Colon),
        Token(Int, "30"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(LeftBrace),
        Token(Ident, "name"),
        Token(Comma),
        Token(Ident, "age"),
        Token(Colon),
        Token(Ident, "user_age"),
        Token(RightBrace),
        Token(Equal),
        Token(Ident, "user"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(EqualEqual),
        Token(String, "Alice"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "user_age"),
        Token(EqualEqual),
        Token(Int, "30"),
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