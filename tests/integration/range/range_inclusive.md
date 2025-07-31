# Program
Status: 🟢
Assertions: 12

```rustleaf
// Test inclusive ranges (1..=10 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
var range = 1..=10;

// Test range properties
assert(range.start == 1);
assert(range.end == 10);
assert(range.inclusive == true);

// Test range iteration (convert to list)
var list = range.to_list();
assert(list.length == 10);
assert(list[0] == 1);
assert(list[9] == 10);
assert(10 in list);

// Test range membership
assert(5 in range);
assert(10 in range);
assert(1 in range);
assert(not (11 in range));
assert(not (0 in range));
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
consume_token: position 4 consumed DotDotEqual
consume_token: position 5 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 7 (Ident(assert))
parse_statement: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 8 consumed LeftParen
parse_expression: starting at position 9 (Ident(range))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (range)
consume_token: position 10 consumed Dot
consume_token: position 11 consumed Ident
consume_token: position 12 consumed EqualEqual
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
parse_program: parsing statement at position 16 (Ident(assert))
parse_statement: starting at position 16 (Ident(assert))
consume_token: position 16 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 16 (Ident(assert))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 17 consumed LeftParen
parse_expression: starting at position 18 (Ident(range))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (range)
consume_token: position 19 consumed Dot
consume_token: position 20 consumed Ident
consume_token: position 21 consumed EqualEqual
consume_token: position 22 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
parse_program: parsing statement at position 25 (Ident(assert))
parse_statement: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 26 consumed LeftParen
parse_expression: starting at position 27 (Ident(range))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (range)
consume_token: position 28 consumed Dot
consume_token: position 29 consumed Ident
consume_token: position 30 consumed EqualEqual
consume_token: position 31 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed Semicolon
parse_program: parsing statement at position 34 (Var)
parse_statement: starting at position 34 (Var)
consume_token: position 34 consumed Var
consume_token: position 35 consumed Ident
consume_token: position 36 consumed Equal
parse_expression: starting at position 37 (Ident(range))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (range)
consume_token: position 38 consumed Dot
consume_token: position 39 consumed Ident
consume_token: position 40 consumed LeftParen
consume_token: position 41 consumed RightParen
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
parse_expression: starting at position 45 (Ident(list))
consume_token: position 45 consumed Ident
parse_primary: success - parsed identifier (list)
consume_token: position 46 consumed Dot
consume_token: position 47 consumed Ident
consume_token: position 48 consumed EqualEqual
consume_token: position 49 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 50 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 51 consumed Semicolon
parse_program: parsing statement at position 52 (Ident(assert))
parse_statement: starting at position 52 (Ident(assert))
consume_token: position 52 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 52 (Ident(assert))
consume_token: position 52 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 53 consumed LeftParen
parse_expression: starting at position 54 (Ident(list))
consume_token: position 54 consumed Ident
parse_primary: success - parsed identifier (list)
consume_token: position 55 consumed LeftBracket
parse_expression: starting at position 56 (Int(0))
consume_token: position 56 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 57 consumed RightBracket
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
parse_expression: starting at position 64 (Ident(list))
consume_token: position 64 consumed Ident
parse_primary: success - parsed identifier (list)
consume_token: position 65 consumed LeftBracket
parse_expression: starting at position 66 (Int(9))
consume_token: position 66 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 67 consumed RightBracket
consume_token: position 68 consumed EqualEqual
consume_token: position 69 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 70 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 71 consumed Semicolon
parse_program: parsing statement at position 72 (Ident(assert))
parse_statement: starting at position 72 (Ident(assert))
consume_token: position 72 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 72 (Ident(assert))
consume_token: position 72 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 73 consumed LeftParen
parse_expression: starting at position 74 (Int(10))
consume_token: position 74 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 75 consumed In
consume_token: position 76 consumed Ident
parse_primary: success - parsed identifier (list)
parse_expression: success - parsed precedence expression
consume_token: position 77 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 78 consumed Semicolon
parse_program: parsing statement at position 79 (Ident(assert))
parse_statement: starting at position 79 (Ident(assert))
consume_token: position 79 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 79 (Ident(assert))
consume_token: position 79 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 80 consumed LeftParen
parse_expression: starting at position 81 (Int(5))
consume_token: position 81 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 82 consumed In
consume_token: position 83 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 84 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 85 consumed Semicolon
parse_program: parsing statement at position 86 (Ident(assert))
parse_statement: starting at position 86 (Ident(assert))
consume_token: position 86 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 86 (Ident(assert))
consume_token: position 86 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 87 consumed LeftParen
parse_expression: starting at position 88 (Int(10))
consume_token: position 88 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 89 consumed In
consume_token: position 90 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 91 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 92 consumed Semicolon
parse_program: parsing statement at position 93 (Ident(assert))
parse_statement: starting at position 93 (Ident(assert))
consume_token: position 93 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 93 (Ident(assert))
consume_token: position 93 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 94 consumed LeftParen
parse_expression: starting at position 95 (Int(1))
consume_token: position 95 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 96 consumed In
consume_token: position 97 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 98 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 99 consumed Semicolon
parse_program: parsing statement at position 100 (Ident(assert))
parse_statement: starting at position 100 (Ident(assert))
consume_token: position 100 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 100 (Ident(assert))
consume_token: position 100 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 101 consumed LeftParen
parse_expression: starting at position 102 (Not)
consume_token: position 102 consumed Not
consume_token: position 103 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 104 (Int(11))
consume_token: position 104 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 105 consumed In
consume_token: position 106 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 107 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 108 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 109 consumed Semicolon
parse_program: parsing statement at position 110 (Ident(assert))
parse_statement: starting at position 110 (Ident(assert))
consume_token: position 110 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 110 (Ident(assert))
consume_token: position 110 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 111 consumed LeftParen
parse_expression: starting at position 112 (Not)
consume_token: position 112 consumed Not
consume_token: position 113 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 114 (Int(0))
consume_token: position 114 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 115 consumed In
consume_token: position 116 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 117 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 118 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 119 consumed Semicolon
parse_program: parsed 14 statements
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
        1: Token(Ident, "range"),
        2: Token(Equal),
        3: Token(Int, "1"),
        4: Token(DotDotEqual),
        5: Token(Int, "10"),
        6: Token(Semicolon),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Ident, "range"),
        10: Token(Dot),
        11: Token(Ident, "start"),
        12: Token(EqualEqual),
        13: Token(Int, "1"),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Ident, "assert"),
        17: Token(LeftParen),
        18: Token(Ident, "range"),
        19: Token(Dot),
        20: Token(Ident, "end"),
        21: Token(EqualEqual),
        22: Token(Int, "10"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Ident, "assert"),
        26: Token(LeftParen),
        27: Token(Ident, "range"),
        28: Token(Dot),
        29: Token(Ident, "inclusive"),
        30: Token(EqualEqual),
        31: Token(True),
        32: Token(RightParen),
        33: Token(Semicolon),
        34: Token(Var),
        35: Token(Ident, "list"),
        36: Token(Equal),
        37: Token(Ident, "range"),
        38: Token(Dot),
        39: Token(Ident, "to_list"),
        40: Token(LeftParen),
        41: Token(RightParen),
        42: Token(Semicolon),
        43: Token(Ident, "assert"),
        44: Token(LeftParen),
        45: Token(Ident, "list"),
        46: Token(Dot),
        47: Token(Ident, "length"),
        48: Token(EqualEqual),
        49: Token(Int, "10"),
        50: Token(RightParen),
        51: Token(Semicolon),
        52: Token(Ident, "assert"),
        53: Token(LeftParen),
        54: Token(Ident, "list"),
        55: Token(LeftBracket),
        56: Token(Int, "0"),
        57: Token(RightBracket),
        58: Token(EqualEqual),
        59: Token(Int, "1"),
        60: Token(RightParen),
        61: Token(Semicolon),
        62: Token(Ident, "assert"),
        63: Token(LeftParen),
        64: Token(Ident, "list"),
        65: Token(LeftBracket),
        66: Token(Int, "9"),
        67: Token(RightBracket),
        68: Token(EqualEqual),
        69: Token(Int, "10"),
        70: Token(RightParen),
        71: Token(Semicolon),
        72: Token(Ident, "assert"),
        73: Token(LeftParen),
        74: Token(Int, "10"),
        75: Token(In),
        76: Token(Ident, "list"),
        77: Token(RightParen),
        78: Token(Semicolon),
        79: Token(Ident, "assert"),
        80: Token(LeftParen),
        81: Token(Int, "5"),
        82: Token(In),
        83: Token(Ident, "range"),
        84: Token(RightParen),
        85: Token(Semicolon),
        86: Token(Ident, "assert"),
        87: Token(LeftParen),
        88: Token(Int, "10"),
        89: Token(In),
        90: Token(Ident, "range"),
        91: Token(RightParen),
        92: Token(Semicolon),
        93: Token(Ident, "assert"),
        94: Token(LeftParen),
        95: Token(Int, "1"),
        96: Token(In),
        97: Token(Ident, "range"),
        98: Token(RightParen),
        99: Token(Semicolon),
        100: Token(Ident, "assert"),
        101: Token(LeftParen),
        102: Token(Not),
        103: Token(LeftParen),
        104: Token(Int, "11"),
        105: Token(In),
        106: Token(Ident, "range"),
        107: Token(RightParen),
        108: Token(RightParen),
        109: Token(Semicolon),
        110: Token(Ident, "assert"),
        111: Token(LeftParen),
        112: Token(Not),
        113: Token(LeftParen),
        114: Token(Int, "0"),
        115: Token(In),
        116: Token(Ident, "range"),
        117: Token(RightParen),
        118: Token(RightParen),
        119: Token(Semicolon),
        120: Token(Eof)
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
                    "range",
                ),
                value: Some(
                    RangeInclusive(
                        Literal(
                            Int(
                                1,
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetAttr(
                                Identifier(
                                    "range",
                                ),
                                "start",
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
                            GetAttr(
                                Identifier(
                                    "range",
                                ),
                                "end",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetAttr(
                                Identifier(
                                    "range",
                                ),
                                "inclusive",
                            ),
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "list",
                ),
                value: Some(
                    MethodCall(
                        Identifier(
                            "range",
                        ),
                        "to_list",
                        [],
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
                            GetAttr(
                                Identifier(
                                    "list",
                                ),
                                "length",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        9,
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        In(
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                            Identifier(
                                "list",
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
                        In(
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            Identifier(
                                "range",
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
                        In(
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                            Identifier(
                                "range",
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
                        In(
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Identifier(
                                "range",
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
                                        11,
                                    ),
                                ),
                                Identifier(
                                    "range",
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
                        Not(
                            In(
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                Identifier(
                                    "range",
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
                        name: "range",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Range(
                                        Range {
                                            start: 1,
                                            end: 10,
                                            inclusive: true,
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
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "range",
                                                        },
                                                    ),
                                                    attr_name: "start",
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
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "range",
                                                        },
                                                    ),
                                                    attr_name: "end",
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
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "range",
                                                        },
                                                    ),
                                                    attr_name: "inclusive",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Bool(
                                                    true,
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
                        name: "list",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "range",
                                                },
                                            ),
                                            attr_name: "to_list",
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
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "list",
                                                        },
                                                    ),
                                                    attr_name: "length",
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
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "list",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                0,
                                                            ),
                                                        },
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
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "list",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                9,
                                                            ),
                                                        },
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
                                                    name: "list",
                                                },
                                            ),
                                            attr_name: "op_contains",
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
                                                    name: "range",
                                                },
                                            ),
                                            attr_name: "op_contains",
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
                                                    name: "range",
                                                },
                                            ),
                                            attr_name: "op_contains",
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
                                                    name: "range",
                                                },
                                            ),
                                            attr_name: "op_contains",
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
                                EvalLogicalNot {
                                    expr: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "range",
                                                        },
                                                    ),
                                                    attr_name: "op_contains",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Int(
                                                            11,
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
                                                            name: "range",
                                                        },
                                                    ),
                                                    attr_name: "op_contains",
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