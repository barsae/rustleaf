# Program
Status: 🟢

```rustleaf
// Test lambda expressions in practical usage scenarios

// Simple lambda assignment and calling
var add_one = |x| x + 1;
assert(add_one(5) == 6);

// Lambda with multiple parameters (when supported)
// var add = |x, y| x + y;
// assert(add(3, 4) == 7);

// Lambda as argument to other functions (higher-order functions)
fn apply(func, value) {
    func(value)
}
var double = |x| x * 2;
assert(apply(double, 21) == 42);

// Lambda with block body
var complex_func = |n| {
    var result = n * n;
    result + 1
};
assert(complex_func(3) == 10); // 3*3 + 1 = 10

// Lambda closures - capturing variables from outer scope
var multiplier = 3;
var multiply_by_three = |x| x * multiplier;
assert(multiply_by_three(4) == 12);
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
        Token(Ident, "add_one"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add_one"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "apply"),
        Token(LeftParen),
        Token(Ident, "func"),
        Token(Comma),
        Token(Ident, "value"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "func"),
        Token(LeftParen),
        Token(Ident, "value"),
        Token(RightParen),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "double"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Star),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "apply"),
        Token(LeftParen),
        Token(Ident, "double"),
        Token(Comma),
        Token(Int, "21"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "complex_func"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "n"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "n"),
        Token(Star),
        Token(Ident, "n"),
        Token(Semicolon),
        Token(Ident, "result"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "complex_func"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "10"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "multiplier"),
        Token(Equal),
        Token(Int, "3"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "multiply_by_three"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Star),
        Token(Ident, "multiplier"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiply_by_three"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "12"),
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
                    "add_one",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "add_one",
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
            FnDecl {
                name: "apply",
                params: [
                    Parameter {
                        name: "func",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "value",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        FunctionCall(
                            Identifier(
                                "func",
                            ),
                            [
                                Identifier(
                                    "value",
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "double",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "x",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "apply",
                                ),
                                [
                                    Identifier(
                                        "double",
                                    ),
                                    Literal(
                                        Int(
                                            21,
                                        ),
                                    ),
                                ],
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
            VarDecl {
                pattern: Variable(
                    "complex_func",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "n",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "result",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "n",
                                                ),
                                                Identifier(
                                                    "n",
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Add(
                                        Identifier(
                                            "result",
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
                                    "complex_func",
                                ),
                                [
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "multiplier",
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
                    "multiply_by_three",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "x",
                                ),
                                Identifier(
                                    "multiplier",
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
                                    "multiply_by_three",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    12,
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
                "add_one",
                Some(
                    Lambda(
                        [
                            "x",
                        ],
                        Call(
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
                                    "add_one",
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
            Function(
                "apply",
                [
                    "func",
                    "value",
                ],
                Block(
                    [],
                    Some(
                        Call(
                            Variable(
                                "func",
                            ),
                            [
                                Variable(
                                    "value",
                                ),
                            ],
                        ),
                    ),
                ),
            ),
            Declare(
                "double",
                Some(
                    Lambda(
                        [
                            "x",
                        ],
                        Call(
                            GetAttr(
                                Variable(
                                    "x",
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
                                    "apply",
                                ),
                                [
                                    Variable(
                                        "double",
                                    ),
                                    Literal(
                                        Int(
                                            21,
                                        ),
                                    ),
                                ],
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
            Declare(
                "complex_func",
                Some(
                    Lambda(
                        [
                            "n",
                        ],
                        Block(
                            [
                                Declare(
                                    "result",
                                    Some(
                                        Call(
                                            GetAttr(
                                                Variable(
                                                    "n",
                                                ),
                                                "op_mul",
                                            ),
                                            [
                                                Variable(
                                                    "n",
                                                ),
                                            ],
                                        ),
                                    ),
                                ),
                            ],
                            Some(
                                Call(
                                    GetAttr(
                                        Variable(
                                            "result",
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
                            Call(
                                Variable(
                                    "complex_func",
                                ),
                                [
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Declare(
                "multiplier",
                Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            ),
            Declare(
                "multiply_by_three",
                Some(
                    Lambda(
                        [
                            "x",
                        ],
                        Call(
                            GetAttr(
                                Variable(
                                    "x",
                                ),
                                "op_mul",
                            ),
                            [
                                Variable(
                                    "multiplier",
                                ),
                            ],
                        ),
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
                            Call(
                                Variable(
                                    "multiply_by_three",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    12,
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