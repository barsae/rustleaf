# Program
Status: 🟢
Assertions: 2

```rustleaf
var lambda = || 42;
var result = lambda();
assert(result == 42);
assert(is_unit(lambda) == false);  // Lambda should not be unit
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
        Token(Ident, "lambda"),
        Token(Equal),
        Token(Pipe),
        Token(Pipe),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "lambda"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "lambda"),
        Token(RightParen),
        Token(EqualEqual),
        Token(False),
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
                    "lambda",
                ),
                value: Some(
                    Lambda {
                        params: [],
                        body: Expression(
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "lambda",
                        ),
                        [],
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
                                "result",
                            ),
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Identifier(
                                        "lambda",
                                    ),
                                ],
                            ),
                            Literal(
                                Bool(
                                    false,
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
    Program(
        [
            Declare(
                "lambda",
                Some(
                    Lambda(
                        LambdaData {
                            params: [],
                            body: Literal(
                                Int(
                                    42,
                                ),
                            ),
                        },
                    ),
                ),
            ),
            Declare(
                "result",
                Some(
                    Call(
                        Variable(
                            "lambda",
                        ),
                        [],
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "result",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                Variable(
                                    "is_unit",
                                ),
                                [
                                    Variable(
                                        "lambda",
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Bool(
                                    false,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
)
```