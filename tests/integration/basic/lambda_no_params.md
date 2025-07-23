# Program

```rustleaf
|| 42;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Pipe,
            text: None,
        },
        Token {
            token_type: Pipe,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "42",
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
            Expression(
                Lambda {
                    params: [],
                    body: Expression(
                        Literal(
                            Int(
                                42,
                            ),
                        ),
                    ),
                },
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: Lambda { params: [], body: Expression(Literal(Int(42))) }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Lambda { params: [], body: Expression(Literal(Int(42))) }",
)
```
