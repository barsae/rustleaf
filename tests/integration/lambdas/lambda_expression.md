# Program
Status: 🟢
Assertions: 4

```rustleaf
var increment = |x| x + 1;
var double = |y| y * 2;
var add_ten = |z| z + 10;

assert(increment(5) == 6);
assert(double(7) == 14);  
assert(add_ten(15) == 25);
assert(increment(0) == 1);
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
        Token(Ident, "increment"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "double"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "y"),
        Token(Pipe),
        Token(Ident, "y"),
        Token(Star),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "add_ten"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "z"),
        Token(Pipe),
        Token(Ident, "z"),
        Token(Plus),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "increment"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "double"),
        Token(LeftParen),
        Token(Int, "7"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "14"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add_ten"),
        Token(LeftParen),
        Token(Int, "15"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "25"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "increment"),
        Token(LeftParen),
        Token(Int, "0"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "1"),
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
                    "increment",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "double",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "y",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "y",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "add_ten",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "z",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "z",
                                ),
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                            ),
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
                            FunctionCall(
                                Identifier(
                                    "increment",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    6,
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
                                    "double",
                                ),
                                [
                                    Literal(
                                        Int(
                                            7,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    14,
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
                                    "add_ten",
                                ),
                                [
                                    Literal(
                                        Int(
                                            15,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    25,
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
                                    "increment",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ],
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
                "increment",
                Some(
                    Lambda(
                        LambdaData {
                            params: [
                                "x",
                            ],
                            body: Call(
                                GetAttr(
                                    Variable(
                                        "x",
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
                        },
                    ),
                ),
            ),
            Declare(
                "double",
                Some(
                    Lambda(
                        LambdaData {
                            params: [
                                "y",
                            ],
                            body: Call(
                                GetAttr(
                                    Variable(
                                        "y",
                                    ),
                                    "op_mul",
                                ),
                                [
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ],
                            ),
                        },
                    ),
                ),
            ),
            Declare(
                "add_ten",
                Some(
                    Lambda(
                        LambdaData {
                            params: [
                                "z",
                            ],
                            body: Call(
                                GetAttr(
                                    Variable(
                                        "z",
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                ],
                            ),
                        },
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
                            Call(
                                Variable(
                                    "increment",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    6,
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
                                    "double",
                                ),
                                [
                                    Literal(
                                        Int(
                                            7,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    14,
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
                                    "add_ten",
                                ),
                                [
                                    Literal(
                                        Int(
                                            15,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    25,
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
                                    "increment",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ],
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
        ],
    ),
)
```