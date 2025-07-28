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
    RustValueRef(
        RefCell {
            value: EvalProgram {
                statements: [
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "x",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "inner_x",
                                init_expr: None,
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "outer_x",
                                init_expr: None,
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalBlock {
                                statements: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalDeclare {
                                                name: "x",
                                                init_expr: Some(
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    2,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                    RustValueRef(
                                        RefCell {
                                            value: EvalAssign {
                                                name: "inner_x",
                                                expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalVariable {
                                                            name: "x",
                                                        },
                                                    },
                                                ),
                                            },
                                        },
                                    ),
                                ],
                                final_expr: None,
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalAssign {
                                name: "outer_x",
                                expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "x",
                                        },
                                    },
                                ),
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "inner_x",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    2,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                            },
                                        },
                                    ),
                                ],
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "outer_x",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                            },
                                        },
                                    ),
                                ],
                            },
                        },
                    ),
                ],
            },
        },
    ),
)
```