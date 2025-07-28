# Program
Status: 🟢
Assertions: 4

```rustleaf
// Test using ranges in for loops and expressions
var sum = 0;
for i in 1..5 {
    sum += i;
}
assert(sum == 10);  // 1 + 2 + 3 + 4 = 10

// Test using ranges with different step sizes
var range = 0..10;
var even_count = 0;
for x in range {
    if x % 2 == 0 {
        even_count += 1;
    }
}
assert(even_count == 5);  // 0, 2, 4, 6, 8

// Test range as expression
var small_range = 3..6;
assert(4 in small_range);
assert(not (6 in small_range));
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
        Token(Ident, "sum"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "i"),
        Token(In),
        Token(Int, "1"),
        Token(DotDot),
        Token(Int, "5"),
        Token(LeftBrace),
        Token(Ident, "sum"),
        Token(PlusEqual),
        Token(Ident, "i"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum"),
        Token(EqualEqual),
        Token(Int, "10"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "range"),
        Token(Equal),
        Token(Int, "0"),
        Token(DotDot),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "even_count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "x"),
        Token(In),
        Token(Ident, "range"),
        Token(LeftBrace),
        Token(If),
        Token(Ident, "x"),
        Token(Percent),
        Token(Int, "2"),
        Token(EqualEqual),
        Token(Int, "0"),
        Token(LeftBrace),
        Token(Ident, "even_count"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "even_count"),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "small_range"),
        Token(Equal),
        Token(Int, "3"),
        Token(DotDot),
        Token(Int, "6"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(In),
        Token(Ident, "small_range"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "6"),
        Token(In),
        Token(Ident, "small_range"),
        Token(RightParen),
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
                    "sum",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "i",
                    ),
                    iter: RangeExclusive(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "sum",
                                ),
                                op: AddAssign,
                                value: Identifier(
                                    "i",
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                },
            ),
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
                                    10,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "range",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                0,
                            ),
                        ),
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "even_count",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: Identifier(
                        "range",
                    ),
                    body: Block {
                        statements: [],
                        final_expr: Some(
                            If {
                                condition: Eq(
                                    Mod(
                                        Identifier(
                                            "x",
                                        ),
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ),
                                then_expr: Block {
                                    statements: [
                                        Assignment {
                                            target: Identifier(
                                                "even_count",
                                            ),
                                            op: AddAssign,
                                            value: Literal(
                                                Int(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ],
                                    final_expr: None,
                                },
                                else_expr: None,
                            },
                        ),
                    },
                },
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "even_count",
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "small_range",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                3,
                            ),
                        ),
                        Literal(
                            Int(
                                6,
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
                        In(
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Identifier(
                                "small_range",
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
                        Not(
                            In(
                                Literal(
                                    Int(
                                        6,
                                    ),
                                ),
                                Identifier(
                                    "small_range",
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
                        EvalDeclare {
                            name: "sum",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            0,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalFor {
                            var_name: "i",
                            iter_expr: EvalRef(
                                EvalLiteral {
                                    value: Range(
                                        Range {
                                            start: 1,
                                            end: 5,
                                            inclusive: false,
                                        },
                                    ),
                                },
                            ),
                            body: EvalRef(
                                EvalBlock {
                                    statements: [
                                        EvalRef(
                                            EvalAssign {
                                                name: "sum",
                                                expr: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalVariable {
                                                                        name: "sum",
                                                                    },
                                                                ),
                                                                attr_name: "op_add",
                                                            },
                                                        ),
                                                        args: [
                                                            EvalRef(
                                                                EvalVariable {
                                                                    name: "i",
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            },
                                        ),
                                    ],
                                    final_expr: None,
                                },
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
                                                        10,
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
                        EvalDeclare {
                            name: "range",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Range(
                                            Range {
                                                start: 0,
                                                end: 10,
                                                inclusive: false,
                                            },
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "even_count",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            0,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalFor {
                            var_name: "x",
                            iter_expr: EvalRef(
                                EvalVariable {
                                    name: "range",
                                },
                            ),
                            body: EvalRef(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        EvalRef(
                                            EvalIf {
                                                condition: EvalRef(
                                                    EvalCall {
                                                        func_expr: EvalRef(
                                                            EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    EvalCall {
                                                                        func_expr: EvalRef(
                                                                            EvalGetAttr {
                                                                                obj_expr: EvalRef(
                                                                                    EvalVariable {
                                                                                        name: "x",
                                                                                    },
                                                                                ),
                                                                                attr_name: "op_mod",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            EvalRef(
                                                                                EvalLiteral {
                                                                                    value: Int(
                                                                                        2,
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
                                                then_expr: EvalRef(
                                                    EvalBlock {
                                                        statements: [
                                                            EvalRef(
                                                                EvalAssign {
                                                                    name: "even_count",
                                                                    expr: EvalRef(
                                                                        EvalCall {
                                                                            func_expr: EvalRef(
                                                                                EvalGetAttr {
                                                                                    obj_expr: EvalRef(
                                                                                        EvalVariable {
                                                                                            name: "even_count",
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "op_add",
                                                                                },
                                                                            ),
                                                                            args: [
                                                                                EvalRef(
                                                                                    EvalLiteral {
                                                                                        value: Int(
                                                                                            1,
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            ],
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                        final_expr: None,
                                                    },
                                                ),
                                                else_expr: None,
                                            },
                                        ),
                                    ),
                                },
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
                                                        name: "even_count",
                                                    },
                                                ),
                                                attr_name: "op_eq",
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
                                        ],
                                    },
                                ),
                            ],
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "small_range",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Range(
                                            Range {
                                                start: 3,
                                                end: 6,
                                                inclusive: false,
                                            },
                                        ),
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
                                                        name: "small_range",
                                                    },
                                                ),
                                                attr_name: "op_contains",
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
                                    EvalLogicalNot {
                                        expr: EvalRef(
                                            EvalCall {
                                                func_expr: EvalRef(
                                                    EvalGetAttr {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "small_range",
                                                            },
                                                        ),
                                                        attr_name: "op_contains",
                                                    },
                                                ),
                                                args: [
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