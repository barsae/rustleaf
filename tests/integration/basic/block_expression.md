# Program

```rustleaf
{
    var x = 10;
    x + 5
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Var,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "10",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Plus,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "5",
            ),
        },
        Token {
            token_type: RightBrace,
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
                Block(
                    Block {
                        statements: [
                            VarDecl {
                                pattern: Variable(
                                    "x",
                                ),
                                value: Some(
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                ),
                            },
                        ],
                        final_expr: Some(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
                        ),
                    },
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
            Block(
                [
                    Declare(
                        "x",
                        Some(
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ),
                    ),
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
                ],
                None,
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
