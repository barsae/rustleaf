# Program
Status: 🔴
Assertions: 0

```rustleaf
// #[fail_quietly]
try {
    risky_operation()
} catch e {
    "error occurred"
}
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: risky_operation",
)
```

# Lex
```rust
Ok(
    [
        Token(Try),
        Token(LeftBrace),
        Token(Ident, "risky_operation"),
        Token(LeftParen),
        Token(RightParen),
        Token(RightBrace),
        Token(Catch),
        Token(Ident, "e"),
        Token(LeftBrace),
        Token(String, "error occurred"),
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
                Try {
                    body: Block {
                        statements: [],
                        final_expr: Some(
                            FunctionCall(
                                Identifier(
                                    "risky_operation",
                                ),
                                [],
                            ),
                        ),
                    },
                    catch: CatchClause {
                        pattern: Variable(
                            "e",
                        ),
                        body: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "error occurred",
                                    ),
                                ),
                            ),
                        },
                    },
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
            Try(
                Block(
                    [],
                    Some(
                        Call(
                            Variable(
                                "risky_operation",
                            ),
                            [],
                        ),
                    ),
                ),
                "e",
                Block(
                    [],
                    Some(
                        Literal(
                            String(
                                "error occurred",
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
)
```