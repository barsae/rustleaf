# Program

```rustleaf
with file = open("data.txt") {
    file.read()
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: With,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "file",
            ),
        },
        Token {
            token_type: Equal,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "open",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "data.txt",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "file",
            ),
        },
        Token {
            token_type: Dot,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "read",
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
                With {
                    resources: [
                        WithResource {
                            name: "file",
                            value: FunctionCall(
                                Identifier(
                                    "open",
                                ),
                                [
                                    Literal(
                                        String(
                                            "data.txt",
                                        ),
                                    ),
                                ],
                            ),
                        },
                    ],
                    body: Block {
                        statements: [],
                        final_expr: Some(
                            MethodCall(
                                Identifier(
                                    "file",
                                ),
                                "read",
                                [],
                            ),
                        ),
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
    "Expression not yet implemented: With { resources: [WithResource { name: \"file\", value: FunctionCall(Identifier(\"open\"), [Literal(String(\"data.txt\"))]) }], body: Block { statements: [], final_expr: Some(MethodCall(Identifier(\"file\"), \"read\", [])) } }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: With { resources: [WithResource { name: \"file\", value: FunctionCall(Identifier(\"open\"), [Literal(String(\"data.txt\"))]) }], body: Block { statements: [], final_expr: Some(MethodCall(Identifier(\"file\"), \"read\", [])) } }",
)
```
