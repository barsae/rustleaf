# Program
Status: 🟢
Assertions: 4

```rustleaf
var x = 1;
var result = if x > 0 {
    "positive"
} else {
    "zero or negative"
};

var y = -5;
var result2 = if y > 0 {
    "positive"
} else {
    "zero or negative"
};

assert(result == "positive");
assert(result2 == "zero or negative");
assert(x == 1);
assert(y == -5);
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "positive"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "zero or negative"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "y"),
        Token(Equal),
        Token(Minus),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(If),
        Token(Ident, "y"),
        Token(Greater),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(String, "positive"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "zero or negative"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "positive"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(String, "zero or negative"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "y"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "5"),
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
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
                    "y",
                ),
                value: Some(
                    Neg(
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "y",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "positive",
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
                                    "zero or negative",
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
                                "x",
                            ),
                            Literal(
                                Int(
                                    1,
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
                                "y",
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        5,
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
    Program(
        [
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            1,
                        ),
                    ),
                ),
            ),
            Declare(
                "result",
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
                                        0,
                                    ),
                                ),
                            ],
                        ),
                        Block(
                            [],
                            Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            Declare(
                "y",
                Some(
                    Call(
                        GetAttr(
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            "op_neg",
                        ),
                        [],
                    ),
                ),
            ),
            Declare(
                "result2",
                Some(
                    If(
                        Call(
                            GetAttr(
                                Variable(
                                    "y",
                                ),
                                "op_gt",
                            ),
                            [
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ],
                        ),
                        Block(
                            [],
                            Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
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
                                "result",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "positive",
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
                                    "zero or negative",
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
                                "x",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
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
                                "y",
                            ),
                            "op_eq",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            5,
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
    ),
)
```