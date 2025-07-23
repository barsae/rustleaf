# Program 🟢
```rustleaf
var super = "not a keyword anymore";
assert(super == "not a keyword anymore");
```

# Output
None

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
        Token(Ident, "super"),
        Token(Equal),
        Token(String, "not a keyword anymore"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "super"),
        Token(EqualEqual),
        Token(String, "not a keyword anymore"),
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
                    "super",
                ),
                value: Some(
                    Literal(
                        String(
                            "not a keyword anymore",
                        ),
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "super",
                            ),
                            Literal(
                                String(
                                    "not a keyword anymore",
                                ),
                            ),
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
    Block(
        [
            Declare(
                "super",
                Some(
                    Literal(
                        String(
                            "not a keyword anymore",
                        ),
                    ),
                ),
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "super",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "not a keyword anymore",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```