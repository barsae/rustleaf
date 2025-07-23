# Program

```rustleaf
|| 42;
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

# Lex

```rust
Ok(
    [
        Token(Pipe),
        Token(Pipe),
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
