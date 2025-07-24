# Program
Status: 🟢
Assertions: 5

```rustleaf
var pi = 3.14;
var small = 0.1;
var negative = -2.5;
assert(pi == 3.14);
assert(small == 0.1);
assert(negative == -2.5);
assert(pi + small == 3.24);
assert(pi * 2.0 == 6.28);
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
        Token(Ident, "pi"),
        Token(Equal),
        Token(Float, "3.14"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "small"),
        Token(Equal),
        Token(Float, "0.1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "negative"),
        Token(Equal),
        Token(Minus),
        Token(Float, "2.5"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "pi"),
        Token(EqualEqual),
        Token(Float, "3.14"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "small"),
        Token(EqualEqual),
        Token(Float, "0.1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "negative"),
        Token(EqualEqual),
        Token(Minus),
        Token(Float, "2.5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "pi"),
        Token(Plus),
        Token(Ident, "small"),
        Token(EqualEqual),
        Token(Float, "3.24"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "pi"),
        Token(Star),
        Token(Float, "2.0"),
        Token(EqualEqual),
        Token(Float, "6.28"),
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
                    "pi",
                ),
                value: Some(
                    Literal(
                        Float(
                            3.14,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "small",
                ),
                value: Some(
                    Literal(
                        Float(
                            0.1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "negative",
                ),
                value: Some(
                    Neg(
                        Literal(
                            Float(
                                2.5,
                            ),
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
                                "pi",
                            ),
                            Literal(
                                Float(
                                    3.14,
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
                            Identifier(
                                "small",
                            ),
                            Literal(
                                Float(
                                    0.1,
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
                            Identifier(
                                "negative",
                            ),
                            Neg(
                                Literal(
                                    Float(
                                        2.5,
                                    ),
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
                            Add(
                                Identifier(
                                    "pi",
                                ),
                                Identifier(
                                    "small",
                                ),
                            ),
                            Literal(
                                Float(
                                    3.24,
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
                            Mul(
                                Identifier(
                                    "pi",
                                ),
                                Literal(
                                    Float(
                                        2.0,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    6.28,
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
                "pi",
                Some(
                    Literal(
                        Float(
                            3.14,
                        ),
                    ),
                ),
            ),
            Declare(
                "small",
                Some(
                    Literal(
                        Float(
                            0.1,
                        ),
                    ),
                ),
            ),
            Declare(
                "negative",
                Some(
                    Call(
                        GetAttr(
                            Literal(
                                Float(
                                    2.5,
                                ),
                            ),
                            "op_neg",
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
                                "pi",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Float(
                                    3.14,
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
                            Variable(
                                "small",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Float(
                                    0.1,
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
                            Variable(
                                "negative",
                            ),
                            "op_eq",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Float(
                                            2.5,
                                        ),
                                    ),
                                    "op_neg",
                                ),
                                [],
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
                                GetAttr(
                                    Variable(
                                        "pi",
                                    ),
                                    "op_add",
                                ),
                                [
                                    Variable(
                                        "small",
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Float(
                                    3.24,
                                ),
                            ),
                        ],
                    ),
                ],
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
                            Call(
                                GetAttr(
                                    Variable(
                                        "pi",
                                    ),
                                    "op_mul",
                                ),
                                [
                                    Literal(
                                        Float(
                                            2.0,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Float(
                                    6.28,
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