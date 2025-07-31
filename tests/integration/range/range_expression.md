# Program
Status: 🟢
Assertions: 4

```rustleaf
// Test using ranges in for loops and expressions
var sum = 0;
for i in 1..5 {
    sum += i;
}
assert(sum == 10);  // 1 + 2 + 3 + 4 = 10

// Test using ranges with different step sizes
var range = 0..10;
var even_count = 0;
for x in range {
    if x % 2 == 0 {
        even_count += 1;
    }
}
assert(even_count == 5);  // 0, 2, 4, 6, 8

// Test range as expression
var small_range = 3..6;
assert(4 in small_range);
assert(not (6 in small_range));
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(0))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (For)
parse_statement: starting at position 5 (For)
parse_expression: starting at position 5 (For)
consume_token: position 5 consumed For
parse_primary: success - parsing for expression
consume_token: position 6 consumed Ident
consume_token: position 7 consumed In
parse_expression: starting at position 8 (Int(1))
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 9 consumed DotDot
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed LeftBrace
parse_statement: starting at position 12 (Ident(sum))
consume_token: position 12 consumed Ident
consume_token: position 13 consumed PlusEqual
parse_expression: starting at position 14 (Ident(i))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (i)
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 16 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 17 (Ident(assert))
parse_statement: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 18 consumed LeftParen
parse_expression: starting at position 19 (Ident(sum))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (sum)
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed Semicolon
parse_program: parsing statement at position 24 (Var)
parse_statement: starting at position 24 (Var)
consume_token: position 24 consumed Var
consume_token: position 25 consumed Ident
consume_token: position 26 consumed Equal
parse_expression: starting at position 27 (Int(0))
consume_token: position 27 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 28 consumed DotDot
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
parse_expression: starting at position 34 (Int(0))
consume_token: position 34 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 36 (For)
parse_statement: starting at position 36 (For)
parse_expression: starting at position 36 (For)
consume_token: position 36 consumed For
parse_primary: success - parsing for expression
consume_token: position 37 consumed Ident
consume_token: position 38 consumed In
parse_expression: starting at position 39 (Ident(range))
consume_token: position 39 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed LeftBrace
parse_statement: starting at position 41 (If)
parse_expression: starting at position 41 (If)
consume_token: position 41 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 42 (Ident(x))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 43 consumed Percent
consume_token: position 44 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 45 consumed EqualEqual
consume_token: position 46 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed LeftBrace
parse_statement: starting at position 48 (Ident(even_count))
consume_token: position 48 consumed Ident
consume_token: position 49 consumed PlusEqual
parse_expression: starting at position 50 (Int(1))
consume_token: position 50 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 51 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 52 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
consume_token: position 53 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 54 (Ident(assert))
parse_statement: starting at position 54 (Ident(assert))
consume_token: position 54 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 54 (Ident(assert))
consume_token: position 54 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 55 consumed LeftParen
parse_expression: starting at position 56 (Ident(even_count))
consume_token: position 56 consumed Ident
parse_primary: success - parsed identifier (even_count)
consume_token: position 57 consumed EqualEqual
consume_token: position 58 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 59 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 60 consumed Semicolon
parse_program: parsing statement at position 61 (Var)
parse_statement: starting at position 61 (Var)
consume_token: position 61 consumed Var
consume_token: position 62 consumed Ident
consume_token: position 63 consumed Equal
parse_expression: starting at position 64 (Int(3))
consume_token: position 64 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 65 consumed DotDot
consume_token: position 66 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 67 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 68 (Ident(assert))
parse_statement: starting at position 68 (Ident(assert))
consume_token: position 68 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 68 (Ident(assert))
consume_token: position 68 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 69 consumed LeftParen
parse_expression: starting at position 70 (Int(4))
consume_token: position 70 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 71 consumed In
consume_token: position 72 consumed Ident
parse_primary: success - parsed identifier (small_range)
parse_expression: success - parsed precedence expression
consume_token: position 73 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 74 consumed Semicolon
parse_program: parsing statement at position 75 (Ident(assert))
parse_statement: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 76 consumed LeftParen
parse_expression: starting at position 77 (Not)
consume_token: position 77 consumed Not
consume_token: position 78 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 79 (Int(6))
consume_token: position 79 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 80 consumed In
consume_token: position 81 consumed Ident
parse_primary: success - parsed identifier (small_range)
parse_expression: success - parsed precedence expression
consume_token: position 82 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 83 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 84 consumed Semicolon
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
        1: Token(Ident, "sum"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(For),
        6: Token(Ident, "i"),
        7: Token(In),
        8: Token(Int, "1"),
        9: Token(DotDot),
        10: Token(Int, "5"),
        11: Token(LeftBrace),
        12: Token(Ident, "sum"),
        13: Token(PlusEqual),
        14: Token(Ident, "i"),
        15: Token(Semicolon),
        16: Token(RightBrace),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "sum"),
        20: Token(EqualEqual),
        21: Token(Int, "10"),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Var),
        25: Token(Ident, "range"),
        26: Token(Equal),
        27: Token(Int, "0"),
        28: Token(DotDot),
        29: Token(Int, "10"),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "even_count"),
        33: Token(Equal),
        34: Token(Int, "0"),
        35: Token(Semicolon),
        36: Token(For),
        37: Token(Ident, "x"),
        38: Token(In),
        39: Token(Ident, "range"),
        40: Token(LeftBrace),
        41: Token(If),
        42: Token(Ident, "x"),
        43: Token(Percent),
        44: Token(Int, "2"),
        45: Token(EqualEqual),
        46: Token(Int, "0"),
        47: Token(LeftBrace),
        48: Token(Ident, "even_count"),
        49: Token(PlusEqual),
        50: Token(Int, "1"),
        51: Token(Semicolon),
        52: Token(RightBrace),
        53: Token(RightBrace),
        54: Token(Ident, "assert"),
        55: Token(LeftParen),
        56: Token(Ident, "even_count"),
        57: Token(EqualEqual),
        58: Token(Int, "5"),
        59: Token(RightParen),
        60: Token(Semicolon),
        61: Token(Var),
        62: Token(Ident, "small_range"),
        63: Token(Equal),
        64: Token(Int, "3"),
        65: Token(DotDot),
        66: Token(Int, "6"),
        67: Token(Semicolon),
        68: Token(Ident, "assert"),
        69: Token(LeftParen),
        70: Token(Int, "4"),
        71: Token(In),
        72: Token(Ident, "small_range"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Ident, "assert"),
        76: Token(LeftParen),
        77: Token(Not),
        78: Token(LeftParen),
        79: Token(Int, "6"),
        80: Token(In),
        81: Token(Ident, "small_range"),
        82: Token(RightParen),
        83: Token(RightParen),
        84: Token(Semicolon),
        85: Token(Eof)
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
                    "sum",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "i",
                    ),
                    iter: RangeExclusive(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        Literal(
                            Int(
                                5,
                            ),
                        ),
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "sum",
                                ),
                                op: AddAssign,
                                value: Identifier(
                                    "i",
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                },
            ),
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
                                    10,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "range",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                0,
                            ),
                        ),
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "even_count",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: Identifier(
                        "range",
                    ),
                    body: Block {
                        statements: [
                            Expression(
                                If {
                                    condition: Eq(
                                        Mod(
                                            Identifier(
                                                "x",
                                            ),
                                            Literal(
                                                Int(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            Int(
                                                0,
                                            ),
                                        ),
                                    ),
                                    then_expr: Block {
                                        statements: [
                                            Assignment {
                                                target: Identifier(
                                                    "even_count",
                                                ),
                                                op: AddAssign,
                                                value: Literal(
                                                    Int(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: None,
                                },
                            ),
                        ],
                        final_expr: None,
                    },
                },
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "even_count",
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "small_range",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                3,
                            ),
                        ),
                        Literal(
                            Int(
                                6,
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
                        In(
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Identifier(
                                "small_range",
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
                        Not(
                            In(
                                Literal(
                                    Int(
                                        6,
                                    ),
                                ),
                                Identifier(
                                    "small_range",
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
                        name: "sum",
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
                    EvalFor {
                        var_name: "i",
                        iter_expr: RustValue(
                            EvalLiteral {
                                value: Range(
                                    Range {
                                        start: 1,
                                        end: 5,
                                        inclusive: false,
                                    },
                                ),
                            },
                        ),
                        body: RustValue(
                            EvalBlock {
                                statements: [
                                    RustValue(
                                        EvalAssign {
                                            name: "sum",
                                            expr: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalVariable {
                                                                    name: "sum",
                                                                },
                                                            ),
                                                            attr_name: "op_add",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalVariable {
                                                                name: "i",
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                        },
                                    ),
                                ],
                                final_expr: None,
                            },
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
                        name: "range",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Range(
                                        Range {
                                            start: 0,
                                            end: 10,
                                            inclusive: false,
                                        },
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "even_count",
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
                    EvalFor {
                        var_name: "x",
                        iter_expr: RustValue(
                            EvalVariable {
                                name: "range",
                            },
                        ),
                        body: RustValue(
                            EvalBlock {
                                statements: [
                                    RustValue(
                                        EvalIf {
                                            condition: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "x",
                                                                                },
                                                                            ),
                                                                            attr_name: "op_mod",
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
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            then_expr: RustValue(
                                                EvalBlock {
                                                    statements: [
                                                        RustValue(
                                                            EvalAssign {
                                                                name: "even_count",
                                                                expr: RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalGetAttr {
                                                                                obj_expr: RustValue(
                                                                                    EvalVariable {
                                                                                        name: "even_count",
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
                                                        ),
                                                    ],
                                                    final_expr: None,
                                                },
                                            ),
                                            else_expr: None,
                                        },
                                    ),
                                ],
                                final_expr: None,
                            },
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
                                                    name: "even_count",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                        ],
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "small_range",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Range(
                                        Range {
                                            start: 3,
                                            end: 6,
                                            inclusive: false,
                                        },
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
                                                    name: "small_range",
                                                },
                                            ),
                                            attr_name: "op_contains",
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
                                EvalLogicalNot {
                                    expr: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "small_range",
                                                        },
                                                    ),
                                                    attr_name: "op_contains",
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