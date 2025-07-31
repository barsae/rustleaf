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
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_expression: starting at position 3
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 5
parse_statement: starting at position 5
parse_expression: starting at position 5
parse_expression: starting at position 8
parse_expression: success
parse_statement: starting at position 12
parse_expression: starting at position 14
parse_expression: success
parse_statement: parsed assignment
parse_expression: success
parse_statement: parsed block-like expression statement
parse_program: parsing statement at position 17
parse_statement: starting at position 17
parse_statement: falling back to expression statement
parse_expression: starting at position 17
parse_expression: starting at position 19
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 24
parse_statement: starting at position 24
parse_expression: starting at position 27
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 31
parse_statement: starting at position 31
parse_expression: starting at position 34
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 36
parse_statement: starting at position 36
parse_expression: starting at position 36
parse_expression: starting at position 39
parse_expression: success
parse_statement: starting at position 41
parse_expression: starting at position 41
parse_expression: starting at position 42
parse_expression: success
parse_statement: starting at position 48
parse_expression: starting at position 50
parse_expression: success
parse_statement: parsed assignment
parse_expression: success
parse_statement: parsed block-like expression statement
parse_expression: success
parse_statement: parsed block-like expression statement
parse_program: parsing statement at position 54
parse_statement: starting at position 54
parse_statement: falling back to expression statement
parse_expression: starting at position 54
parse_expression: starting at position 56
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 61
parse_statement: starting at position 61
parse_expression: starting at position 64
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 68
parse_statement: starting at position 68
parse_statement: falling back to expression statement
parse_expression: starting at position 68
parse_expression: starting at position 70
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 75
parse_statement: starting at position 75
parse_statement: falling back to expression statement
parse_expression: starting at position 75
parse_expression: starting at position 77
parse_expression: starting at position 79
parse_expression: success
parse_expression: success
parse_expression: success
parse_program: parsed 10 statements
```

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
                        statements: [
                            Expression(
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalDeclare {
                        name: "sum",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalFor {
                        var_name: "i",
                        iter_expr: RustValue(
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
                        body: RustValue(
                            EvalBlock {
                                statements: [
                                    RustValue(
                                        EvalAssign {
                                            name: "sum",
                                            expr: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalVariable {
                                                                    name: "sum",
                                                                },
                                                            ),
                                                            attr_name: "op_add",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
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
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "sum",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
                RustValue(
                    EvalDeclare {
                        name: "range",
                        init_expr: Some(
                            RustValue(
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
                RustValue(
                    EvalDeclare {
                        name: "even_count",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalFor {
                        var_name: "x",
                        iter_expr: RustValue(
                            EvalVariable {
                                name: "range",
                            },
                        ),
                        body: RustValue(
                            EvalBlock {
                                statements: [
                                    RustValue(
                                        EvalIf {
                                            condition: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "x",
                                                                                },
                                                                            ),
                                                                            attr_name: "op_mod",
                                                                        },
                                                                    ),
                                                                    args: [
                                                                        RustValue(
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
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            then_expr: RustValue(
                                                EvalBlock {
                                                    statements: [
                                                        RustValue(
                                                            EvalAssign {
                                                                name: "even_count",
                                                                expr: RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalGetAttr {
                                                                                obj_expr: RustValue(
                                                                                    EvalVariable {
                                                                                        name: "even_count",
                                                                                    },
                                                                                ),
                                                                                attr_name: "op_add",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
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
                                ],
                                final_expr: None,
                            },
                        ),
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "even_count",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
                RustValue(
                    EvalDeclare {
                        name: "small_range",
                        init_expr: Some(
                            RustValue(
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
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "small_range",
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalLogicalNot {
                                    expr: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "small_range",
                                                        },
                                                    ),
                                                    attr_name: "op_contains",
                                                },
                                            ),
                                            args: [
                                                RustValue(
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
)
```