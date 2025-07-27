# Program
Status: 🟢
Assertions: 3

```rustleaf
var y;
var z;
y = 100;
z = "assigned later";
assert(y == 100);
assert(z == "assigned later");
assert(y + 23 == 123);
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
        Token(Ident, "y"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "z"),
        Token(Semicolon),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "100"),
        Token(Semicolon),
        Token(Ident, "z"),
        Token(Equal),
        Token(String, "assigned later"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(EqualEqual),
        Token(Int, "100"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
        Token(EqualEqual),
        Token(String, "assigned later"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(Plus),
        Token(Int, "23"),
        Token(EqualEqual),
        Token(Int, "123"),
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
                    "y",
                ),
                value: None,
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: None,
            },
            Assignment {
                target: Identifier(
                    "y",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        100,
                    ),
                ),
            },
            Assignment {
                target: Identifier(
                    "z",
                ),
                op: Assign,
                value: Literal(
                    String(
                        "assigned later",
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
                                "y",
                            ),
                            Literal(
                                Int(
                                    100,
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
                                "z",
                            ),
                            Literal(
                                String(
                                    "assigned later",
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
                                    "y",
                                ),
                                Literal(
                                    Int(
                                        23,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    123,
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
                "y",
                None,
            ),
            Declare(
                "z",
                None,
            ),
            Assign(
                "y",
                Literal(
                    Int(
                        100,
                    ),
                ),
            ),
            Assign(
                "z",
                Literal(
                    String(
                        "assigned later",
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
                                "y",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    100,
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
                                "z",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "assigned later",
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
                                    Variable(
                                        "y",
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            23,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    123,
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