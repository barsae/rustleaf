# Program

```rustleaf
super.method();
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Super,
            text: None,
        },
        Token {
            token_type: Dot,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "method",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
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
                MethodCall(
                    Super,
                    "method",
                    [],
                ),
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: Super",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Super",
)
```
