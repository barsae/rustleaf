# Program
Status: 🟢
Assertions: 3

```rustleaf
var result1 = if true { 42 } else { 0 };
var result2 = if false { 100 } else { 200 };
var x = 5;
var result3 = if x > 3 { "big" } else { "small" };
assert(result1 == 42);
assert(result2 == 200);
assert(result3 == "big");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (If)
consume_token: position 3 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 4 (True)
consume_token: position 4 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed LeftBrace
parse_statement: starting at position 6 (Int(42))
parse_statement: falling back to expression statement
parse_expression: starting at position 6 (Int(42))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 7
parse_expression: starting at position 6 (Int(42))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed RightBrace
consume_token: position 8 consumed Else
consume_token: position 9 consumed LeftBrace
parse_statement: starting at position 10 (Int(0))
parse_statement: falling back to expression statement
parse_expression: starting at position 10 (Int(0))
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 11
parse_expression: starting at position 10 (Int(0))
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 13 (Var)
parse_statement: starting at position 13 (Var)
consume_token: position 13 consumed Var
consume_token: position 14 consumed Ident
consume_token: position 15 consumed Equal
parse_expression: starting at position 16 (If)
consume_token: position 16 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 17 (False)
consume_token: position 17 consumed False
parse_primary: success - parsed boolean literal (false)
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed LeftBrace
parse_statement: starting at position 19 (Int(100))
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Int(100))
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 20
parse_expression: starting at position 19 (Int(100))
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightBrace
consume_token: position 21 consumed Else
consume_token: position 22 consumed LeftBrace
parse_statement: starting at position 23 (Int(200))
parse_statement: falling back to expression statement
parse_expression: starting at position 23 (Int(200))
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 24
parse_expression: starting at position 23 (Int(200))
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 26 (Var)
parse_statement: starting at position 26 (Var)
consume_token: position 26 consumed Var
consume_token: position 27 consumed Ident
consume_token: position 28 consumed Equal
parse_expression: starting at position 29 (Int(5))
consume_token: position 29 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 31 (Var)
parse_statement: starting at position 31 (Var)
consume_token: position 31 consumed Var
consume_token: position 32 consumed Ident
consume_token: position 33 consumed Equal
parse_expression: starting at position 34 (If)
consume_token: position 34 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 35 (Ident(x))
consume_token: position 35 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 36 consumed Greater
consume_token: position 37 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed LeftBrace
parse_statement: starting at position 39 (String(big))
parse_statement: falling back to expression statement
parse_expression: starting at position 39 (String(big))
consume_token: position 39 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 40
parse_expression: starting at position 39 (String(big))
consume_token: position 39 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed RightBrace
consume_token: position 41 consumed Else
consume_token: position 42 consumed LeftBrace
parse_statement: starting at position 43 (String(small))
parse_statement: falling back to expression statement
parse_expression: starting at position 43 (String(small))
consume_token: position 43 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 44
parse_expression: starting at position 43 (String(small))
consume_token: position 43 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 46 (Ident(assert))
parse_statement: starting at position 46 (Ident(assert))
consume_token: position 46 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 46 (Ident(assert))
consume_token: position 46 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 47 consumed LeftParen
parse_expression: starting at position 48 (Ident(result1))
consume_token: position 48 consumed Ident
parse_primary: success - parsed identifier (result1)
consume_token: position 49 consumed EqualEqual
consume_token: position 50 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 51 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 52 consumed Semicolon
parse_program: parsing statement at position 53 (Ident(assert))
parse_statement: starting at position 53 (Ident(assert))
consume_token: position 53 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 53 (Ident(assert))
consume_token: position 53 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 54 consumed LeftParen
parse_expression: starting at position 55 (Ident(result2))
consume_token: position 55 consumed Ident
parse_primary: success - parsed identifier (result2)
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
parse_expression: starting at position 62 (Ident(result3))
consume_token: position 62 consumed Ident
parse_primary: success - parsed identifier (result3)
consume_token: position 63 consumed EqualEqual
consume_token: position 64 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 65 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 66 consumed Semicolon
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
        1: Token(Ident, "result1"),
        2: Token(Equal),
        3: Token(If),
        4: Token(True),
        5: Token(LeftBrace),
        6: Token(Int, "42"),
        7: Token(RightBrace),
        8: Token(Else),
        9: Token(LeftBrace),
        10: Token(Int, "0"),
        11: Token(RightBrace),
        12: Token(Semicolon),
        13: Token(Var),
        14: Token(Ident, "result2"),
        15: Token(Equal),
        16: Token(If),
        17: Token(False),
        18: Token(LeftBrace),
        19: Token(Int, "100"),
        20: Token(RightBrace),
        21: Token(Else),
        22: Token(LeftBrace),
        23: Token(Int, "200"),
        24: Token(RightBrace),
        25: Token(Semicolon),
        26: Token(Var),
        27: Token(Ident, "x"),
        28: Token(Equal),
        29: Token(Int, "5"),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "result3"),
        33: Token(Equal),
        34: Token(If),
        35: Token(Ident, "x"),
        36: Token(Greater),
        37: Token(Int, "3"),
        38: Token(LeftBrace),
        39: Token(String, "big"),
        40: Token(RightBrace),
        41: Token(Else),
        42: Token(LeftBrace),
        43: Token(String, "small"),
        44: Token(RightBrace),
        45: Token(Semicolon),
        46: Token(Ident, "assert"),
        47: Token(LeftParen),
        48: Token(Ident, "result1"),
        49: Token(EqualEqual),
        50: Token(Int, "42"),
        51: Token(RightParen),
        52: Token(Semicolon),
        53: Token(Ident, "assert"),
        54: Token(LeftParen),
        55: Token(Ident, "result2"),
        56: Token(EqualEqual),
        57: Token(Int, "200"),
        58: Token(RightParen),
        59: Token(Semicolon),
        60: Token(Ident, "assert"),
        61: Token(LeftParen),
        62: Token(Ident, "result3"),
        63: Token(EqualEqual),
        64: Token(String, "big"),
        65: Token(RightParen),
        66: Token(Semicolon),
        67: Token(Eof)
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
                    "result1",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                true,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Literal(
                            Bool(
                                false,
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        Int(
                                            200,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "big",
                                    ),
                                ),
                            ),
                        },
                        else_expr: Some(
                            Block {
                                statements: [],
                                final_expr: Some(
                                    Literal(
                                        String(
                                            "small",
                                        ),
                                    ),
                                ),
                            },
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
                            Identifier(
                                "result1",
                            ),
                            Literal(
                                Int(
                                    42,
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
                                "result2",
                            ),
                            Literal(
                                Int(
                                    200,
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
                                "result3",
                            ),
                            Literal(
                                String(
                                    "big",
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
                        name: "result1",
                        init_expr: Some(
                            RustValue(
                                EvalIf {
                                    condition: RustValue(
                                        EvalLiteral {
                                            value: Bool(
                                                true,
                                            ),
                                        },
                                    ),
                                    then_expr: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Int(
                                                            42,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    else_expr: Some(
                                        RustValue(
                                            EvalBlock {
                                                statements: [],
                                                final_expr: Some(
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
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result2",
                        init_expr: Some(
                            RustValue(
                                EvalIf {
                                    condition: RustValue(
                                        EvalLiteral {
                                            value: Bool(
                                                false,
                                            ),
                                        },
                                    ),
                                    then_expr: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Int(
                                                            100,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    else_expr: Some(
                                        RustValue(
                                            EvalBlock {
                                                statements: [],
                                                final_expr: Some(
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                200,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        5,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result3",
                        init_expr: Some(
                            RustValue(
                                EvalIf {
                                    condition: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "x",
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
                                    then_expr: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalLiteral {
                                                        value: String(
                                                            "big",
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    else_expr: Some(
                                        RustValue(
                                            EvalBlock {
                                                statements: [],
                                                final_expr: Some(
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: String(
                                                                "small",
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
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
                                                    name: "result1",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                                    name: "result2",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    200,
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
                                                    name: "result3",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "big",
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