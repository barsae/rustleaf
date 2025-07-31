# Program
Status: 🟢
Assertions: 4

```rustleaf
fn add(x, y) { x + y }
fn multiply(a, b) { a * b }

var sum = add(5, 3);
var product = multiply(4, 6);

assert(sum == 8);
assert(product == 24);
assert(add(10, -2) == 8);
assert(multiply(0, 100) == 0);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed Ident
consume_token: position 4 consumed Comma
consume_token: position 5 consumed Ident
consume_token: position 6 consumed RightParen
consume_token: position 7 consumed LeftBrace
parse_statement: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 9 consumed Plus
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 11
parse_expression: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 9 consumed Plus
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 12 (Fn)
parse_statement: starting at position 12 (Fn)
consume_token: position 12 consumed Fn
consume_token: position 13 consumed Ident
consume_token: position 14 consumed LeftParen
consume_token: position 15 consumed Ident
consume_token: position 16 consumed Comma
consume_token: position 17 consumed Ident
consume_token: position 18 consumed RightParen
consume_token: position 19 consumed LeftBrace
parse_statement: starting at position 20 (Ident(a))
consume_token: position 20 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 20 (Ident(a))
consume_token: position 20 consumed Ident
parse_primary: success - parsed identifier (a)
consume_token: position 21 consumed Star
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (b)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 23
parse_expression: starting at position 20 (Ident(a))
consume_token: position 20 consumed Ident
parse_primary: success - parsed identifier (a)
consume_token: position 21 consumed Star
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (b)
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 24 (Var)
parse_statement: starting at position 24 (Var)
consume_token: position 24 consumed Var
consume_token: position 25 consumed Ident
consume_token: position 26 consumed Equal
parse_expression: starting at position 27 (Ident(add))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (add)
consume_token: position 28 consumed LeftParen
parse_expression: starting at position 29 (Int(5))
consume_token: position 29 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Comma
parse_expression: starting at position 31 (Int(3))
consume_token: position 31 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 34 (Var)
parse_statement: starting at position 34 (Var)
consume_token: position 34 consumed Var
consume_token: position 35 consumed Ident
consume_token: position 36 consumed Equal
parse_expression: starting at position 37 (Ident(multiply))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (multiply)
consume_token: position 38 consumed LeftParen
parse_expression: starting at position 39 (Int(4))
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed Comma
parse_expression: starting at position 41 (Int(6))
consume_token: position 41 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 42 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 44 (Ident(assert))
parse_statement: starting at position 44 (Ident(assert))
consume_token: position 44 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 44 (Ident(assert))
consume_token: position 44 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 45 consumed LeftParen
parse_expression: starting at position 46 (Ident(sum))
consume_token: position 46 consumed Ident
parse_primary: success - parsed identifier (sum)
consume_token: position 47 consumed EqualEqual
consume_token: position 48 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 49 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 50 consumed Semicolon
parse_program: parsing statement at position 51 (Ident(assert))
parse_statement: starting at position 51 (Ident(assert))
consume_token: position 51 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 51 (Ident(assert))
consume_token: position 51 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 52 consumed LeftParen
parse_expression: starting at position 53 (Ident(product))
consume_token: position 53 consumed Ident
parse_primary: success - parsed identifier (product)
consume_token: position 54 consumed EqualEqual
consume_token: position 55 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 56 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 57 consumed Semicolon
parse_program: parsing statement at position 58 (Ident(assert))
parse_statement: starting at position 58 (Ident(assert))
consume_token: position 58 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 58 (Ident(assert))
consume_token: position 58 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 59 consumed LeftParen
parse_expression: starting at position 60 (Ident(add))
consume_token: position 60 consumed Ident
parse_primary: success - parsed identifier (add)
consume_token: position 61 consumed LeftParen
parse_expression: starting at position 62 (Int(10))
consume_token: position 62 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 63 consumed Comma
parse_expression: starting at position 64 (Minus)
consume_token: position 64 consumed Minus
consume_token: position 65 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 66 consumed RightParen
consume_token: position 67 consumed EqualEqual
consume_token: position 68 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 70 consumed Semicolon
parse_program: parsing statement at position 71 (Ident(assert))
parse_statement: starting at position 71 (Ident(assert))
consume_token: position 71 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 71 (Ident(assert))
consume_token: position 71 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 72 consumed LeftParen
parse_expression: starting at position 73 (Ident(multiply))
consume_token: position 73 consumed Ident
parse_primary: success - parsed identifier (multiply)
consume_token: position 74 consumed LeftParen
parse_expression: starting at position 75 (Int(0))
consume_token: position 75 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 76 consumed Comma
parse_expression: starting at position 77 (Int(100))
consume_token: position 77 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 78 consumed RightParen
consume_token: position 79 consumed EqualEqual
consume_token: position 80 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 81 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 82 consumed Semicolon
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
        0: Token(Fn),
        1: Token(Ident, "add"),
        2: Token(LeftParen),
        3: Token(Ident, "x"),
        4: Token(Comma),
        5: Token(Ident, "y"),
        6: Token(RightParen),
        7: Token(LeftBrace),
        8: Token(Ident, "x"),
        9: Token(Plus),
        10: Token(Ident, "y"),
        11: Token(RightBrace),
        12: Token(Fn),
        13: Token(Ident, "multiply"),
        14: Token(LeftParen),
        15: Token(Ident, "a"),
        16: Token(Comma),
        17: Token(Ident, "b"),
        18: Token(RightParen),
        19: Token(LeftBrace),
        20: Token(Ident, "a"),
        21: Token(Star),
        22: Token(Ident, "b"),
        23: Token(RightBrace),
        24: Token(Var),
        25: Token(Ident, "sum"),
        26: Token(Equal),
        27: Token(Ident, "add"),
        28: Token(LeftParen),
        29: Token(Int, "5"),
        30: Token(Comma),
        31: Token(Int, "3"),
        32: Token(RightParen),
        33: Token(Semicolon),
        34: Token(Var),
        35: Token(Ident, "product"),
        36: Token(Equal),
        37: Token(Ident, "multiply"),
        38: Token(LeftParen),
        39: Token(Int, "4"),
        40: Token(Comma),
        41: Token(Int, "6"),
        42: Token(RightParen),
        43: Token(Semicolon),
        44: Token(Ident, "assert"),
        45: Token(LeftParen),
        46: Token(Ident, "sum"),
        47: Token(EqualEqual),
        48: Token(Int, "8"),
        49: Token(RightParen),
        50: Token(Semicolon),
        51: Token(Ident, "assert"),
        52: Token(LeftParen),
        53: Token(Ident, "product"),
        54: Token(EqualEqual),
        55: Token(Int, "24"),
        56: Token(RightParen),
        57: Token(Semicolon),
        58: Token(Ident, "assert"),
        59: Token(LeftParen),
        60: Token(Ident, "add"),
        61: Token(LeftParen),
        62: Token(Int, "10"),
        63: Token(Comma),
        64: Token(Minus),
        65: Token(Int, "2"),
        66: Token(RightParen),
        67: Token(EqualEqual),
        68: Token(Int, "8"),
        69: Token(RightParen),
        70: Token(Semicolon),
        71: Token(Ident, "assert"),
        72: Token(LeftParen),
        73: Token(Ident, "multiply"),
        74: Token(LeftParen),
        75: Token(Int, "0"),
        76: Token(Comma),
        77: Token(Int, "100"),
        78: Token(RightParen),
        79: Token(EqualEqual),
        80: Token(Int, "0"),
        81: Token(RightParen),
        82: Token(Semicolon),
        83: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "multiply",
                params: [
                    Parameter {
                        name: "a",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "b",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Mul(
                            Identifier(
                                "a",
                            ),
                            Identifier(
                                "b",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "sum",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
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
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    6,
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
                                "sum",
                            ),
                            Literal(
                                Int(
                                    8,
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
                                "product",
                            ),
                            Literal(
                                Int(
                                    24,
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
                                    "add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    Neg(
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    8,
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
                                    "multiply",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            100,
                                        ),
                                    ),
                                ],
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
                    EvalFunction {
                        data: FunctionData {
                            name: "add",
                            params: [
                                (
                                    "x",
                                    None,
                                    Regular,
                                ),
                                (
                                    "y",
                                    None,
                                    Regular,
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
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
                                                        EvalVariable {
                                                            name: "y",
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "multiply",
                            params: [
                                (
                                    "a",
                                    None,
                                    Regular,
                                ),
                                (
                                    "b",
                                    None,
                                    Regular,
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalGetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "a",
                                                            },
                                                        ),
                                                        attr_name: "op_mul",
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
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "sum",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "add",
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
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "product",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "multiply",
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
                                                    name: "sum",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    8,
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
                                                    name: "product",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    24,
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
                                                            name: "add",
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
                                                        RustValue(
                                                            EvalCall {
                                                                func_expr: RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalLiteral {
                                                                                value: Int(
                                                                                    2,
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
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    8,
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
                                                            name: "multiply",
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
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    100,
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
            ],
        },
    ),
)
```