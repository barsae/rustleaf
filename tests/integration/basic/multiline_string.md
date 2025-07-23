# Program

```rustleaf
"""This is a
multiline string
with multiple lines""";
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: MultilineString,
            text: Some(
                "This is a\nmultiline string\nwith multiple lines",
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
                Literal(
                    String(
                        "This is a\nmultiline string\nwith multiple lines",
                    ),
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
            Literal(
                String(
                    "This is a\nmultiline string\nwith multiple lines",
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
Ok(
    Unit,
)
```
