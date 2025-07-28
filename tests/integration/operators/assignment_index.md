# Program
Status: 🟢
Assertions: 1

```rustleaf
var arr = [1, 2, 3];
arr[0] = 99;
assert(arr == [99, 2, 3]);
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
        Token(Ident, "arr"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "arr"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(Equal),
        Token(Int, "99"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "arr"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "99"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
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
                    "arr",
                ),
                value: Some(
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
                        ],
                    ),
                ),
            },
            Assignment {
                target: GetItem(
                    Identifier(
                        "arr",
                    ),
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
                op: Assign,
                value: Literal(
                    Int(
                        99,
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
                                "arr",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            99,
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
                                ],
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
    EvalProgram {
        statements: [
            EvalDeclare {
                name: "arr",
                init_expr: Some(
                    EvalList {
                        elements: [
                            EvalLiteral {
                                value: Int(
                                    1,
                                ),
                            },
                            EvalLiteral {
                                value: Int(
                                    2,
                                ),
                            },
                            EvalLiteral {
                                value: Int(
                                    3,
                                ),
                            },
                        ],
                    },
                ),
            },
            EvalSetItem {
                obj_expr: EvalVariable {
                    name: "arr",
                },
                index_expr: EvalLiteral {
                    value: Int(
                        0,
                    ),
                },
                value_expr: EvalLiteral {
                    value: Int(
                        99,
                    ),
                },
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "arr",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalList {
                                elements: [
                                    EvalLiteral {
                                        value: Int(
                                            99,
                                        ),
                                    },
                                    EvalLiteral {
                                        value: Int(
                                            2,
                                        ),
                                    },
                                    EvalLiteral {
                                        value: Int(
                                            3,
                                        ),
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        ],
    },
)
```