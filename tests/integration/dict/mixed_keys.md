# Program
Status: 🟢
Assertions: 3

```rustleaf
// Dict access with different key types
var my_dict = {"a": 1, "b": 2};
assert(my_dict["a"] == 1);

// Mixed key types
var mixed = {1: "one", "two": 2, true: "yes"};
assert(mixed[1] == "one");
assert(mixed["two"] == 2);
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
        Token(Ident, "my_dict"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "a"),
        Token(Colon),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "b"),
        Token(Colon),
        Token(Int, "2"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "my_dict"),
        Token(LeftBracket),
        Token(String, "a"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "mixed"),
        Token(Equal),
        Token(LeftBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(String, "one"),
        Token(Comma),
        Token(String, "two"),
        Token(Colon),
        Token(Int, "2"),
        Token(Comma),
        Token(True),
        Token(Colon),
        Token(String, "yes"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed"),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(String, "one"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed"),
        Token(LeftBracket),
        Token(String, "two"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "2"),
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
                    "my_dict",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "b",
                                    ),
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
                                    "my_dict",
                                ),
                                Literal(
                                    String(
                                        "a",
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
            VarDecl {
                pattern: Variable(
                    "mixed",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "one",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "two",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "yes",
                                    ),
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
                                    "mixed",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            Literal(
                                String(
                                    "one",
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
                                    "mixed",
                                ),
                                Literal(
                                    String(
                                        "two",
                                    ),
                                ),
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
                            name: "my_dict",
                            init_expr: Some(
                                EvalRef(
                                    EvalDict {
                                        pairs: [
                                            (
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "a",
                                                        ),
                                                    },
                                                ),
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: Int(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            (
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "b",
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "my_dict",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: String(
                                                                    "a",
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
                        EvalDeclare {
                            name: "mixed",
                            init_expr: Some(
                                EvalRef(
                                    EvalDict {
                                        pairs: [
                                            (
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: Int(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "one",
                                                        ),
                                                    },
                                                ),
                                            ),
                                            (
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "two",
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
                                            ),
                                            (
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: Bool(
                                                            true,
                                                        ),
                                                    },
                                                ),
                                                EvalRef(
                                                    EvalLiteral {
                                                        value: String(
                                                            "yes",
                                                        ),
                                                    },
                                                ),
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
                                                    EvalGetItem {
                                                        obj_expr: EvalRef(
                                                            EvalVariable {
                                                                name: "mixed",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    1,
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
                                                    value: String(
                                                        "one",
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
                                                                name: "mixed",
                                                            },
                                                        ),
                                                        index_expr: EvalRef(
                                                            EvalLiteral {
                                                                value: String(
                                                                    "two",
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
                ],
            },
        ),
    ),
)
```