# Program
Status: 🟡
Assertions: 0

```rustleaf
var e = parse("var a = 1; var b = 2;");
print(e);
```

# Output
```
var a = 1;
var b = 2;
```

# Result
```rust
Ok(
    Unit,
)
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "e"),
        Token(Equal),
        Token(Ident, "parse"),
        Token(LeftParen),
        Token(String, "var a = 1; var b = 2;"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "e"),
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
            VarDecl {
                pattern: Variable(
                    "e",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "parse",
                        ),
                        [
                            Literal(
                                String(
                                    "var a = 1; var b = 2;",
                                ),
                            ),
                        ],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Identifier(
                            "e",
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Program(
        [
            Declare(
                "e",
                Some(
                    Call(
                        Variable(
                            "parse",
                        ),
                        [
                            Literal(
                                String(
                                    "var a = 1; var b = 2;",
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            Call(
                Variable(
                    "print",
                ),
                [
                    Variable(
                        "e",
                    ),
                ],
            ),
        ],
    ),
)
```