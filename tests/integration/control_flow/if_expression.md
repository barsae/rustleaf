# Program
Status: 🟢
Assertions: 0

```rustleaf
var x = 1;
if x > 0 {
    "positive"
} else {
    "zero or negative"
}
```

# Output
None

# Result
```rust
Ok(
    String(
        "positive",
    ),
)
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
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
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            },
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
        [
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            ),
        ],
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