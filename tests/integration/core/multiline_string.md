# Program
Status: 🟢
Assertions: 3

```rustleaf
var multiline = "This is a
multiline string
with multiple lines";
assert(multiline != "single line");
assert("multiline" in multiline);
assert("This is a" in multiline);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
parse_expression: starting at position 3 (String(This is a
multiline string
with multiple lines))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Ident(assert))
parse_statement: starting at position 5 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 5 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 7 (Ident(multiline))
parse_primary: success - parsed identifier (multiline)
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 12 (Ident(assert))
parse_statement: starting at position 12 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 12 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 14 (String(multiline))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (multiline)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 19 (Ident(assert))
parse_statement: starting at position 19 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 21 (String(This is a))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (multiline)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsed 4 statements
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
        Token(Ident, "multiline"),
        Token(Equal),
        Token(String, "This is a\nmultiline string\nwith multiple lines"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiline"),
        Token(BangEqual),
        Token(String, "single line"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "multiline"),
        Token(In),
        Token(Ident, "multiline"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "This is a"),
        Token(In),
        Token(Ident, "multiline"),
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
                    "multiline",
                ),
                value: Some(
                    Literal(
                        String(
                            "This is a\nmultiline string\nwith multiple lines",
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
                        Ne(
                            Identifier(
                                "multiline",
                            ),
                            Literal(
                                String(
                                    "single line",
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
                        In(
                            Literal(
                                String(
                                    "multiline",
                                ),
                            ),
                            Identifier(
                                "multiline",
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
                                    "This is a",
                                ),
                            ),
                            Identifier(
                                "multiline",
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
                        name: "multiline",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "This is a\nmultiline string\nwith multiple lines",
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
                                                    name: "multiline",
                                                },
                                            ),
                                            attr_name: "op_ne",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "single line",
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
                                                    name: "multiline",
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "multiline",
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
                                                    name: "multiline",
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "This is a",
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