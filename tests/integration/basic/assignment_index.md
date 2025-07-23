# Program

```rustleaf
arr[0] = 99;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "arr",
            ),
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "0",
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
            token_type: Int,
            text: Some(
                "99",
            ),
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
            Assignment {
                target: GetItem(
                    Identifier(
                        "arr",
                    ),
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
                op: Assign,
                value: Literal(
                    Int(
                        99,
                    ),
                ),
            },
        ],
    ),
)
```

# Eval

```rust
Ok(
    Block(
        [
            SetItem(
                Variable(
                    "arr",
                ),
                Literal(
                    Int(
                        0,
                    ),
                ),
                Literal(
                    Int(
                        99,
                    ),
                ),
            ),
        ],
        None,
    ),
)
```

# Output

```

```

# Result

```rust
Err(
    "eval not implemented for: SetItem(Variable(\"arr\"), Literal(Int(0)), Literal(Int(99)))",
)
```
