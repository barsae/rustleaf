# Program

```rustleaf
for x in [1, 2, 3] {
    print(x);
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: For,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: In,
            text: None,
        },
        Token {
            token_type: LeftBracket,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "1",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "2",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "3",
            ),
        },
        Token {
            token_type: RightBracket,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "print",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: Semicolon,
            text: None,
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
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                    body: Block {
                        statements: [
                            Expression(
                                FunctionCall(
                                    Identifier(
                                        "print",
                                    ),
                                    [
                                        Identifier(
                                            "x",
                                        ),
                                    ],
                                ),
                            ),
                        ],
                        final_expr: None,
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
    "Expression not yet implemented: For { pattern: Variable(\"x\"), iter: List([Literal(Int(1)), Literal(Int(2)), Literal(Int(3))]), body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Identifier(\"x\")]))], final_expr: None } }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: For { pattern: Variable(\"x\"), iter: List([Literal(Int(1)), Literal(Int(2)), Literal(Int(3))]), body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Identifier(\"x\")]))], final_expr: None } }",
)
```
