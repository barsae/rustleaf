# Program

```rustleaf
var [a, b, c] = [1, 2, 3];
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Var,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "a",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "b",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "c",
            ),
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "1",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "2",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "3",
            ),
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
    ],
)
```

# Parse

```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: List(
                    [
                        Variable(
                            "a",
                        ),
                        Variable(
                            "b",
                        ),
                        Variable(
                            "c",
                        ),
                    ],
                ),
                value: Some(
                    List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                ),
            },
        ],
    ),
)
```

# Eval

```rust
Err(
    "Complex patterns not yet implemented",
)
```

# Output

```

```

# Result

```rust
Err(
    "Complex patterns not yet implemented",
)
```
