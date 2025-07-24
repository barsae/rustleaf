# Program
Status: 🟢
Assertions: 4

```rustleaf
var result = loop {
    break 42;
};

var result2 = loop {
    break "hello";
};

var counter = 0;
var result3 = loop {
    counter = counter + 1;
    if counter == 3 {
        break counter * 5;
    }
};

assert(result == 42);
assert(result2 == "hello");
assert(result3 == 15);
assert(counter == 3);
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
        Token(Ident, "result"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(Int, "42"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Break),
        Token(String, "hello"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Ident, "counter"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "counter"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Break),
        Token(Ident, "counter"),
        Token(Star),
        Token(Int, "5"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
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
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(String, "hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "counter"),
        Token(EqualEqual),
        Token(Int, "3"),
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
                    "result",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Break(
                                    Some(
                                        Literal(
                                            Int(
                                                42,
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Break(
                                    Some(
                                        Literal(
                                            String(
                                                "hello",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "counter",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "counter",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "counter",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Eq(
                                            Identifier(
                                                "counter",
                                            ),
                                            Literal(
                                                Int(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Break(
                                                    Some(
                                                        Mul(
                                                            Identifier(
                                                                "counter",
                                                            ),
                                                            Literal(
                                                                Int(
                                                                    5,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                            ],
                            final_expr: None,
                        },
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
                            Identifier(
                                "result2",
                            ),
                            Literal(
                                String(
                                    "hello",
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
                                Int(
                                    15,
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
                                "counter",
                            ),
                            Literal(
                                Int(
                                    3,
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
                "result",
                Some(
                    Loop(
                        Block(
                            [
                                Break(
                                    Some(
                                        Literal(
                                            Int(
                                                42,
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            None,
                        ),
                    ),
                ),
            ),
            Declare(
                "result2",
                Some(
                    Loop(
                        Block(
                            [
                                Break(
                                    Some(
                                        Literal(
                                            String(
                                                "hello",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            None,
                        ),
                    ),
                ),
            ),
            Declare(
                "counter",
                Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            ),
            Declare(
                "result3",
                Some(
                    Loop(
                        Block(
                            [
                                Assign(
                                    "counter",
                                    Call(
                                        GetAttr(
                                            Variable(
                                                "counter",
                                            ),
                                            "op_add",
                                        ),
                                        [
                                            Literal(
                                                Int(
                                                    1,
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                                If(
                                    Call(
                                        GetAttr(
                                            Variable(
                                                "counter",
                                            ),
                                            "op_eq",
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
                                        [
                                            Break(
                                                Some(
                                                    Call(
                                                        GetAttr(
                                                            Variable(
                                                                "counter",
                                                            ),
                                                            "op_mul",
                                                        ),
                                                        [
                                                            Literal(
                                                                Int(
                                                                    5,
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                        None,
                                    ),
                                    None,
                                ),
                            ],
                            None,
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
                            Variable(
                                "result2",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "hello",
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
                                Int(
                                    15,
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
                            Variable(
                                "counter",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    3,
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