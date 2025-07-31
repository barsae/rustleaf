# Program
Status: 🟢
Assertions: 6

```rustleaf
// Test is_unit with various values
assert(not is_unit(42));
assert(not is_unit("hello"));
assert(not is_unit(true));
assert(not is_unit([]));
assert(not is_unit({}));

// Test with a function that returns Unit
fn side_effect() {
    var x = 1;
}
assert(is_unit(side_effect()));
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Ident(assert))
parse_statement: starting at position 0 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 0 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 2 (Not)
parse_primary: success - parsed identifier (is_unit)
parse_expression: starting at position 5 (Int(42))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 9 (Ident(assert))
parse_statement: starting at position 9 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 9 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 11 (Not)
parse_primary: success - parsed identifier (is_unit)
parse_expression: starting at position 14 (String(hello))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 18 (Ident(assert))
parse_statement: starting at position 18 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 20 (Not)
parse_primary: success - parsed identifier (is_unit)
parse_expression: starting at position 23 (True)
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 27 (Ident(assert))
parse_statement: starting at position 27 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 27 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 29 (Not)
parse_primary: success - parsed identifier (is_unit)
parse_expression: starting at position 32 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 37 (Ident(assert))
parse_statement: starting at position 37 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 37 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 39 (Not)
parse_primary: success - parsed identifier (is_unit)
parse_expression: starting at position 42 (LeftBrace)
parse_primary: success - parsing block or dict
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 47 (Fn)
parse_statement: starting at position 47 (Fn)
parse_statement: starting at position 52 (Var)
parse_expression: starting at position 55 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 58 (Ident(assert))
parse_statement: starting at position 58 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 58 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 60 (Ident(is_unit))
parse_primary: success - parsed identifier (is_unit)
parse_expression: starting at position 62 (Ident(side_effect))
parse_primary: success - parsed identifier (side_effect)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Int, "42"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(True),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(LeftBracket),
        Token(RightBracket),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "side_effect"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "is_unit"),
        Token(LeftParen),
        Token(Ident, "side_effect"),
        Token(LeftParen),
        Token(RightParen),
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
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
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        String(
                                            "hello",
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        Bool(
                                            true,
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Dict(
                                        [],
                                    ),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "side_effect",
                params: [],
                body: Block {
                    statements: [
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
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
                                FunctionCall(
                                    Identifier(
                                        "side_effect",
                                    ),
                                    [],
                                ),
                            ],
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
                                                EvalVariable {
                                                    name: "is_unit",
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
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalLiteral {
                                                        value: String(
                                                            "hello",
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
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Bool(
                                                            true,
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
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalList {
                                                        elements: [],
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
                                                EvalVariable {
                                                    name: "is_unit",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalDict {
                                                        pairs: [],
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
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "side_effect",
                            params: [],
                            body: RustValue(
                                EvalBlock {
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
                                    ],
                                    final_expr: None,
                                },
                            ),
                        },
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
                                        EvalVariable {
                                            name: "is_unit",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "side_effect",
                                                    },
                                                ),
                                                args: [],
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