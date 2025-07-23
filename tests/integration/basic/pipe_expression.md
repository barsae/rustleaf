# Program 🔴
```rustleaf
// #[fail_quietly]
data : transform() : validate();
```

# Output
```
None
```

# Result
```rust
Err(
    "Expression not yet implemented: Pipe(Pipe(Identifier(\"data\"), FunctionCall(Identifier(\"transform\"), [])), FunctionCall(Identifier(\"validate\"), []))",
)
```

# Lex
```rust
Ok(
    [
        Token(Ident, "data"),
        Token(Colon),
        Token(Ident, "transform"),
        Token(LeftParen),
        Token(RightParen),
        Token(Colon),
        Token(Ident, "validate"),
        Token(LeftParen),
        Token(RightParen),
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
                Pipe(
                    Pipe(
                        Identifier(
                            "data",
                        ),
                        FunctionCall(
                            Identifier(
                                "transform",
                            ),
                            [],
                        ),
                    ),
                    FunctionCall(
                        Identifier(
                            "validate",
                        ),
                        [],
                    ),
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Err(
    "Expression not yet implemented: Pipe(Pipe(Identifier(\"data\"), FunctionCall(Identifier(\"transform\"), [])), FunctionCall(Identifier(\"validate\"), []))",
)
```