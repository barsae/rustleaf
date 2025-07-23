# Program

```rustleaf
pub fn greet(name) {
    print("Hello, " + name);
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Pub,
            text: None,
        },
        Token {
            token_type: Fn,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "greet",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
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
                "print",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "Hello, ",
            ),
        },
        Token {
            token_type: Plus,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
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
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        Expression(
                            FunctionCall(
                                Identifier(
                                    "print",
                                ),
                                [
                                    Add(
                                        Literal(
                                            String(
                                                "Hello, ",
                                            ),
                                        ),
                                        Identifier(
                                            "name",
                                        ),
                                    ),
                                ],
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: true,
            },
        ],
    ),
)
```

# Eval

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"greet\", params: [Parameter { name: \"name\", default: None, kind: Regular }], body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Add(Literal(String(\"Hello, \")), Identifier(\"name\"))]))], final_expr: None }, is_pub: true }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: FnDecl { name: \"greet\", params: [Parameter { name: \"name\", default: None, kind: Regular }], body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Add(Literal(String(\"Hello, \")), Identifier(\"name\"))]))], final_expr: None }, is_pub: true }",
)
```
