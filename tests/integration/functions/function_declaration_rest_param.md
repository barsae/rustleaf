# Program
Status: 🟢
Assertions: 10

```rustleaf
fn sum(*args) { args }
fn first_and_rest(first, *rest) { [first, rest] }

var empty = sum();
var single = sum(1);
var multiple = sum(1, 2, 3, 4, 5);

var mixed1 = first_and_rest(42);
var mixed2 = first_and_rest(10, 20, 30);

// Test that rest parameters collect into lists
assert(empty == []);
assert(single == [1]);
assert(multiple == [1, 2, 3, 4, 5]);

// Test mixed regular and rest parameters
assert(mixed1 == [42, []]);
assert(mixed2 == [10, [20, 30]]);

// Test individual access
assert(single[0] == 1);
assert(multiple[0] == 1);
assert(multiple[4] == 5);
assert(mixed1[0] == 42);
assert(mixed2[0] == 10);
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
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(Star),
        Token(Ident, "args"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "args"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "first_and_rest"),
        Token(LeftParen),
        Token(Ident, "first"),
        Token(Comma),
        Token(Star),
        Token(Ident, "rest"),
        Token(RightParen),
        Token(LeftBrace),
        Token(LeftBracket),
        Token(Ident, "first"),
        Token(Comma),
        Token(Ident, "rest"),
        Token(RightBracket),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "empty"),
        Token(Equal),
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "single"),
        Token(Equal),
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "multiple"),
        Token(Equal),
        Token(Ident, "sum"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "mixed1"),
        Token(Equal),
        Token(Ident, "first_and_rest"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "mixed2"),
        Token(Equal),
        Token(Ident, "first_and_rest"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Comma),
        Token(Int, "20"),
        Token(Comma),
        Token(Int, "30"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "empty"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "single"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiple"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "5"),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed1"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "42"),
        Token(Comma),
        Token(LeftBracket),
        Token(RightBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed2"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "10"),
        Token(Comma),
        Token(LeftBracket),
        Token(Int, "20"),
        Token(Comma),
        Token(Int, "30"),
        Token(RightBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "single"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiple"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiple"),
        Token(LeftBracket),
        Token(Int, "4"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed1"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed2"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "10"),
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
                name: "sum",
                params: [
                    Parameter {
                        name: "args",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "args",
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "first_and_rest",
                params: [
                    Parameter {
                        name: "first",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "rest",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        List(
                            [
                                Identifier(
                                    "first",
                                ),
                                Identifier(
                                    "rest",
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "empty",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "single",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
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
            },
            VarDecl {
                pattern: Variable(
                    "multiple",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "mixed1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "mixed2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                            Literal(
                                Int(
                                    20,
                                ),
                            ),
                            Literal(
                                Int(
                                    30,
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
                                "empty",
                            ),
                            List(
                                [],
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
                                "single",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ],
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
                                "multiple",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
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
                                "mixed1",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            42,
                                        ),
                                    ),
                                    List(
                                        [],
                                    ),
                                ],
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
                                "mixed2",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    List(
                                        [
                                            Literal(
                                                Int(
                                                    20,
                                                ),
                                            ),
                                            Literal(
                                                Int(
                                                    30,
                                                ),
                                            ),
                                        ],
                                    ),
                                ],
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
                            GetItem(
                                Identifier(
                                    "single",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
                            GetItem(
                                Identifier(
                                    "multiple",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
                            GetItem(
                                Identifier(
                                    "multiple",
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "mixed1",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
                            GetItem(
                                Identifier(
                                    "mixed2",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
                                name: "sum",
                                params: [
                                    (
                                        "args",
                                        None,
                                        Rest,
                                    ),
                                ],
                                body: Eval(
                                    EvalRef(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                EvalRef(
                                                    EvalVariable {
                                                        name: "args",
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
                                name: "first_and_rest",
                                params: [
                                    (
                                        "first",
                                        None,
                                        Regular,
                                    ),
                                    (
                                        "rest",
                                        None,
                                        Rest,
                                    ),
                                ],
                                body: Eval(
                                    EvalRef(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                EvalRef(
                                                    EvalList {
                                                        elements: [
                                                            EvalRef(
                                                                EvalVariable {
                                                                    name: "first",
                                                                },
                                                            ),
                                                            EvalRef(
                                                                EvalVariable {
                                                                    name: "rest",
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
                            name: "empty",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "sum",
                                            },
                                        ),
                                        args: [],
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "single",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "sum",
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
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "multiple",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "sum",
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
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        2,
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
                                                        5,
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
                            name: "mixed1",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "first_and_rest",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        42,
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
                            name: "mixed2",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "first_and_rest",
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
                                                EvalLiteral {
                                                    value: Int(
                                                        20,
                                                    ),
                                                },
                                            ),
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        30,
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
                                                        name: "empty",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalList {
                                                    elements: [],
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
                                                        name: "single",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalList {
                                                    elements: [
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
                                                        name: "multiple",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalList {
                                                    elements: [
                                                        EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                        EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    2,
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
                                                        name: "mixed1",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalList {
                                                    elements: [
                                                        EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    42,
                                                                ),
                                                            },
                                                        ),
                                                        EvalRef(
                                                            EvalList {
                                                                elements: [],
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
                                                        name: "mixed2",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalList {
                                                    elements: [
                                                        EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    10,
                                                                ),
                                                            },
                                                        ),
                                                        EvalRef(
                                                            EvalList {
                                                                elements: [
                                                                    EvalRef(
                                                                        EvalLiteral {
                                                                            value: Int(
                                                                                20,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    EvalRef(
                                                                        EvalLiteral {
                                                                            value: Int(
                                                                                30,
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "single",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_eq",
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "multiple",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_eq",
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "multiple",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    4,
                                                                ),
                                                            },
                                                        ),
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "mixed1",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        42,
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "mixed2",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
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
                ],
            },
        ),
    ),
)
```