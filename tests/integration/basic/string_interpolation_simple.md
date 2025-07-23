# Program

```rustleaf
"Hello ${name}";
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

# Lex

```rust
Ok(
    [
        Token(StringPart, "Hello "),
        Token(InterpolationStart),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(Semicolon),
        Token(Eof),
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
