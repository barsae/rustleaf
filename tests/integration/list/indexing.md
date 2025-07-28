# Program
Status: 🟢
Assertions: 2

```rustleaf
// List indexing and operations
var my_list = [1, 2, 3];
assert(my_list[0] == 1);
assert(my_list[2] == 3);
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
        Token(Ident, "my_list"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "my_list"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "my_list"),
        Token(LeftBracket),
        Token(Int, "2"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "3"),
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
                    "my_list",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "my_list",
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
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
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
    EvalProgram {
        statements: [
            EvalDeclare {
                name: "my_list",
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalGetItem {
                                obj_expr: EvalVariable {
                                    name: "my_list",
                                },
                                index_expr: EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    1,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalGetItem {
                                obj_expr: EvalVariable {
                                    name: "my_list",
                                },
                                index_expr: EvalLiteral {
                                    value: Int(
                                        2,
                                    ),
                                },
                            },
                            attr_name: "op_eq",
                        },
                        args: [
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
)
```