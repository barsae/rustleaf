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
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (String(This is a
multiline string
with multiple lines))
consume_token: position 3 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Ident(assert))
parse_statement: starting at position 5 (Ident(assert))
consume_token: position 5 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 5 (Ident(assert))
consume_token: position 5 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 6 consumed LeftParen
parse_expression: starting at position 7 (Ident(multiline))
consume_token: position 7 consumed Ident
parse_primary: success - parsed identifier (multiline)
consume_token: position 8 consumed BangEqual
consume_token: position 9 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed Semicolon
parse_program: parsing statement at position 12 (Ident(assert))
parse_statement: starting at position 12 (Ident(assert))
consume_token: position 12 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 12 (Ident(assert))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 13 consumed LeftParen
parse_expression: starting at position 14 (String(multiline))
consume_token: position 14 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 15 consumed In
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (multiline)
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed Semicolon
parse_program: parsing statement at position 19 (Ident(assert))
parse_statement: starting at position 19 (Ident(assert))
consume_token: position 19 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Ident(assert))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 20 consumed LeftParen
parse_expression: starting at position 21 (String(This is a))
consume_token: position 21 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 22 consumed In
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (multiline)
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Semicolon
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
        0: Token(Var),
        1: Token(Ident, "multiline"),
        2: Token(Equal),
        3: Token(String, "This is a\nmultiline string\nwith multiple lines"),
        4: Token(Semicolon),
        5: Token(Ident, "assert"),
        6: Token(LeftParen),
        7: Token(Ident, "multiline"),
        8: Token(BangEqual),
        9: Token(String, "single line"),
        10: Token(RightParen),
        11: Token(Semicolon),
        12: Token(Ident, "assert"),
        13: Token(LeftParen),
        14: Token(String, "multiline"),
        15: Token(In),
        16: Token(Ident, "multiline"),
        17: Token(RightParen),
        18: Token(Semicolon),
        19: Token(Ident, "assert"),
        20: Token(LeftParen),
        21: Token(String, "This is a"),
        22: Token(In),
        23: Token(Ident, "multiline"),
        24: Token(RightParen),
        25: Token(Semicolon),
        26: Token(Eof)
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