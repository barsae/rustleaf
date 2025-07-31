# Program
Status: 🟢
Assertions: 10

```rustleaf
// Test exclusive ranges (1..10 = [1, 2, 3, 4, 5, 6, 7, 8, 9])
var range = 1..10;

// Test range properties
assert(range.start == 1);
assert(range.end == 10);
assert(range.inclusive == false);

// Test range iteration (convert to list)
var list = range.to_list();
assert(list.length == 9);
assert(list[0] == 1);
assert(list[8] == 9);
assert(not (10 in list));

// Test range membership
assert(5 in range);
assert(not (10 in range));
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
consume_token: position 4 consumed DotDot
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
consume_token: position 31 consumed False
parse_primary: success - parsed boolean literal (false)
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
parse_expression: starting at position 66 (Int(8))
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
parse_expression: starting at position 74 (Not)
consume_token: position 74 consumed Not
consume_token: position 75 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 76 (Int(10))
consume_token: position 76 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 77 consumed In
consume_token: position 78 consumed Ident
parse_primary: success - parsed identifier (list)
parse_expression: success - parsed precedence expression
consume_token: position 79 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 80 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 81 consumed Semicolon
parse_program: parsing statement at position 82 (Ident(assert))
parse_statement: starting at position 82 (Ident(assert))
consume_token: position 82 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 82 (Ident(assert))
consume_token: position 82 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 83 consumed LeftParen
parse_expression: starting at position 84 (Int(5))
consume_token: position 84 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 85 consumed In
consume_token: position 86 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 87 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 88 consumed Semicolon
parse_program: parsing statement at position 89 (Ident(assert))
parse_statement: starting at position 89 (Ident(assert))
consume_token: position 89 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 89 (Ident(assert))
consume_token: position 89 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 90 consumed LeftParen
parse_expression: starting at position 91 (Not)
consume_token: position 91 consumed Not
consume_token: position 92 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 93 (Int(10))
consume_token: position 93 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 94 consumed In
consume_token: position 95 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 96 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 97 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 98 consumed Semicolon
parse_program: parsing statement at position 99 (Ident(assert))
parse_statement: starting at position 99 (Ident(assert))
consume_token: position 99 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 99 (Ident(assert))
consume_token: position 99 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 100 consumed LeftParen
parse_expression: starting at position 101 (Not)
consume_token: position 101 consumed Not
consume_token: position 102 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 103 (Int(0))
consume_token: position 103 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 104 consumed In
consume_token: position 105 consumed Ident
parse_primary: success - parsed identifier (range)
parse_expression: success - parsed precedence expression
consume_token: position 106 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 107 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 108 consumed Semicolon
parse_program: parsed 12 statements
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
        4: Token(DotDot),
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
        31: Token(False),
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
        49: Token(Int, "9"),
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
        66: Token(Int, "8"),
        67: Token(RightBracket),
        68: Token(EqualEqual),
        69: Token(Int, "9"),
        70: Token(RightParen),
        71: Token(Semicolon),
        72: Token(Ident, "assert"),
        73: Token(LeftParen),
        74: Token(Not),
        75: Token(LeftParen),
        76: Token(Int, "10"),
        77: Token(In),
        78: Token(Ident, "list"),
        79: Token(RightParen),
        80: Token(RightParen),
        81: Token(Semicolon),
        82: Token(Ident, "assert"),
        83: Token(LeftParen),
        84: Token(Int, "5"),
        85: Token(In),
        86: Token(Ident, "range"),
        87: Token(RightParen),
        88: Token(Semicolon),
        89: Token(Ident, "assert"),
        90: Token(LeftParen),
        91: Token(Not),
        92: Token(LeftParen),
        93: Token(Int, "10"),
        94: Token(In),
        95: Token(Ident, "range"),
        96: Token(RightParen),
        97: Token(RightParen),
        98: Token(Semicolon),
        99: Token(Ident, "assert"),
        100: Token(LeftParen),
        101: Token(Not),
        102: Token(LeftParen),
        103: Token(Int, "0"),
        104: Token(In),
        105: Token(Ident, "range"),
        106: Token(RightParen),
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
                    "range",
                ),
                value: Some(
                    RangeExclusive(
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
                                    false,
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
                                    9,
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
                                        8,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    9,
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
                                        10,
                                    ),
                                ),
                                Identifier(
                                    "list",
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
                        Not(
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
                                                    false,
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
                                                    9,
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
                                                                8,
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
                                                    9,
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
                                                            10,
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