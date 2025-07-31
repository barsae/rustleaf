# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = 1;
var inner_x;
var outer_x;
{
    var x = 2;
    inner_x = x;
}
outer_x = x;
assert(inner_x == 2);
assert(outer_x == 1);
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
parse_statement: parsed var declaration
parse_program: parsing statement at position 8
parse_statement: starting at position 8
parse_statement: parsed var declaration
parse_program: parsing statement at position 11
parse_statement: starting at position 11
parse_expression: starting at position 11
parse_statement: starting at position 12
parse_expression: starting at position 15
parse_expression: success
parse_statement: parsed var declaration
parse_statement: starting at position 17
parse_expression: starting at position 19
parse_expression: success
parse_statement: parsed assignment
parse_expression: success
parse_statement: parsed block-like expression statement
parse_program: parsing statement at position 22
parse_statement: starting at position 22
parse_expression: starting at position 24
parse_expression: success
parse_statement: parsed assignment
parse_program: parsing statement at position 26
parse_statement: starting at position 26
parse_statement: falling back to expression statement
parse_expression: starting at position 26
parse_expression: starting at position 28
parse_expression: success
parse_expression: success
parse_program: parsing statement at position 33
parse_statement: starting at position 33
parse_statement: falling back to expression statement
parse_expression: starting at position 33
parse_expression: starting at position 35
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "inner_x"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "outer_x"),
        Token(Semicolon),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Ident, "inner_x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "outer_x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "inner_x"),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "outer_x"),
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
                    "inner_x",
                ),
                value: None,
            },
            VarDecl {
                pattern: Variable(
                    "outer_x",
                ),
                value: None,
            },
            Expression(
                Block(
                    Block {
                        statements: [
                            VarDecl {
                                pattern: Variable(
                                    "x",
                                ),
                                value: Some(
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ),
                            },
                            Assignment {
                                target: Identifier(
                                    "inner_x",
                                ),
                                op: Assign,
                                value: Identifier(
                                    "x",
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                ),
            ),
            Assignment {
                target: Identifier(
                    "outer_x",
                ),
                op: Assign,
                value: Identifier(
                    "x",
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
                                "inner_x",
                            ),
                            Literal(
                                Int(
                                    2,
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
                                "outer_x",
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalDeclare {
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "inner_x",
                        init_expr: None,
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "outer_x",
                        init_expr: None,
                    },
                ),
                RustValue(
                    EvalBlock {
                        statements: [
                            RustValue(
                                EvalDeclare {
                                    name: "x",
                                    init_expr: Some(
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    2,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            RustValue(
                                EvalAssign {
                                    name: "inner_x",
                                    expr: RustValue(
                                        EvalVariable {
                                            name: "x",
                                        },
                                    ),
                                },
                            ),
                        ],
                        final_expr: None,
                    },
                ),
                RustValue(
                    EvalAssign {
                        name: "outer_x",
                        expr: RustValue(
                            EvalVariable {
                                name: "x",
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
                                                    name: "inner_x",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                                    name: "outer_x",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                        ],
                    },
                ),
            ],
        },
    ),
)
```