# Program 🔴
```rustleaf
// #[fail_quietly]
if x > 0 {
    "positive"
} else {
    "zero or negative"
}
```

# Output
```
None
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
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "positive"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "zero or negative"),
        Token(RightBrace),
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
                If {
                    condition: Gt(
                        Identifier(
                            "x",
                        ),
                        Literal(
                            Int(
                                0,
                            ),
                        ),
                    ),
                    then_expr: Block {
                        statements: [],
                        final_expr: Some(
                            Literal(
                                String(
                                    "positive",
                                ),
                            ),
                        ),
                    },
                    else_expr: Some(
                        Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "zero or negative",
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [],
        Some(
            If(
                Call(
                    GetAttr(
                        Variable(
                            "x",
                        ),
                        "op_gt",
                    ),
                    [
                        Literal(
                            Int(
                                0,
                            ),
                        ),
                    ],
                ),
                Block(
                    [],
                    Some(
                        Literal(
                            String(
                                "positive",
                            ),
                        ),
                    ),
                ),
                Some(
                    Block(
                        [],
                        Some(
                            Literal(
                                String(
                                    "zero or negative",
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
)
```