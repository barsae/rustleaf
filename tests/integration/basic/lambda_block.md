# Program 🔴
```rustleaf
// #[fail_quietly]
|x| {
    print(x);
    x + 1
};
```

# Output
```
None
```

# Result
```rust
Err(
    "Expression not yet implemented: Lambda { params: [\"x\"], body: Block(Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Identifier(\"x\")]))], final_expr: Some(Add(Identifier(\"x\"), Literal(Int(1)))) }) }",
)
```

# Lex
```rust
Ok(
    [
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBrace),
        Token(Semicolon),
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