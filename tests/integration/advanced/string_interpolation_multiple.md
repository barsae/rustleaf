# Program 🟢
```rustleaf
var a = 3;
var b = 7;
var result = "${a} and ${b} equals ${a + b}";
assert(result == "3 and 7 equals 10");
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
        Token(Ident, "a"),
        Token(Equal),
        Token(Int, "3"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "b"),
        Token(Equal),
        Token(Int, "7"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(InterpolationStart),
        Token(Ident, "a"),
        Token(InterpolationEnd),
        Token(StringPart, " and "),
        Token(InterpolationStart),
        Token(Ident, "b"),
        Token(InterpolationEnd),
        Token(StringPart, " equals "),
        Token(InterpolationStart),
        Token(Ident, "a"),
        Token(Plus),
        Token(Ident, "b"),
        Token(InterpolationEnd),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "3 and 7 equals 10"),
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
                    "a",
                ),
                value: Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "b",
                ),
                value: Some(
                    Literal(
                        Int(
                            7,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Expression(
                                Identifier(
                                    "a",
                                ),
                            ),
                            Text(
                                " and ",
                            ),
                            Expression(
                                Identifier(
                                    "b",
                                ),
                            ),
                            Text(
                                " equals ",
                            ),
                            Expression(
                                Add(
                                    Identifier(
                                        "a",
                                    ),
                                    Identifier(
                                        "b",
                                    ),
                                ),
                            ),
                        ],
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
                                String(
                                    "3 and 7 equals 10",
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
                "a",
                Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            ),
            Declare(
                "b",
                Some(
                    Literal(
                        Int(
                            7,
                        ),
                    ),
                ),
            ),
            Declare(
                "result",
                Some(
                    Call(
                        GetAttr(
                            Call(
                                GetAttr(
                                    Call(
                                        GetAttr(
                                            Call(
                                                GetAttr(
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            Variable(
                                                                "a",
                                                            ),
                                                        ],
                                                    ),
                                                    "op_add",
                                                ),
                                                [
                                                    Literal(
                                                        String(
                                                            " and ",
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            "op_add",
                                        ),
                                        [
                                            Call(
                                                Variable(
                                                    "str",
                                                ),
                                                [
                                                    Variable(
                                                        "b",
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        String(
                                            " equals ",
                                        ),
                                    ),
                                ],
                            ),
                            "op_add",
                        ),
                        [
                            Call(
                                Variable(
                                    "str",
                                ),
                                [
                                    Call(
                                        GetAttr(
                                            Variable(
                                                "a",
                                            ),
                                            "op_add",
                                        ),
                                        [
                                            Variable(
                                                "b",
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                        ],
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
                                "result",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "3 and 7 equals 10",
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