# Program
Status: 🟢
Assertions: 10

```rustleaf
// Basic comparison tests
assert(3 == 3);
assert(3 != 4);
assert(3 < 4);
assert(4 > 3);
assert(3 <= 3);
assert(3 >= 3);

// Mixed numeric comparisons
assert(3 == 3.0);
assert(3.5 > 3);

// String comparisons
assert("a" < "b");
assert("hello" == "hello");
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
parse_expression: starting at position 2 (Int(3))
consume_token: position 2 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 3 consumed EqualEqual
consume_token: position 4 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed Semicolon
parse_program: parsing statement at position 7 (Ident(assert))
parse_statement: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 8 consumed LeftParen
parse_expression: starting at position 9 (Int(3))
consume_token: position 9 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 10 consumed BangEqual
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_program: parsing statement at position 14 (Ident(assert))
parse_statement: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 15 consumed LeftParen
parse_expression: starting at position 16 (Int(3))
consume_token: position 16 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 17 consumed Less
consume_token: position 18 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
parse_program: parsing statement at position 21 (Ident(assert))
parse_statement: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (Int(4))
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 24 consumed Greater
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Semicolon
parse_program: parsing statement at position 28 (Ident(assert))
parse_statement: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Int(3))
consume_token: position 30 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 31 consumed LessEqual
consume_token: position 32 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed Semicolon
parse_program: parsing statement at position 35 (Ident(assert))
parse_statement: starting at position 35 (Ident(assert))
consume_token: position 35 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 35 (Ident(assert))
consume_token: position 35 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 36 consumed LeftParen
parse_expression: starting at position 37 (Int(3))
consume_token: position 37 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 38 consumed GreaterEqual
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 41 consumed Semicolon
parse_program: parsing statement at position 42 (Ident(assert))
parse_statement: starting at position 42 (Ident(assert))
consume_token: position 42 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 42 (Ident(assert))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 43 consumed LeftParen
parse_expression: starting at position 44 (Int(3))
consume_token: position 44 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 45 consumed EqualEqual
consume_token: position 46 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed Semicolon
parse_program: parsing statement at position 49 (Ident(assert))
parse_statement: starting at position 49 (Ident(assert))
consume_token: position 49 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 49 (Ident(assert))
consume_token: position 49 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 50 consumed LeftParen
parse_expression: starting at position 51 (Float(3.5))
consume_token: position 51 consumed Float
parse_primary: success - parsed numeric/string literal
consume_token: position 52 consumed Greater
consume_token: position 53 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed Semicolon
parse_program: parsing statement at position 56 (Ident(assert))
parse_statement: starting at position 56 (Ident(assert))
consume_token: position 56 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 56 (Ident(assert))
consume_token: position 56 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 57 consumed LeftParen
parse_expression: starting at position 58 (String(a))
consume_token: position 58 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 59 consumed Less
consume_token: position 60 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 61 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Semicolon
parse_program: parsing statement at position 63 (Ident(assert))
parse_statement: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 64 consumed LeftParen
parse_expression: starting at position 65 (String(hello))
consume_token: position 65 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 66 consumed EqualEqual
consume_token: position 67 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 68 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Semicolon
parse_program: parsed 10 statements
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
        2: Token(Int, "3"),
        3: Token(EqualEqual),
        4: Token(Int, "3"),
        5: Token(RightParen),
        6: Token(Semicolon),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Int, "3"),
        10: Token(BangEqual),
        11: Token(Int, "4"),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Int, "3"),
        17: Token(Less),
        18: Token(Int, "4"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Int, "4"),
        24: Token(Greater),
        25: Token(Int, "3"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Int, "3"),
        31: Token(LessEqual),
        32: Token(Int, "3"),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Int, "3"),
        38: Token(GreaterEqual),
        39: Token(Int, "3"),
        40: Token(RightParen),
        41: Token(Semicolon),
        42: Token(Ident, "assert"),
        43: Token(LeftParen),
        44: Token(Int, "3"),
        45: Token(EqualEqual),
        46: Token(Float, "3.0"),
        47: Token(RightParen),
        48: Token(Semicolon),
        49: Token(Ident, "assert"),
        50: Token(LeftParen),
        51: Token(Float, "3.5"),
        52: Token(Greater),
        53: Token(Int, "3"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Ident, "assert"),
        57: Token(LeftParen),
        58: Token(String, "a"),
        59: Token(Less),
        60: Token(String, "b"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(String, "hello"),
        66: Token(EqualEqual),
        67: Token(String, "hello"),
        68: Token(RightParen),
        69: Token(Semicolon),
        70: Token(Eof)
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
                            Literal(
                                Int(
                                    3,
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
                        Ne(
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Int(
                                    4,
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
                        Lt(
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Int(
                                    4,
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
                        Gt(
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
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Le(
                            Literal(
                                Int(
                                    3,
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
                        Ge(
                            Literal(
                                Int(
                                    3,
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
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Float(
                                    3.0,
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
                        Gt(
                            Literal(
                                Float(
                                    3.5,
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
                        Lt(
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Literal(
                                String(
                                    "b",
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
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello",
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
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
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
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_ne",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    4,
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
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_lt",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    4,
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
                                                EvalLiteral {
                                                    value: Int(
                                                        4,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_gt",
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
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_le",
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
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_ge",
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
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Float(
                                                    3.0,
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
                                                EvalLiteral {
                                                    value: Float(
                                                        3.5,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_gt",
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
                                                EvalLiteral {
                                                    value: String(
                                                        "a",
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_lt",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "b",
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
                                                EvalLiteral {
                                                    value: String(
                                                        "hello",
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                        ],
                    },
                ),
            ],
        },
    ),
)
```