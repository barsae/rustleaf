# Program

```rustleaf
try {
    risky_operation()
} catch e {
    "error occurred"
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Try,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "risky_operation",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Catch,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "e",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "error occurred",
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
Err(
    "Expression not yet implemented: Try { body: Block { statements: [], final_expr: Some(FunctionCall(Identifier(\"risky_operation\"), [])) }, catch: CatchClause { pattern: Variable(\"e\"), body: Block { statements: [], final_expr: Some(Literal(String(\"error occurred\"))) } } }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Try { body: Block { statements: [], final_expr: Some(FunctionCall(Identifier(\"risky_operation\"), [])) }, catch: CatchClause { pattern: Variable(\"e\"), body: Block { statements: [], final_expr: Some(Literal(String(\"error occurred\"))) } } }",
)
```
