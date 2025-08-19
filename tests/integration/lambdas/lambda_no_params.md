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
        0: Token(Var),
        1: Token(Ident, "lambda"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Pipe),
        5: Token(Int, "42"),
        6: Token(Semicolon),
        7: Token(Var),
        8: Token(Ident, "result"),
        9: Token(Equal),
        10: Token(Ident, "lambda"),
        11: Token(LeftParen),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Ident, "result"),
        17: Token(EqualEqual),
        18: Token(Int, "42"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "is_unit"),
        24: Token(LeftParen),
        25: Token(Ident, "lambda"),
        26: Token(RightParen),
        27: Token(EqualEqual),
        28: Token(False),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Eof)
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
    RustValue(<unknown>),
)
```