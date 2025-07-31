# Program
Status: 🟢
Assertions: 4

```rustleaf
var increment = |x| x + 1;
var double = |y| y * 2;
var add_ten = |z| z + 10;

assert(increment(5) == 6);
assert(double(7) == 14);  
assert(add_ten(15) == 25);
assert(increment(0) == 1);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 3 consumed Pipe
consume_token: position 4 consumed Ident
consume_token: position 5 consumed Pipe
parse_expression: starting at position 6 (Ident(x))
consume_token: position 6 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 7 consumed Plus
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Var)
parse_statement: starting at position 10 (Var)
consume_token: position 10 consumed Var
consume_token: position 11 consumed Ident
consume_token: position 12 consumed Equal
parse_expression: starting at position 13 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 13 consumed Pipe
consume_token: position 14 consumed Ident
consume_token: position 15 consumed Pipe
parse_expression: starting at position 16 (Ident(y))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (y)
consume_token: position 17 consumed Star
consume_token: position 18 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 20 (Var)
parse_statement: starting at position 20 (Var)
consume_token: position 20 consumed Var
consume_token: position 21 consumed Ident
consume_token: position 22 consumed Equal
parse_expression: starting at position 23 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 23 consumed Pipe
consume_token: position 24 consumed Ident
consume_token: position 25 consumed Pipe
parse_expression: starting at position 26 (Ident(z))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (z)
consume_token: position 27 consumed Plus
consume_token: position 28 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 30 (Ident(assert))
parse_statement: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 31 consumed LeftParen
parse_expression: starting at position 32 (Ident(increment))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (increment)
consume_token: position 33 consumed LeftParen
parse_expression: starting at position 34 (Int(5))
consume_token: position 34 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed RightParen
consume_token: position 36 consumed EqualEqual
consume_token: position 37 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed Semicolon
parse_program: parsing statement at position 40 (Ident(assert))
parse_statement: starting at position 40 (Ident(assert))
consume_token: position 40 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 40 (Ident(assert))
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 41 consumed LeftParen
parse_expression: starting at position 42 (Ident(double))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (double)
consume_token: position 43 consumed LeftParen
parse_expression: starting at position 44 (Int(7))
consume_token: position 44 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed RightParen
consume_token: position 46 consumed EqualEqual
consume_token: position 47 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 49 consumed Semicolon
parse_program: parsing statement at position 50 (Ident(assert))
parse_statement: starting at position 50 (Ident(assert))
consume_token: position 50 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 50 (Ident(assert))
consume_token: position 50 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 51 consumed LeftParen
parse_expression: starting at position 52 (Ident(add_ten))
consume_token: position 52 consumed Ident
parse_primary: success - parsed identifier (add_ten)
consume_token: position 53 consumed LeftParen
parse_expression: starting at position 54 (Int(15))
consume_token: position 54 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed RightParen
consume_token: position 56 consumed EqualEqual
consume_token: position 57 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 58 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 59 consumed Semicolon
parse_program: parsing statement at position 60 (Ident(assert))
parse_statement: starting at position 60 (Ident(assert))
consume_token: position 60 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 60 (Ident(assert))
consume_token: position 60 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 61 consumed LeftParen
parse_expression: starting at position 62 (Ident(increment))
consume_token: position 62 consumed Ident
parse_primary: success - parsed identifier (increment)
consume_token: position 63 consumed LeftParen
parse_expression: starting at position 64 (Int(0))
consume_token: position 64 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 65 consumed RightParen
consume_token: position 66 consumed EqualEqual
consume_token: position 67 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 68 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Semicolon
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
        1: Token(Ident, "increment"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Ident, "x"),
        5: Token(Pipe),
        6: Token(Ident, "x"),
        7: Token(Plus),
        8: Token(Int, "1"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "double"),
        12: Token(Equal),
        13: Token(Pipe),
        14: Token(Ident, "y"),
        15: Token(Pipe),
        16: Token(Ident, "y"),
        17: Token(Star),
        18: Token(Int, "2"),
        19: Token(Semicolon),
        20: Token(Var),
        21: Token(Ident, "add_ten"),
        22: Token(Equal),
        23: Token(Pipe),
        24: Token(Ident, "z"),
        25: Token(Pipe),
        26: Token(Ident, "z"),
        27: Token(Plus),
        28: Token(Int, "10"),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "increment"),
        33: Token(LeftParen),
        34: Token(Int, "5"),
        35: Token(RightParen),
        36: Token(EqualEqual),
        37: Token(Int, "6"),
        38: Token(RightParen),
        39: Token(Semicolon),
        40: Token(Ident, "assert"),
        41: Token(LeftParen),
        42: Token(Ident, "double"),
        43: Token(LeftParen),
        44: Token(Int, "7"),
        45: Token(RightParen),
        46: Token(EqualEqual),
        47: Token(Int, "14"),
        48: Token(RightParen),
        49: Token(Semicolon),
        50: Token(Ident, "assert"),
        51: Token(LeftParen),
        52: Token(Ident, "add_ten"),
        53: Token(LeftParen),
        54: Token(Int, "15"),
        55: Token(RightParen),
        56: Token(EqualEqual),
        57: Token(Int, "25"),
        58: Token(RightParen),
        59: Token(Semicolon),
        60: Token(Ident, "assert"),
        61: Token(LeftParen),
        62: Token(Ident, "increment"),
        63: Token(LeftParen),
        64: Token(Int, "0"),
        65: Token(RightParen),
        66: Token(EqualEqual),
        67: Token(Int, "1"),
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
            VarDecl {
                pattern: Variable(
                    "increment",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "double",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "y",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "y",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "add_ten",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "z",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "z",
                                ),
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "increment",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    6,
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
                            FunctionCall(
                                Identifier(
                                    "double",
                                ),
                                [
                                    Literal(
                                        Int(
                                            7,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    14,
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
                            FunctionCall(
                                Identifier(
                                    "add_ten",
                                ),
                                [
                                    Literal(
                                        Int(
                                            15,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    25,
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
                            FunctionCall(
                                Identifier(
                                    "increment",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ],
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
                        name: "increment",
                        init_expr: Some(
                            RustValue(
                                EvalLambda {
                                    data: LambdaData {
                                        params: [
                                            "x",
                                        ],
                                        body: RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalGetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "x",
                                                            },
                                                        ),
                                                        attr_name: "op_add",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                1,
                                                            ),
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    },
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "double",
                        init_expr: Some(
                            RustValue(
                                EvalLambda {
                                    data: LambdaData {
                                        params: [
                                            "y",
                                        ],
                                        body: RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalGetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "y",
                                                            },
                                                        ),
                                                        attr_name: "op_mul",
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
                                    },
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "add_ten",
                        init_expr: Some(
                            RustValue(
                                EvalLambda {
                                    data: LambdaData {
                                        params: [
                                            "z",
                                        ],
                                        body: RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalGetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "z",
                                                            },
                                                        ),
                                                        attr_name: "op_add",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                10,
                                                            ),
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    },
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
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalVariable {
                                                            name: "increment",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    5,
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
                                                    6,
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
                                                        EvalVariable {
                                                            name: "double",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    7,
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
                                                    14,
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
                                                        EvalVariable {
                                                            name: "add_ten",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    15,
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
                                                    25,
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
                                                        EvalVariable {
                                                            name: "increment",
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
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
            ],
        },
    ),
)
```