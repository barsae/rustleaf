# Program

```rustleaf
"${a} and ${b} equals ${a + b}";
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: InterpolationStart,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "a",
            ),
        },
        Token {
            token_type: InterpolationEnd,
            text: None,
        },
        Token {
            token_type: StringPart,
            text: Some(
                " and ",
            ),
        },
        Token {
            token_type: InterpolationStart,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "b",
            ),
        },
        Token {
            token_type: InterpolationEnd,
            text: None,
        },
        Token {
            token_type: StringPart,
            text: Some(
                " equals ",
            ),
        },
        Token {
            token_type: InterpolationStart,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "a",
            ),
        },
        Token {
            token_type: Plus,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "b",
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
                        Expression(
                            Identifier(
                                "a",
                            ),
                        ),
                        Text(
                            " and ",
                        ),
                        Expression(
                            Identifier(
                                "b",
                            ),
                        ),
                        Text(
                            " equals ",
                        ),
                        Expression(
                            Add(
                                Identifier(
                                    "a",
                                ),
                                Identifier(
                                    "b",
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
    "Expression not yet implemented: InterpolatedString([Expression(Identifier(\"a\")), Text(\" and \"), Expression(Identifier(\"b\")), Text(\" equals \"), Expression(Add(Identifier(\"a\"), Identifier(\"b\")))])",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: InterpolatedString([Expression(Identifier(\"a\")), Text(\" and \"), Expression(Identifier(\"b\")), Text(\" equals \"), Expression(Add(Identifier(\"a\"), Identifier(\"b\")))])",
)
```
