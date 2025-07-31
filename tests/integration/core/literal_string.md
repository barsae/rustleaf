# Program
Status: 🔴
Assertions: 0

```rustleaf
var greeting = "hello";
var empty = "";
assert(greeting == "hello");
assert(empty == "");
assert(greeting + " world" == "hello world");
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
        Token(Ident, "greeting"),
        Token(Equal),
        Token(String, "hello"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "empty"),
        Token(Equal),
        Token(String, ""),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting"),
        Token(EqualEqual),
        Token(String, "hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "empty"),
        Token(EqualEqual),
        Token(String, ""),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greeting"),
        Token(Plus),
        Token(String, " world"),
        Token(EqualEqual),
        Token(String, "hello world"),
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