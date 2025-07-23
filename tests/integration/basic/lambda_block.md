# Program

```rustleaf
|x| {
    print(x);
    x + 1
};
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Pipe,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "x",
            ),
        },
        Token {
            token_type: Pipe,
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
                "1",
            ),
        },
        Token {
            token_type: RightBrace,
            text: None,
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
            Expression(
                Lambda {
                    params: [
                        "x",
                    ],
                    body: Block(
                        Block {
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
                            final_expr: Some(
                                Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            1,
                                        ),
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
Err(
    "Expression not yet implemented: Lambda { params: [\"x\"], body: Block(Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Identifier(\"x\")]))], final_expr: Some(Add(Identifier(\"x\"), Literal(Int(1)))) }) }",
)
```

# Output

```

```

# Result

```rust
Err(
    "Expression not yet implemented: Lambda { params: [\"x\"], body: Block(Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Identifier(\"x\")]))], final_expr: Some(Add(Identifier(\"x\"), Literal(Int(1)))) }) }",
)
```
