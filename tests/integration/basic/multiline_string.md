# Program

```rustleaf
"""This is a
multiline string
with multiple lines""";
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

# Lex

```rust
Ok(
    [
        Token(MultilineString, "This is a\nmultiline string\nwith multiple lines"),
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
