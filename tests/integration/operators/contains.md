# Program
Status: 🟢
Assertions: 9

```rustleaf
// Test 'in' operator (op_contains)

// String contains
assert("hello" in "hello world");
assert(not ("xyz" in "hello world"));

// List contains
var my_list = [1, 2, 3, "hello"];
assert(2 in my_list);
assert("hello" in my_list);
assert(not (99 in my_list));

// Dict contains (check keys)
var my_dict = {"a": 1, "b": 2, 3: "three"};
assert("a" in my_dict);
assert("b" in my_dict);
assert(3 in my_dict);
assert(not ("z" in my_dict));
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
        Token(String, "hello"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "xyz"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "my_list"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(String, "hello"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "99"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
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
        Token(Comma),
        Token(Int, "3"),
        Token(Colon),
        Token(String, "three"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "a"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "b"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "z"),
        Token(In),
        Token(Ident, "my_dict"),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        In(
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello world",
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
                        Not(
                            In(
                                Literal(
                                    String(
                                        "xyz",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "hello world",
                                    ),
                                ),
                            ),
                        ),
                    ],
                ),
            ),
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
                            Literal(
                                String(
                                    "hello",
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
                        In(
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                        In(
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                                        99,
                                    ),
                                ),
                                Identifier(
                                    "my_list",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
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
                            (
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "three",
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
                        In(
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                        In(
                            Literal(
                                String(
                                    "b",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                        In(
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                                    String(
                                        "z",
                                    ),
                                ),
                                Identifier(
                                    "my_dict",
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalLiteral {
                                value: String(
                                    "hello world",
                                ),
                            },
                            attr_name: "op_contains",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "hello",
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
                    EvalLogicalNot {
                        expr: EvalCall {
                            func_expr: EvalGetAttr {
                                obj_expr: EvalLiteral {
                                    value: String(
                                        "hello world",
                                    ),
                                },
                                attr_name: "op_contains",
                            },
                            args: [
                                EvalLiteral {
                                    value: String(
                                        "xyz",
                                    ),
                                },
                            ],
                        },
                    },
                ],
            },
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
                            EvalLiteral {
                                value: String(
                                    "hello",
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
                            obj_expr: EvalVariable {
                                name: "my_list",
                            },
                            attr_name: "op_contains",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    2,
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
                            obj_expr: EvalVariable {
                                name: "my_list",
                            },
                            attr_name: "op_contains",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "hello",
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
                    EvalLogicalNot {
                        expr: EvalCall {
                            func_expr: EvalGetAttr {
                                obj_expr: EvalVariable {
                                    name: "my_list",
                                },
                                attr_name: "op_contains",
                            },
                            args: [
                                EvalLiteral {
                                    value: Int(
                                        99,
                                    ),
                                },
                            ],
                        },
                    },
                ],
            },
            EvalDeclare {
                name: "my_dict",
                init_expr: Some(
                    EvalDict {
                        pairs: [
                            (
                                EvalLiteral {
                                    value: String(
                                        "a",
                                    ),
                                },
                                EvalLiteral {
                                    value: Int(
                                        1,
                                    ),
                                },
                            ),
                            (
                                EvalLiteral {
                                    value: String(
                                        "b",
                                    ),
                                },
                                EvalLiteral {
                                    value: Int(
                                        2,
                                    ),
                                },
                            ),
                            (
                                EvalLiteral {
                                    value: Int(
                                        3,
                                    ),
                                },
                                EvalLiteral {
                                    value: String(
                                        "three",
                                    ),
                                },
                            ),
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
                            obj_expr: EvalVariable {
                                name: "my_dict",
                            },
                            attr_name: "op_contains",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "a",
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
                            obj_expr: EvalVariable {
                                name: "my_dict",
                            },
                            attr_name: "op_contains",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "b",
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
                            obj_expr: EvalVariable {
                                name: "my_dict",
                            },
                            attr_name: "op_contains",
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
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalLogicalNot {
                        expr: EvalCall {
                            func_expr: EvalGetAttr {
                                obj_expr: EvalVariable {
                                    name: "my_dict",
                                },
                                attr_name: "op_contains",
                            },
                            args: [
                                EvalLiteral {
                                    value: String(
                                        "z",
                                    ),
                                },
                            ],
                        },
                    },
                ],
            },
        ],
    },
)
```