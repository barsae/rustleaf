# Program
Status: 🟢
Assertions: 4

```rustleaf
var num = 123;
var zero = 0;
var negative = -42;
assert(num == 123);
assert(zero == 0);
assert(negative == -42);
assert(num + zero == 123);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(123))
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
parse_expression: starting at position 8 (Int(0))
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
parse_expression: starting at position 13 (Minus)
consume_token: position 13 consumed Minus
consume_token: position 14 consumed Int
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
parse_expression: starting at position 18 (Ident(num))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (num)
consume_token: position 19 consumed EqualEqual
consume_token: position 20 consumed Int
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
parse_expression: starting at position 25 (Ident(zero))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (zero)
consume_token: position 26 consumed EqualEqual
consume_token: position 27 consumed Int
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
consume_token: position 35 consumed Int
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
parse_expression: starting at position 40 (Ident(num))
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (num)
consume_token: position 41 consumed Plus
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (zero)
consume_token: position 43 consumed EqualEqual
consume_token: position 44 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed Semicolon
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
        0: Token(Var),
        1: Token(Ident, "num"),
        2: Token(Equal),
        3: Token(Int, "123"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "zero"),
        7: Token(Equal),
        8: Token(Int, "0"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "negative"),
        12: Token(Equal),
        13: Token(Minus),
        14: Token(Int, "42"),
        15: Token(Semicolon),
        16: Token(Ident, "assert"),
        17: Token(LeftParen),
        18: Token(Ident, "num"),
        19: Token(EqualEqual),
        20: Token(Int, "123"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Ident, "assert"),
        24: Token(LeftParen),
        25: Token(Ident, "zero"),
        26: Token(EqualEqual),
        27: Token(Int, "0"),
        28: Token(RightParen),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "negative"),
        33: Token(EqualEqual),
        34: Token(Minus),
        35: Token(Int, "42"),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Ident, "assert"),
        39: Token(LeftParen),
        40: Token(Ident, "num"),
        41: Token(Plus),
        42: Token(Ident, "zero"),
        43: Token(EqualEqual),
        44: Token(Int, "123"),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Eof)
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
                    "num",
                ),
                value: Some(
                    Literal(
                        Int(
                            123,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "zero",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
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
                            Int(
                                42,
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
                                "num",
                            ),
                            Literal(
                                Int(
                                    123,
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
                                "zero",
                            ),
                            Literal(
                                Int(
                                    0,
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
                                    Int(
                                        42,
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
                                    "num",
                                ),
                                Identifier(
                                    "zero",
                                ),
                            ),
                            Literal(
                                Int(
                                    123,
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
                        name: "num",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        123,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "zero",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
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
                                                    value: Int(
                                                        42,
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
                                                    name: "num",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    123,
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
                                                    name: "zero",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    0,
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
                                                                value: Int(
                                                                    42,
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
                                                                    name: "num",
                                                                },
                                                            ),
                                                            attr_name: "op_add",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalVariable {
                                                                name: "zero",
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
                                                    123,
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