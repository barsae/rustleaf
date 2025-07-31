# Program
Status: 🟢
Assertions: 1

```rustleaf
var a = 3;
var b = 7;
var result = "${a} and ${b} equals ${a + b}";
assert(result == "3 and 7 equals 10");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(3))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Int(7))
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Var)
parse_statement: starting at position 10 (Var)
consume_token: position 10 consumed Var
consume_token: position 11 consumed Ident
consume_token: position 12 consumed Equal
parse_expression: starting at position 13 (InterpolationStart)
parse_primary: success - parsing interpolated string
consume_token: position 13 consumed InterpolationStart
parse_expression: starting at position 14 (Ident(a))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (a)
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed InterpolationEnd
consume_token: position 16 consumed StringPart
consume_token: position 17 consumed InterpolationStart
parse_expression: starting at position 18 (Ident(b))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (b)
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed InterpolationEnd
consume_token: position 20 consumed StringPart
consume_token: position 21 consumed InterpolationStart
parse_expression: starting at position 22 (Ident(a))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (a)
consume_token: position 23 consumed Plus
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (b)
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed InterpolationEnd
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 27 (Ident(assert))
parse_statement: starting at position 27 (Ident(assert))
consume_token: position 27 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 27 (Ident(assert))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 28 consumed LeftParen
parse_expression: starting at position 29 (Ident(result))
consume_token: position 29 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 30 consumed EqualEqual
consume_token: position 31 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed Semicolon
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
        1: Token(Ident, "a"),
        2: Token(Equal),
        3: Token(Int, "3"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "b"),
        7: Token(Equal),
        8: Token(Int, "7"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "result"),
        12: Token(Equal),
        13: Token(InterpolationStart),
        14: Token(Ident, "a"),
        15: Token(InterpolationEnd),
        16: Token(StringPart, " and "),
        17: Token(InterpolationStart),
        18: Token(Ident, "b"),
        19: Token(InterpolationEnd),
        20: Token(StringPart, " equals "),
        21: Token(InterpolationStart),
        22: Token(Ident, "a"),
        23: Token(Plus),
        24: Token(Ident, "b"),
        25: Token(InterpolationEnd),
        26: Token(Semicolon),
        27: Token(Ident, "assert"),
        28: Token(LeftParen),
        29: Token(Ident, "result"),
        30: Token(EqualEqual),
        31: Token(String, "3 and 7 equals 10"),
        32: Token(RightParen),
        33: Token(Semicolon),
        34: Token(Eof)
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
                    "a",
                ),
                value: Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "b",
                ),
                value: Some(
                    Literal(
                        Int(
                            7,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Expression(
                                Identifier(
                                    "a",
                                ),
                            ),
                            Text(
                                " and ",
                            ),
                            Expression(
                                Identifier(
                                    "b",
                                ),
                            ),
                            Text(
                                " equals ",
                            ),
                            Expression(
                                Add(
                                    Identifier(
                                        "a",
                                    ),
                                    Identifier(
                                        "b",
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
                            Identifier(
                                "result",
                            ),
                            Literal(
                                String(
                                    "3 and 7 equals 10",
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
                        name: "a",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        3,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "b",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        7,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalVariable {
                                                            name: "str",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalVariable {
                                                                name: "a",
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            attr_name: "op_add",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    " and ",
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "str",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalVariable {
                                                            name: "b",
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    " equals ",
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "str",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalCall {
                                                            func_expr: RustValue(
                                                                EvalGetAttr {
                                                                    obj_expr: RustValue(
                                                                        EvalVariable {
                                                                            name: "a",
                                                                        },
                                                                    ),
                                                                    attr_name: "op_add",
                                                                },
                                                            ),
                                                            args: [
                                                                RustValue(
                                                                    EvalVariable {
                                                                        name: "b",
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
                                                    name: "result",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "3 and 7 equals 10",
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