# Program

```rustleaf
1..10;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Int,
            text: Some(
                "1",
            ),
        },
        Token {
            token_type: DotDot,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "10",
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
                RangeExclusive(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                    Literal(
                        Int(
                            10,
                        ),
                    ),
                ),
            ),
        ],
    ),
)
```

# Eval

```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(10)))",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: RangeExclusive(Literal(Int(1)), Literal(Int(10)))",
)
```
