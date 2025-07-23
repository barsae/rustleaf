# Program

```rustleaf
x = 42;
```

# Output

```

```

# Result

```rust
Err(
    "Undefined variable: x",
)
```

# Lex

```rust
Ok(
    [
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "42"),
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
            Assignment {
                target: Identifier(
                    "x",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        42,
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
            Assign(
                "x",
                Literal(
                    Int(
                        42,
                    ),
                ),
            ),
        ],
        None,
    ),
)
```
