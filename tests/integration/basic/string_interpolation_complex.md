# Program

```rustleaf
"Value: ${x + y * 2}";
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: StringPart,
            text: Some(
                "Value: ",
            ),
        },
        Token {
            token_type: InterpolationStart,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Plus,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "y",
            ),
        },
        Token {
            token_type: Star,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "2",
            ),
        },
        Token {
            token_type: InterpolationEnd,
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
                InterpolatedString(
                    [
                        Text(
                            "Value: ",
                        ),
                        Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Mul(
                                    Identifier(
                                        "y",
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ),
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
Err(
    "Expression not yet implemented: InterpolatedString([Text(\"Value: \"), Expression(Add(Identifier(\"x\"), Mul(Identifier(\"y\"), Literal(Int(2)))))])",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: InterpolatedString([Text(\"Value: \"), Expression(Add(Identifier(\"x\"), Mul(Identifier(\"y\"), Literal(Int(2)))))])",
)
```
