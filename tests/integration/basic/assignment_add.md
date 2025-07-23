# Program

```rustleaf
x += 5;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: PlusEqual,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "5",
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
                BinaryOp(
                    Add,
                    Variable(
                        "x",
                    ),
                    Literal(
                        Int(
                            5,
                        ),
                    ),
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
Err(
    "eval not implemented for: BinaryOp(Add, Variable(\"x\"), Literal(Int(5)))",
)
```
