# Program

```rustleaf
print("hello, world");
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "print",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "hello, world",
            ),
        },
        Token {
            token_type: RightParen,
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
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Literal(
                            String(
                                "hello, world",
                            ),
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
            Call(
                Variable(
                    "print",
                ),
                [
                    Literal(
                        String(
                            "hello, world",
                        ),
                    ),
                ],
            ),
        ],
        None,
    ),
)
```

# Output

```
String("hello, world")
```

# Result

```rust
Ok(
    Unit,
)
```
