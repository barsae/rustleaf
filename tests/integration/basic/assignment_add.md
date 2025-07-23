# Program

```rustleaf
x += 5;
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
        Token(PlusEqual),
        Token(Int, "5"),
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
                op: AddAssign,
                value: Literal(
                    Int(
                        5,
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
                Call(
                    GetAttr(
                        Variable(
                            "x",
                        ),
                        "op_add",
                    ),
                    [
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                    ],
                ),
            ),
        ],
        None,
    ),
)
```
