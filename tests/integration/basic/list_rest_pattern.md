# Program

```rustleaf
var [first, *rest] = [1, 2, 3, 4];
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
                "first",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Star,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "rest",
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
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "4",
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
                pattern: ListRest(
                    [
                        Variable(
                            "first",
                        ),
                    ],
                    Some(
                        "rest",
                    ),
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
                            Literal(
                                Int(
                                    4,
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
