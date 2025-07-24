# Program
Status: 🟢
Assertions: 4

```rustleaf
var positive = 42;
var negative = -positive;
var double_neg = -negative;
assert(negative == -42);
assert(double_neg == 42);
assert(-100 == -100);
assert(-(5 + 3) == -8);
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
        Token(Ident, "positive"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "negative"),
        Token(Equal),
        Token(Minus),
        Token(Ident, "positive"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "double_neg"),
        Token(Equal),
        Token(Minus),
        Token(Ident, "negative"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "negative"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "double_neg"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Minus),
        Token(Int, "100"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "100"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Minus),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Plus),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "8"),
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
                    "positive",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
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
                        Identifier(
                            "positive",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "double_neg",
                ),
                value: Some(
                    Neg(
                        Identifier(
                            "negative",
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
                                "negative",
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        42,
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
                            Identifier(
                                "double_neg",
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
                            Neg(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        100,
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
                            Neg(
                                Add(
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        8,
                                    ),
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
                "positive",
                Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            ),
            Declare(
                "negative",
                Some(
                    Call(
                        GetAttr(
                            Variable(
                                "positive",
                            ),
                            "op_neg",
                        ),
                        [],
                    ),
                ),
            ),
            Declare(
                "double_neg",
                Some(
                    Call(
                        GetAttr(
                            Variable(
                                "negative",
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
                                "negative",
                            ),
                            "op_eq",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            42,
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
                            Variable(
                                "double_neg",
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
                                GetAttr(
                                    Literal(
                                        Int(
                                            100,
                                        ),
                                    ),
                                    "op_neg",
                                ),
                                [],
                            ),
                            "op_eq",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            100,
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
                                    Call(
                                        GetAttr(
                                            Literal(
                                                Int(
                                                    5,
                                                ),
                                            ),
                                            "op_add",
                                        ),
                                        [
                                            Literal(
                                                Int(
                                                    3,
                                                ),
                                            ),
                                        ],
                                    ),
                                    "op_neg",
                                ),
                                [],
                            ),
                            "op_eq",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            8,
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
        ),
    ),
)
```