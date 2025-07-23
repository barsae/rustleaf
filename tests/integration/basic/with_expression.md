# Program 🔴

```rustleaf
// #[fail_quietly]
with file = open("data.txt") {
    file.read()
}
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

# Lex

```rust
Ok(
    [
        Token(With),
        Token(Ident, "file"),
        Token(Equal),
        Token(Ident, "open"),
        Token(LeftParen),
        Token(String, "data.txt"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "file"),
        Token(Dot),
        Token(Ident, "read"),
        Token(LeftParen),
        Token(RightParen),
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
