# Program
Status: 🟢
Assertions: 4

```rustleaf
assert(1 + 2 == 3);
assert(5 - 3 == 2);
assert(4 * 3 == 12);
assert(10 / 2 == 5.0);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Ident(assert))
parse_statement: starting at position 0 (Ident(assert))
consume_token: position 0 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 0 (Ident(assert))
consume_token: position 0 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 1 consumed LeftParen
parse_expression: starting at position 2 (Int(1))
consume_token: position 2 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 3 consumed Plus
consume_token: position 4 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 5 consumed EqualEqual
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed Semicolon
parse_program: parsing statement at position 9 (Ident(assert))
parse_statement: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 10 consumed LeftParen
parse_expression: starting at position 11 (Int(5))
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 12 consumed Minus
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 14 consumed EqualEqual
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
parse_program: parsing statement at position 18 (Ident(assert))
parse_statement: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 19 consumed LeftParen
parse_expression: starting at position 20 (Int(4))
consume_token: position 20 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 21 consumed Star
consume_token: position 22 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 23 consumed EqualEqual
consume_token: position 24 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Semicolon
parse_program: parsing statement at position 27 (Ident(assert))
parse_statement: starting at position 27 (Ident(assert))
consume_token: position 27 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 27 (Ident(assert))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 28 consumed LeftParen
parse_expression: starting at position 29 (Int(10))
consume_token: position 29 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 30 consumed Slash
consume_token: position 31 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 32 consumed EqualEqual
consume_token: position 33 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed Semicolon
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(Int, "1"),
        3: Token(Plus),
        4: Token(Int, "2"),
        5: Token(EqualEqual),
        6: Token(Int, "3"),
        7: Token(RightParen),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Int, "5"),
        12: Token(Minus),
        13: Token(Int, "3"),
        14: Token(EqualEqual),
        15: Token(Int, "2"),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Int, "4"),
        21: Token(Star),
        22: Token(Int, "3"),
        23: Token(EqualEqual),
        24: Token(Int, "12"),
        25: Token(RightParen),
        26: Token(Semicolon),
        27: Token(Ident, "assert"),
        28: Token(LeftParen),
        29: Token(Int, "10"),
        30: Token(Slash),
        31: Token(Int, "2"),
        32: Token(EqualEqual),
        33: Token(Float, "5.0"),
        34: Token(RightParen),
        35: Token(Semicolon),
        36: Token(Eof)
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
                            Add(
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Sub(
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        3,
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Mul(
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    12,
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
                            Div(
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    5.0,
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
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                            attr_name: "op_add",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    3,
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
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            attr_name: "op_sub",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    3,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        4,
                                                                    ),
                                                                },
                                                            ),
                                                            attr_name: "op_mul",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    3,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    12,
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
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalLiteral {
                                                                    value: Int(
                                                                        10,
                                                                    ),
                                                                },
                                                            ),
                                                            attr_name: "op_div",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Float(
                                                    5.0,
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