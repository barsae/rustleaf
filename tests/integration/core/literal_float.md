# Program
Status: 🟢
Assertions: 5

```rustleaf
var pi = 3.14;
var small = 0.1;
var negative = -2.5;
assert(pi == 3.14);
assert(small == 0.1);
assert(negative == -2.5);
assert(pi + small == 3.24);
assert(pi * 2.0 == 6.28);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Float(3.14))
consume_token: position 3 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Float(0.1))
consume_token: position 8 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Var)
parse_statement: starting at position 10 (Var)
consume_token: position 10 consumed Var
consume_token: position 11 consumed Ident
consume_token: position 12 consumed Equal
parse_expression: starting at position 13 (Minus)
consume_token: position 13 consumed Minus
consume_token: position 14 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 16 (Ident(assert))
parse_statement: starting at position 16 (Ident(assert))
consume_token: position 16 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 16 (Ident(assert))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 17 consumed LeftParen
parse_expression: starting at position 18 (Ident(pi))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (pi)
consume_token: position 19 consumed EqualEqual
consume_token: position 20 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed Semicolon
parse_program: parsing statement at position 23 (Ident(assert))
parse_statement: starting at position 23 (Ident(assert))
consume_token: position 23 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 23 (Ident(assert))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 24 consumed LeftParen
parse_expression: starting at position 25 (Ident(small))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (small)
consume_token: position 26 consumed EqualEqual
consume_token: position 27 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 28 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed Semicolon
parse_program: parsing statement at position 30 (Ident(assert))
parse_statement: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 31 consumed LeftParen
parse_expression: starting at position 32 (Ident(negative))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (negative)
consume_token: position 33 consumed EqualEqual
consume_token: position 34 consumed Minus
consume_token: position 35 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 36 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 37 consumed Semicolon
parse_program: parsing statement at position 38 (Ident(assert))
parse_statement: starting at position 38 (Ident(assert))
consume_token: position 38 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 38 (Ident(assert))
consume_token: position 38 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 39 consumed LeftParen
parse_expression: starting at position 40 (Ident(pi))
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (pi)
consume_token: position 41 consumed Plus
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (small)
consume_token: position 43 consumed EqualEqual
consume_token: position 44 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed Semicolon
parse_program: parsing statement at position 47 (Ident(assert))
parse_statement: starting at position 47 (Ident(assert))
consume_token: position 47 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 47 (Ident(assert))
consume_token: position 47 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 48 consumed LeftParen
parse_expression: starting at position 49 (Ident(pi))
consume_token: position 49 consumed Ident
parse_primary: success - parsed identifier (pi)
consume_token: position 50 consumed Star
consume_token: position 51 consumed Float
parse_primary: success - parsed numeric/string literal
consume_token: position 52 consumed EqualEqual
consume_token: position 53 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed Semicolon
parse_program: parsed 8 statements
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
        1: Token(Ident, "pi"),
        2: Token(Equal),
        3: Token(Float, "3.14"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "small"),
        7: Token(Equal),
        8: Token(Float, "0.1"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "negative"),
        12: Token(Equal),
        13: Token(Minus),
        14: Token(Float, "2.5"),
        15: Token(Semicolon),
        16: Token(Ident, "assert"),
        17: Token(LeftParen),
        18: Token(Ident, "pi"),
        19: Token(EqualEqual),
        20: Token(Float, "3.14"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Ident, "assert"),
        24: Token(LeftParen),
        25: Token(Ident, "small"),
        26: Token(EqualEqual),
        27: Token(Float, "0.1"),
        28: Token(RightParen),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "negative"),
        33: Token(EqualEqual),
        34: Token(Minus),
        35: Token(Float, "2.5"),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Ident, "assert"),
        39: Token(LeftParen),
        40: Token(Ident, "pi"),
        41: Token(Plus),
        42: Token(Ident, "small"),
        43: Token(EqualEqual),
        44: Token(Float, "3.24"),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Ident, "assert"),
        48: Token(LeftParen),
        49: Token(Ident, "pi"),
        50: Token(Star),
        51: Token(Float, "2.0"),
        52: Token(EqualEqual),
        53: Token(Float, "6.28"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Eof)
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
                    "pi",
                ),
                value: Some(
                    Literal(
                        Float(
                            3.14,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "small",
                ),
                value: Some(
                    Literal(
                        Float(
                            0.1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "negative",
                ),
                value: Some(
                    Neg(
                        Literal(
                            Float(
                                2.5,
                            ),
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
                        Eq(
                            Identifier(
                                "pi",
                            ),
                            Literal(
                                Float(
                                    3.14,
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
                            Identifier(
                                "small",
                            ),
                            Literal(
                                Float(
                                    0.1,
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
                            Identifier(
                                "negative",
                            ),
                            Neg(
                                Literal(
                                    Float(
                                        2.5,
                                    ),
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
                            Add(
                                Identifier(
                                    "pi",
                                ),
                                Identifier(
                                    "small",
                                ),
                            ),
                            Literal(
                                Float(
                                    3.24,
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
                                Identifier(
                                    "pi",
                                ),
                                Literal(
                                    Float(
                                        2.0,
                                    ),
                                ),
                            ),
                            Literal(
                                Float(
                                    6.28,
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
                        name: "pi",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Float(
                                        3.14,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "small",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Float(
                                        0.1,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "negative",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalLiteral {
                                                    value: Float(
                                                        2.5,
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_neg",
                                        },
                                    ),
                                    args: [],
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
                                                    name: "pi",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Float(
                                                    3.14,
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
                                                    name: "small",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Float(
                                                    0.1,
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
                                                    name: "negative",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                                                    2.5,
                                                                ),
                                                            },
                                                        ),
                                                        attr_name: "op_neg",
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
                                                                EvalVariable {
                                                                    name: "pi",
                                                                },
                                                            ),
                                                            attr_name: "op_add",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalVariable {
                                                                name: "small",
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
                                                    3.24,
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
                                                                EvalVariable {
                                                                    name: "pi",
                                                                },
                                                            ),
                                                            attr_name: "op_mul",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Float(
                                                                    2.0,
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
                                                    6.28,
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