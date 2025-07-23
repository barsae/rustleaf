# Program

```rustleaf
"Hello ${name}";
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: StringPart,
            text: Some(
                "Hello ",
            ),
        },
        Token {
            token_type: InterpolationStart,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
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
                            "Hello ",
                        ),
                        Expression(
                            Identifier(
                                "name",
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
    "Expression not yet implemented: InterpolatedString([Text(\"Hello \"), Expression(Identifier(\"name\"))])",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: InterpolatedString([Text(\"Hello \"), Expression(Identifier(\"name\"))])",
)
```
