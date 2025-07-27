# Program
Status: 🟢
Assertions: 3

```rustleaf
var result1 = if true { 42 } else { 0 };
var result2 = if false { 100 } else { 200 };
var x = 5;
var result3 = if x > 3 { "big" } else { "small" };
assert(result1 == 42);
assert(result2 == 200);
assert(result3 == "big");
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
        Token(Ident, "result1"),
        Token(Equal),
        Token(If),
        Token(True),
        Token(LeftBrace),
        Token(Int, "42"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(If),
        Token(False),
        Token(LeftBrace),
        Token(Int, "100"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Int, "200"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(String, "big"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "small"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result1"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Int, "200"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(String, "big"),
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
                    "result1",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                true,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                false,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            200,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "big",
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        String(
                                            "small",
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
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
                                "result1",
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
                            Identifier(
                                "result2",
                            ),
                            Literal(
                                Int(
                                    200,
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
                                "result3",
                            ),
                            Literal(
                                String(
                                    "big",
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
                "result1",
                Some(
                    If(
                        Literal(
                            Bool(
                                true,
                            ),
                        ),
                        Block(
                            [],
                            Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        ),
                        Some(
                            Block(
                                [],
                                Some(
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Declare(
                "result2",
                Some(
                    If(
                        Literal(
                            Bool(
                                false,
                            ),
                        ),
                        Block(
                            [],
                            Some(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                        ),
                        Some(
                            Block(
                                [],
                                Some(
                                    Literal(
                                        Int(
                                            200,
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            ),
            Declare(
                "result3",
                Some(
                    If(
                        Call(
                            GetAttr(
                                Variable(
                                    "x",
                                ),
                                "op_gt",
                            ),
                            [
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                            ],
                        ),
                        Block(
                            [],
                            Some(
                                Literal(
                                    String(
                                        "big",
                                    ),
                                ),
                            ),
                        ),
                        Some(
                            Block(
                                [],
                                Some(
                                    Literal(
                                        String(
                                            "small",
                                        ),
                                    ),
                                ),
                            ),
                        ),
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
                                "result1",
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
                            Variable(
                                "result2",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    200,
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
                                "result3",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "big",
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