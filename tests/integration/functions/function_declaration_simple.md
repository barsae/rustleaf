# Program
Status: 🟢
Assertions: 4

```rustleaf
fn add(x, y) { x + y }
fn multiply(a, b) { a * b }

var sum = add(5, 3);
var product = multiply(4, 6);

assert(sum == 8);
assert(product == 24);
assert(add(10, -2) == 8);
assert(multiply(0, 100) == 0);
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
        Token(Fn),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Ident, "a"),
        Token(Comma),
        Token(Ident, "b"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "a"),
        Token(Star),
        Token(Ident, "b"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "sum"),
        Token(Equal),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum"),
        Token(EqualEqual),
        Token(Int, "8"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product"),
        Token(EqualEqual),
        Token(Int, "24"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Comma),
        Token(Minus),
        Token(Int, "2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "8"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "0"),
        Token(Comma),
        Token(Int, "100"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "0"),
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
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "multiply",
                params: [
                    Parameter {
                        name: "a",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "b",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Mul(
                            Identifier(
                                "a",
                            ),
                            Identifier(
                                "b",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "sum",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
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
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    6,
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
                                "sum",
                            ),
                            Literal(
                                Int(
                                    8,
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
                                "product",
                            ),
                            Literal(
                                Int(
                                    24,
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
                                    "add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    Neg(
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    8,
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
                                    "multiply",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            100,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    0,
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
    Eval(
        EvalRef(
            EvalProgram {
                statements: [
                    EvalRef(
                        EvalFunction {
                            data: FunctionData {
                                name: "add",
                                params: [
                                    (
                                        "x",
                                        None,
                                        Regular,
                                    ),
                                    (
                                        "y",
                                        None,
                                        Regular,
                                    ),
                                ],
                                body: Eval(
                                    EvalRef(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalVariable {
                                                                        name: "x",
                                                                    },
                                                                ),
                                                                attr_name: "op_add",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalVariable {
                                                                    name: "y",
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    EvalRef(
                        EvalFunction {
                            data: FunctionData {
                                name: "multiply",
                                params: [
                                    (
                                        "a",
                                        None,
                                        Regular,
                                    ),
                                    (
                                        "b",
                                        None,
                                        Regular,
                                    ),
                                ],
                                body: Eval(
                                    EvalRef(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalVariable {
                                                                        name: "a",
                                                                    },
                                                                ),
                                                                attr_name: "op_mul",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalVariable {
                                                                    name: "b",
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "sum",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "add",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        5,
                                                    ),
                                                },
                                            ),
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "product",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "multiply",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        4,
                                                    ),
                                                },
                                            ),
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        6,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalVariable {
                                                        name: "sum",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        8,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalVariable {
                                                        name: "product",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        24,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "add",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        10,
                                                                    ),
                                                                },
                                                            ),
                                                            EvalRef(
                                                                EvalCall {
                                                                    func_expr: EvalRef(
                                                                        EvalGetAttr {
                                                                            obj_expr: EvalRef(
                                                                                EvalLiteral {
                                                                                    value: Int(
                                                                                        2,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                            attr_name: "op_neg",
                                                                        },
                                                                    ),
                                                                    args: [],
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        8,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "multiply",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        0,
                                                                    ),
                                                                },
                                                            ),
                                                            EvalRef(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        100,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        0,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                ],
            },
        ),
    ),
)
```