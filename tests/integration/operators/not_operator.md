# Program
Status: 🟢
Assertions: 4

```rustleaf
// Test 'not' as unary operator
assert(not true == false);
assert(not false == true);

// Test with expressions
var x = 5;
assert(not (x > 10) == true);
assert(not (x < 3) == true);   // x=5, x<3 is false, not false is true
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(True),
        Token(EqualEqual),
        Token(False),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(False),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "10"),
        Token(RightParen),
        Token(EqualEqual),
        Token(True),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(True),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Not(
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    false,
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
                            Not(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Not(
                                Gt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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
                            Not(
                                Lt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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
                                                                    value: EvalLogicalNot {
                                                                        expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalLiteral {
                                                                                    value: Bool(
                                                                                        true,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        ),
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
                                                                value: Bool(
                                                                    false,
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
                                                                    value: EvalLogicalNot {
                                                                        expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalLiteral {
                                                                                    value: Bool(
                                                                                        false,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        ),
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
                                                                value: Bool(
                                                                    true,
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
                            value: EvalDeclare {
                                name: "x",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: Int(
                                                    5,
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
                                                                    value: EvalLogicalNot {
                                                                        expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalCall {
                                                                                    func_expr: RustValueRef(
                                                                                        RefCell {
                                                                                            value: EvalGetAttr {
                                                                                                obj_expr: RustValueRef(
                                                                                                    RefCell {
                                                                                                        value: EvalVariable {
                                                                                                            name: "x",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                attr_name: "op_gt",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    args: [
                                                                                        RustValueRef(
                                                                                            RefCell {
                                                                                                value: EvalLiteral {
                                                                                                    value: Int(
                                                                                                        10,
                                                                                                    ),
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            },
                                                                        ),
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
                                                                value: Bool(
                                                                    true,
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
                                                                    value: EvalLogicalNot {
                                                                        expr: RustValueRef(
                                                                            RefCell {
                                                                                value: EvalCall {
                                                                                    func_expr: RustValueRef(
                                                                                        RefCell {
                                                                                            value: EvalGetAttr {
                                                                                                obj_expr: RustValueRef(
                                                                                                    RefCell {
                                                                                                        value: EvalVariable {
                                                                                                            name: "x",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                attr_name: "op_lt",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    args: [
                                                                                        RustValueRef(
                                                                                            RefCell {
                                                                                                value: EvalLiteral {
                                                                                                    value: Int(
                                                                                                        3,
                                                                                                    ),
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            },
                                                                        ),
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
                                                                value: Bool(
                                                                    true,
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