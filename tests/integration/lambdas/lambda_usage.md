# Program
Status: 🟢
Assertions: 4

```rustleaf
// Test lambda expressions in practical usage scenarios

// Simple lambda assignment and calling
var add_one = |x| x + 1;
assert(add_one(5) == 6);

// Lambda with multiple parameters (when supported)
// var add = |x, y| x + y;
// assert(add(3, 4) == 7);

// Lambda as argument to other functions (higher-order functions)
fn apply(func, value) {
    func(value)
}
var double = |x| x * 2;
assert(apply(double, 21) == 42);

// Lambda with block body
var complex_func = |n| {
    var result = n * n;
    result + 1
};
assert(complex_func(3) == 10); // 3*3 + 1 = 10

// Lambda closures - capturing variables from outer scope
var multiplier = 3;
var multiply_by_three = |x| x * multiplier;
assert(multiply_by_three(4) == 12);
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
parse_program: parsing statement at position 10 (Ident(assert))
parse_statement: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 11 consumed LeftParen
parse_expression: starting at position 12 (Ident(add_one))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (add_one)
consume_token: position 13 consumed LeftParen
parse_expression: starting at position 14 (Int(5))
consume_token: position 14 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed RightParen
consume_token: position 16 consumed EqualEqual
consume_token: position 17 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Semicolon
parse_program: parsing statement at position 20 (Fn)
parse_statement: starting at position 20 (Fn)
consume_token: position 20 consumed Fn
consume_token: position 21 consumed Ident
consume_token: position 22 consumed LeftParen
consume_token: position 23 consumed Ident
consume_token: position 24 consumed Comma
consume_token: position 25 consumed Ident
consume_token: position 26 consumed RightParen
consume_token: position 27 consumed LeftBrace
parse_statement: starting at position 28 (Ident(func))
consume_token: position 28 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 28 (Ident(func))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (func)
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Ident(value))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (value)
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 32
parse_expression: starting at position 28 (Ident(func))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (func)
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Ident(value))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (value)
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 33 (Var)
parse_statement: starting at position 33 (Var)
consume_token: position 33 consumed Var
consume_token: position 34 consumed Ident
consume_token: position 35 consumed Equal
parse_expression: starting at position 36 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 36 consumed Pipe
consume_token: position 37 consumed Ident
consume_token: position 38 consumed Pipe
parse_expression: starting at position 39 (Ident(x))
consume_token: position 39 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 40 consumed Star
consume_token: position 41 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
consume_token: position 42 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 43 (Ident(assert))
parse_statement: starting at position 43 (Ident(assert))
consume_token: position 43 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 43 (Ident(assert))
consume_token: position 43 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 44 consumed LeftParen
parse_expression: starting at position 45 (Ident(apply))
consume_token: position 45 consumed Ident
parse_primary: success - parsed identifier (apply)
consume_token: position 46 consumed LeftParen
parse_expression: starting at position 47 (Ident(double))
consume_token: position 47 consumed Ident
parse_primary: success - parsed identifier (double)
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed Comma
parse_expression: starting at position 49 (Int(21))
consume_token: position 49 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 50 consumed RightParen
consume_token: position 51 consumed EqualEqual
consume_token: position 52 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 53 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed Semicolon
parse_program: parsing statement at position 55 (Var)
parse_statement: starting at position 55 (Var)
consume_token: position 55 consumed Var
consume_token: position 56 consumed Ident
consume_token: position 57 consumed Equal
parse_expression: starting at position 58 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 58 consumed Pipe
consume_token: position 59 consumed Ident
consume_token: position 60 consumed Pipe
consume_token: position 61 consumed LeftBrace
parse_statement: starting at position 62 (Var)
consume_token: position 62 consumed Var
consume_token: position 63 consumed Ident
consume_token: position 64 consumed Equal
parse_expression: starting at position 65 (Ident(n))
consume_token: position 65 consumed Ident
parse_primary: success - parsed identifier (n)
consume_token: position 66 consumed Star
consume_token: position 67 consumed Ident
parse_primary: success - parsed identifier (n)
parse_expression: success - parsed precedence expression
consume_token: position 68 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 69 (Ident(result))
consume_token: position 69 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 69 (Ident(result))
consume_token: position 69 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 70 consumed Plus
consume_token: position 71 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 72
parse_expression: starting at position 69 (Ident(result))
consume_token: position 69 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 70 consumed Plus
consume_token: position 71 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 72 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 73 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 74 (Ident(assert))
parse_statement: starting at position 74 (Ident(assert))
consume_token: position 74 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 74 (Ident(assert))
consume_token: position 74 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 75 consumed LeftParen
parse_expression: starting at position 76 (Ident(complex_func))
consume_token: position 76 consumed Ident
parse_primary: success - parsed identifier (complex_func)
consume_token: position 77 consumed LeftParen
parse_expression: starting at position 78 (Int(3))
consume_token: position 78 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 79 consumed RightParen
consume_token: position 80 consumed EqualEqual
consume_token: position 81 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 82 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 83 consumed Semicolon
parse_program: parsing statement at position 84 (Var)
parse_statement: starting at position 84 (Var)
consume_token: position 84 consumed Var
consume_token: position 85 consumed Ident
consume_token: position 86 consumed Equal
parse_expression: starting at position 87 (Int(3))
consume_token: position 87 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 88 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 89 (Var)
parse_statement: starting at position 89 (Var)
consume_token: position 89 consumed Var
consume_token: position 90 consumed Ident
consume_token: position 91 consumed Equal
parse_expression: starting at position 92 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 92 consumed Pipe
consume_token: position 93 consumed Ident
consume_token: position 94 consumed Pipe
parse_expression: starting at position 95 (Ident(x))
consume_token: position 95 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 96 consumed Star
consume_token: position 97 consumed Ident
parse_primary: success - parsed identifier (multiplier)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
consume_token: position 98 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 99 (Ident(assert))
parse_statement: starting at position 99 (Ident(assert))
consume_token: position 99 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 99 (Ident(assert))
consume_token: position 99 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 100 consumed LeftParen
parse_expression: starting at position 101 (Ident(multiply_by_three))
consume_token: position 101 consumed Ident
parse_primary: success - parsed identifier (multiply_by_three)
consume_token: position 102 consumed LeftParen
parse_expression: starting at position 103 (Int(4))
consume_token: position 103 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 104 consumed RightParen
consume_token: position 105 consumed EqualEqual
consume_token: position 106 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 107 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 108 consumed Semicolon
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
        0: Token(Var),
        1: Token(Ident, "add_one"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Ident, "x"),
        5: Token(Pipe),
        6: Token(Ident, "x"),
        7: Token(Plus),
        8: Token(Int, "1"),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "add_one"),
        13: Token(LeftParen),
        14: Token(Int, "5"),
        15: Token(RightParen),
        16: Token(EqualEqual),
        17: Token(Int, "6"),
        18: Token(RightParen),
        19: Token(Semicolon),
        20: Token(Fn),
        21: Token(Ident, "apply"),
        22: Token(LeftParen),
        23: Token(Ident, "func"),
        24: Token(Comma),
        25: Token(Ident, "value"),
        26: Token(RightParen),
        27: Token(LeftBrace),
        28: Token(Ident, "func"),
        29: Token(LeftParen),
        30: Token(Ident, "value"),
        31: Token(RightParen),
        32: Token(RightBrace),
        33: Token(Var),
        34: Token(Ident, "double"),
        35: Token(Equal),
        36: Token(Pipe),
        37: Token(Ident, "x"),
        38: Token(Pipe),
        39: Token(Ident, "x"),
        40: Token(Star),
        41: Token(Int, "2"),
        42: Token(Semicolon),
        43: Token(Ident, "assert"),
        44: Token(LeftParen),
        45: Token(Ident, "apply"),
        46: Token(LeftParen),
        47: Token(Ident, "double"),
        48: Token(Comma),
        49: Token(Int, "21"),
        50: Token(RightParen),
        51: Token(EqualEqual),
        52: Token(Int, "42"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Var),
        56: Token(Ident, "complex_func"),
        57: Token(Equal),
        58: Token(Pipe),
        59: Token(Ident, "n"),
        60: Token(Pipe),
        61: Token(LeftBrace),
        62: Token(Var),
        63: Token(Ident, "result"),
        64: Token(Equal),
        65: Token(Ident, "n"),
        66: Token(Star),
        67: Token(Ident, "n"),
        68: Token(Semicolon),
        69: Token(Ident, "result"),
        70: Token(Plus),
        71: Token(Int, "1"),
        72: Token(RightBrace),
        73: Token(Semicolon),
        74: Token(Ident, "assert"),
        75: Token(LeftParen),
        76: Token(Ident, "complex_func"),
        77: Token(LeftParen),
        78: Token(Int, "3"),
        79: Token(RightParen),
        80: Token(EqualEqual),
        81: Token(Int, "10"),
        82: Token(RightParen),
        83: Token(Semicolon),
        84: Token(Var),
        85: Token(Ident, "multiplier"),
        86: Token(Equal),
        87: Token(Int, "3"),
        88: Token(Semicolon),
        89: Token(Var),
        90: Token(Ident, "multiply_by_three"),
        91: Token(Equal),
        92: Token(Pipe),
        93: Token(Ident, "x"),
        94: Token(Pipe),
        95: Token(Ident, "x"),
        96: Token(Star),
        97: Token(Ident, "multiplier"),
        98: Token(Semicolon),
        99: Token(Ident, "assert"),
        100: Token(LeftParen),
        101: Token(Ident, "multiply_by_three"),
        102: Token(LeftParen),
        103: Token(Int, "4"),
        104: Token(RightParen),
        105: Token(EqualEqual),
        106: Token(Int, "12"),
        107: Token(RightParen),
        108: Token(Semicolon),
        109: Token(Eof)
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
                    "add_one",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "add_one",
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
            FnDecl {
                name: "apply",
                params: [
                    Parameter {
                        name: "func",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "value",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        FunctionCall(
                            Identifier(
                                "func",
                            ),
                            [
                                Identifier(
                                    "value",
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "double",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "x",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "apply",
                                ),
                                [
                                    Identifier(
                                        "double",
                                    ),
                                    Literal(
                                        Int(
                                            21,
                                        ),
                                    ),
                                ],
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
            VarDecl {
                pattern: Variable(
                    "complex_func",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "n",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "result",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "n",
                                                ),
                                                Identifier(
                                                    "n",
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Add(
                                        Identifier(
                                            "result",
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
                                    "complex_func",
                                ),
                                [
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "multiplier",
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
                    "multiply_by_three",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Mul(
                                Identifier(
                                    "x",
                                ),
                                Identifier(
                                    "multiplier",
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
                                    "multiply_by_three",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
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
                        name: "add_one",
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
                                                            name: "add_one",
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
                    EvalFunction {
                        data: FunctionData {
                            name: "apply",
                            params: [
                                (
                                    "func",
                                    None,
                                    Regular,
                                ),
                                (
                                    "value",
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
                                                    EvalVariable {
                                                        name: "func",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalVariable {
                                                            name: "value",
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
                        name: "double",
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
                                                            name: "apply",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalVariable {
                                                                name: "double",
                                                            },
                                                        ),
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    21,
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
                    EvalDeclare {
                        name: "complex_func",
                        init_expr: Some(
                            RustValue(
                                EvalLambda {
                                    data: LambdaData {
                                        params: [
                                            "n",
                                        ],
                                        body: RustValue(
                                            EvalBlock {
                                                statements: [
                                                    RustValue(
                                                        EvalDeclare {
                                                            name: "result",
                                                            init_expr: Some(
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalGetAttr {
                                                                                obj_expr: RustValue(
                                                                                    EvalVariable {
                                                                                        name: "n",
                                                                                    },
                                                                                ),
                                                                                attr_name: "op_mul",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "n",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                final_expr: Some(
                                                    RustValue(
                                                        EvalCall {
                                                            func_expr: RustValue(
                                                                EvalGetAttr {
                                                                    obj_expr: RustValue(
                                                                        EvalVariable {
                                                                            name: "result",
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
                                                ),
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
                                                            name: "complex_func",
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
                                                    10,
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
                    EvalDeclare {
                        name: "multiplier",
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
                        name: "multiply_by_three",
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
                                                        attr_name: "op_mul",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalVariable {
                                                            name: "multiplier",
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
                                                            name: "multiply_by_three",
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
            ],
        },
    ),
)
```