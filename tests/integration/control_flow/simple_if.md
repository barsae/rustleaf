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
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_expression: starting at position 3
parse_expression: starting at position 4
parse_expression: success
parse_statement: starting at position 6
parse_statement: falling back to expression statement
parse_expression: starting at position 6
parse_expression: success
parse_expression: starting at position 6
parse_expression: success
parse_statement: starting at position 10
parse_statement: falling back to expression statement
parse_expression: starting at position 10
parse_expression: success
parse_expression: starting at position 10
parse_expression: success
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 13
parse_statement: starting at position 13
parse_expression: starting at position 16
parse_expression: starting at position 17
parse_expression: success
parse_statement: starting at position 19
parse_statement: falling back to expression statement
parse_expression: starting at position 19
parse_expression: success
parse_expression: starting at position 19
parse_expression: success
parse_statement: starting at position 23
parse_statement: falling back to expression statement
parse_expression: starting at position 23
parse_expression: success
parse_expression: starting at position 23
parse_expression: success
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 26
parse_statement: starting at position 26
parse_expression: starting at position 29
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 31
parse_statement: starting at position 31
parse_expression: starting at position 34
parse_expression: starting at position 35
parse_expression: success
parse_statement: starting at position 39
parse_statement: falling back to expression statement
parse_expression: starting at position 39
parse_expression: success
parse_expression: starting at position 39
parse_expression: success
parse_statement: starting at position 43
parse_statement: falling back to expression statement
parse_expression: starting at position 43
parse_expression: success
parse_expression: starting at position 43
parse_expression: success
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 46
parse_statement: starting at position 46
parse_statement: falling back to expression statement
parse_expression: starting at position 46
parse_expression: starting at position 48
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 53
parse_statement: starting at position 53
parse_statement: falling back to expression statement
parse_expression: starting at position 53
parse_expression: starting at position 55
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 60
parse_statement: starting at position 60
parse_statement: falling back to expression statement
parse_expression: starting at position 60
parse_expression: starting at position 62
parse_expression: success
parse_expression: success
parse_program: parsed 7 statements
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalDeclare {
                        name: "result1",
                        init_expr: Some(
                            RustValue(
                                EvalIf {
                                    condition: RustValue(
                                        EvalLiteral {
                                            value: Bool(
                                                true,
                                            ),
                                        },
                                    ),
                                    then_expr: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Int(
                                                            42,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    else_expr: Some(
                                        RustValue(
                                            EvalBlock {
                                                statements: [],
                                                final_expr: Some(
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
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result2",
                        init_expr: Some(
                            RustValue(
                                EvalIf {
                                    condition: RustValue(
                                        EvalLiteral {
                                            value: Bool(
                                                false,
                                            ),
                                        },
                                    ),
                                    then_expr: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Int(
                                                            100,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    else_expr: Some(
                                        RustValue(
                                            EvalBlock {
                                                statements: [],
                                                final_expr: Some(
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                200,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        5,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result3",
                        init_expr: Some(
                            RustValue(
                                EvalIf {
                                    condition: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "x",
                                                        },
                                                    ),
                                                    attr_name: "op_gt",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Int(
                                                            3,
                                                        ),
                                                    },
                                                ),
                                            ],
                                        },
                                    ),
                                    then_expr: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalLiteral {
                                                        value: String(
                                                            "big",
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    else_expr: Some(
                                        RustValue(
                                            EvalBlock {
                                                statements: [],
                                                final_expr: Some(
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: String(
                                                                "small",
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
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
                                                    name: "result1",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
                                                    name: "result2",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    200,
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
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "result3",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "big",
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
)
```