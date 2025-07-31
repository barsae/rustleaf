# Program
Status: 🟢
Assertions: 4

```rustleaf
var x = 1;
var result = if x > 0 {
    "positive"
} else {
    "zero or negative"
};

var y = -5;
var result2 = if y > 0 {
    "positive"
} else {
    "zero or negative"
};

assert(result == "positive");
assert(result2 == "zero or negative");
assert(x == 1);
assert(y == -5);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(1))
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
parse_expression: starting at position 8 (If)
consume_token: position 8 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 10 consumed Greater
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed LeftBrace
parse_statement: starting at position 13 (String(positive))
parse_statement: falling back to expression statement
parse_expression: starting at position 13 (String(positive))
consume_token: position 13 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 14
parse_expression: starting at position 13 (String(positive))
consume_token: position 13 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightBrace
consume_token: position 15 consumed Else
consume_token: position 16 consumed LeftBrace
parse_statement: starting at position 17 (String(zero or negative))
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (String(zero or negative))
consume_token: position 17 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 18
parse_expression: starting at position 17 (String(zero or negative))
consume_token: position 17 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 20 (Var)
parse_statement: starting at position 20 (Var)
consume_token: position 20 consumed Var
consume_token: position 21 consumed Ident
consume_token: position 22 consumed Equal
parse_expression: starting at position 23 (Minus)
consume_token: position 23 consumed Minus
consume_token: position 24 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 26 (Var)
parse_statement: starting at position 26 (Var)
consume_token: position 26 consumed Var
consume_token: position 27 consumed Ident
consume_token: position 28 consumed Equal
parse_expression: starting at position 29 (If)
consume_token: position 29 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 30 (Ident(y))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (y)
consume_token: position 31 consumed Greater
consume_token: position 32 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed LeftBrace
parse_statement: starting at position 34 (String(positive))
parse_statement: falling back to expression statement
parse_expression: starting at position 34 (String(positive))
consume_token: position 34 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 35
parse_expression: starting at position 34 (String(positive))
consume_token: position 34 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed RightBrace
consume_token: position 36 consumed Else
consume_token: position 37 consumed LeftBrace
parse_statement: starting at position 38 (String(zero or negative))
parse_statement: falling back to expression statement
parse_expression: starting at position 38 (String(zero or negative))
consume_token: position 38 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 39
parse_expression: starting at position 38 (String(zero or negative))
consume_token: position 38 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 41 (Ident(assert))
parse_statement: starting at position 41 (Ident(assert))
consume_token: position 41 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 41 (Ident(assert))
consume_token: position 41 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 42 consumed LeftParen
parse_expression: starting at position 43 (Ident(result))
consume_token: position 43 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 44 consumed EqualEqual
consume_token: position 45 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed Semicolon
parse_program: parsing statement at position 48 (Ident(assert))
parse_statement: starting at position 48 (Ident(assert))
consume_token: position 48 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 48 (Ident(assert))
consume_token: position 48 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 49 consumed LeftParen
parse_expression: starting at position 50 (Ident(result2))
consume_token: position 50 consumed Ident
parse_primary: success - parsed identifier (result2)
consume_token: position 51 consumed EqualEqual
consume_token: position 52 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 53 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed Semicolon
parse_program: parsing statement at position 55 (Ident(assert))
parse_statement: starting at position 55 (Ident(assert))
consume_token: position 55 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 55 (Ident(assert))
consume_token: position 55 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 56 consumed LeftParen
parse_expression: starting at position 57 (Ident(x))
consume_token: position 57 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 58 consumed EqualEqual
consume_token: position 59 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 60 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 61 consumed Semicolon
parse_program: parsing statement at position 62 (Ident(assert))
parse_statement: starting at position 62 (Ident(assert))
consume_token: position 62 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 62 (Ident(assert))
consume_token: position 62 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 63 consumed LeftParen
parse_expression: starting at position 64 (Ident(y))
consume_token: position 64 consumed Ident
parse_primary: success - parsed identifier (y)
consume_token: position 65 consumed EqualEqual
consume_token: position 66 consumed Minus
consume_token: position 67 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 68 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Semicolon
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "1"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(If),
        9: Token(Ident, "x"),
        10: Token(Greater),
        11: Token(Int, "0"),
        12: Token(LeftBrace),
        13: Token(String, "positive"),
        14: Token(RightBrace),
        15: Token(Else),
        16: Token(LeftBrace),
        17: Token(String, "zero or negative"),
        18: Token(RightBrace),
        19: Token(Semicolon),
        20: Token(Var),
        21: Token(Ident, "y"),
        22: Token(Equal),
        23: Token(Minus),
        24: Token(Int, "5"),
        25: Token(Semicolon),
        26: Token(Var),
        27: Token(Ident, "result2"),
        28: Token(Equal),
        29: Token(If),
        30: Token(Ident, "y"),
        31: Token(Greater),
        32: Token(Int, "0"),
        33: Token(LeftBrace),
        34: Token(String, "positive"),
        35: Token(RightBrace),
        36: Token(Else),
        37: Token(LeftBrace),
        38: Token(String, "zero or negative"),
        39: Token(RightBrace),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Ident, "result"),
        44: Token(EqualEqual),
        45: Token(String, "positive"),
        46: Token(RightParen),
        47: Token(Semicolon),
        48: Token(Ident, "assert"),
        49: Token(LeftParen),
        50: Token(Ident, "result2"),
        51: Token(EqualEqual),
        52: Token(String, "zero or negative"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Ident, "assert"),
        56: Token(LeftParen),
        57: Token(Ident, "x"),
        58: Token(EqualEqual),
        59: Token(Int, "1"),
        60: Token(RightParen),
        61: Token(Semicolon),
        62: Token(Ident, "assert"),
        63: Token(LeftParen),
        64: Token(Ident, "y"),
        65: Token(EqualEqual),
        66: Token(Minus),
        67: Token(Int, "5"),
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
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
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
                    "y",
                ),
                value: Some(
                    Neg(
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    If {
                        condition: Gt(
                            Identifier(
                                "y",
                            ),
                            Literal(
                                Int(
                                    0,
                                ),
                            ),
                        ),
                        then_expr: Block {
                            statements: [],
                            final_expr: Some(
                                Literal(
                                    String(
                                        "positive",
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
                                            "zero or negative",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "positive",
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
                                String(
                                    "zero or negative",
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
                                "x",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "y",
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        5,
                                    ),
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
                RustValue(
                    EvalDeclare {
                        name: "result",
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
                                                            0,
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
                                                            "positive",
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
                                                                "zero or negative",
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
                        name: "y",
                        init_expr: Some(
                            RustValue(
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
                    EvalDeclare {
                        name: "result2",
                        init_expr: Some(
                            RustValue(
                                EvalIf {
                                    condition: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "y",
                                                        },
                                                    ),
                                                    attr_name: "op_gt",
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
                                    then_expr: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalLiteral {
                                                        value: String(
                                                            "positive",
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
                                                                "zero or negative",
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
                                                    "positive",
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
                                                value: String(
                                                    "zero or negative",
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
                                                    name: "x",
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
                                                    name: "y",
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
                                                                    5,
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
            ],
        },
    ),
)
```