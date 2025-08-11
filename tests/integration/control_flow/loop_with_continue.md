# Program
Status: 🟢
Assertions: 5

```rustleaf
var i = 0;
var result = loop {
    i = i + 1;
    if i < 3 {
        continue;
    }
    break i * 10;
};

var j = 0;
var count = 0;
var result2 = loop {
    j = j + 1;
    if j <= 5 {
        count = count + 1;
        continue;
    }
    break j + count;
};

assert(result == 30);
assert(i == 3);
assert(result2 == 11);  
assert(j == 6);
assert(count == 5);
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
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Loop)
consume_token: position 8 consumed Loop
parse_primary: success - parsing loop expression
consume_token: position 9 consumed LeftBrace
parse_statement: starting at position 10 (Ident(i))
consume_token: position 10 consumed Ident
consume_token: position 11 consumed Equal
parse_expression: starting at position 12 (Ident(i))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (i)
consume_token: position 13 consumed Plus
consume_token: position 14 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 16 (If)
parse_expression: starting at position 16 (If)
consume_token: position 16 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 17 (Ident(i))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (i)
consume_token: position 18 consumed Less
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed LeftBrace
parse_statement: starting at position 21 (Continue)
consume_token: position 21 consumed Continue
consume_token: position 22 consumed Semicolon
parse_statement: success - parsed continue statement
consume_token: position 23 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_statement: starting at position 24 (Break)
consume_token: position 24 consumed Break
parse_expression: starting at position 25 (Ident(i))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (i)
consume_token: position 26 consumed Star
consume_token: position 27 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 28 consumed Semicolon
parse_statement: success - parsed break statement
consume_token: position 29 consumed RightBrace
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
parse_program: parsing statement at position 36 (Var)
parse_statement: starting at position 36 (Var)
consume_token: position 36 consumed Var
consume_token: position 37 consumed Ident
consume_token: position 38 consumed Equal
parse_expression: starting at position 39 (Int(0))
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 41 (Var)
parse_statement: starting at position 41 (Var)
consume_token: position 41 consumed Var
consume_token: position 42 consumed Ident
consume_token: position 43 consumed Equal
parse_expression: starting at position 44 (Loop)
consume_token: position 44 consumed Loop
parse_primary: success - parsing loop expression
consume_token: position 45 consumed LeftBrace
parse_statement: starting at position 46 (Ident(j))
consume_token: position 46 consumed Ident
consume_token: position 47 consumed Equal
parse_expression: starting at position 48 (Ident(j))
consume_token: position 48 consumed Ident
parse_primary: success - parsed identifier (j)
consume_token: position 49 consumed Plus
consume_token: position 50 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 51 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 52 (If)
parse_expression: starting at position 52 (If)
consume_token: position 52 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 53 (Ident(j))
consume_token: position 53 consumed Ident
parse_primary: success - parsed identifier (j)
consume_token: position 54 consumed LessEqual
consume_token: position 55 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 56 consumed LeftBrace
parse_statement: starting at position 57 (Ident(count))
consume_token: position 57 consumed Ident
consume_token: position 58 consumed Equal
parse_expression: starting at position 59 (Ident(count))
consume_token: position 59 consumed Ident
parse_primary: success - parsed identifier (count)
consume_token: position 60 consumed Plus
consume_token: position 61 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 63 (Continue)
consume_token: position 63 consumed Continue
consume_token: position 64 consumed Semicolon
parse_statement: success - parsed continue statement
consume_token: position 65 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_statement: starting at position 66 (Break)
consume_token: position 66 consumed Break
parse_expression: starting at position 67 (Ident(j))
consume_token: position 67 consumed Ident
parse_primary: success - parsed identifier (j)
consume_token: position 68 consumed Plus
consume_token: position 69 consumed Ident
parse_primary: success - parsed identifier (count)
parse_expression: success - parsed precedence expression
consume_token: position 70 consumed Semicolon
parse_statement: success - parsed break statement
consume_token: position 71 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 72 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 73 (Ident(assert))
parse_statement: starting at position 73 (Ident(assert))
consume_token: position 73 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 73 (Ident(assert))
consume_token: position 73 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 74 consumed LeftParen
parse_expression: starting at position 75 (Ident(result))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 76 consumed EqualEqual
consume_token: position 77 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 78 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 79 consumed Semicolon
parse_program: parsing statement at position 80 (Ident(assert))
parse_statement: starting at position 80 (Ident(assert))
consume_token: position 80 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 80 (Ident(assert))
consume_token: position 80 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 81 consumed LeftParen
parse_expression: starting at position 82 (Ident(i))
consume_token: position 82 consumed Ident
parse_primary: success - parsed identifier (i)
consume_token: position 83 consumed EqualEqual
consume_token: position 84 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 85 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 86 consumed Semicolon
parse_program: parsing statement at position 87 (Ident(assert))
parse_statement: starting at position 87 (Ident(assert))
consume_token: position 87 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 87 (Ident(assert))
consume_token: position 87 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 88 consumed LeftParen
parse_expression: starting at position 89 (Ident(result2))
consume_token: position 89 consumed Ident
parse_primary: success - parsed identifier (result2)
consume_token: position 90 consumed EqualEqual
consume_token: position 91 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 92 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 93 consumed Semicolon
parse_program: parsing statement at position 94 (Ident(assert))
parse_statement: starting at position 94 (Ident(assert))
consume_token: position 94 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 94 (Ident(assert))
consume_token: position 94 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 95 consumed LeftParen
parse_expression: starting at position 96 (Ident(j))
consume_token: position 96 consumed Ident
parse_primary: success - parsed identifier (j)
consume_token: position 97 consumed EqualEqual
consume_token: position 98 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 99 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 100 consumed Semicolon
parse_program: parsing statement at position 101 (Ident(assert))
parse_statement: starting at position 101 (Ident(assert))
consume_token: position 101 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 101 (Ident(assert))
consume_token: position 101 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 102 consumed LeftParen
parse_expression: starting at position 103 (Ident(count))
consume_token: position 103 consumed Ident
parse_primary: success - parsed identifier (count)
consume_token: position 104 consumed EqualEqual
consume_token: position 105 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 106 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 107 consumed Semicolon
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
        1: Token(Ident, "i"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(Loop),
        9: Token(LeftBrace),
        10: Token(Ident, "i"),
        11: Token(Equal),
        12: Token(Ident, "i"),
        13: Token(Plus),
        14: Token(Int, "1"),
        15: Token(Semicolon),
        16: Token(If),
        17: Token(Ident, "i"),
        18: Token(Less),
        19: Token(Int, "3"),
        20: Token(LeftBrace),
        21: Token(Continue),
        22: Token(Semicolon),
        23: Token(RightBrace),
        24: Token(Break),
        25: Token(Ident, "i"),
        26: Token(Star),
        27: Token(Int, "10"),
        28: Token(Semicolon),
        29: Token(RightBrace),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "j"),
        33: Token(Equal),
        34: Token(Int, "0"),
        35: Token(Semicolon),
        36: Token(Var),
        37: Token(Ident, "count"),
        38: Token(Equal),
        39: Token(Int, "0"),
        40: Token(Semicolon),
        41: Token(Var),
        42: Token(Ident, "result2"),
        43: Token(Equal),
        44: Token(Loop),
        45: Token(LeftBrace),
        46: Token(Ident, "j"),
        47: Token(Equal),
        48: Token(Ident, "j"),
        49: Token(Plus),
        50: Token(Int, "1"),
        51: Token(Semicolon),
        52: Token(If),
        53: Token(Ident, "j"),
        54: Token(LessEqual),
        55: Token(Int, "5"),
        56: Token(LeftBrace),
        57: Token(Ident, "count"),
        58: Token(Equal),
        59: Token(Ident, "count"),
        60: Token(Plus),
        61: Token(Int, "1"),
        62: Token(Semicolon),
        63: Token(Continue),
        64: Token(Semicolon),
        65: Token(RightBrace),
        66: Token(Break),
        67: Token(Ident, "j"),
        68: Token(Plus),
        69: Token(Ident, "count"),
        70: Token(Semicolon),
        71: Token(RightBrace),
        72: Token(Semicolon),
        73: Token(Ident, "assert"),
        74: Token(LeftParen),
        75: Token(Ident, "result"),
        76: Token(EqualEqual),
        77: Token(Int, "30"),
        78: Token(RightParen),
        79: Token(Semicolon),
        80: Token(Ident, "assert"),
        81: Token(LeftParen),
        82: Token(Ident, "i"),
        83: Token(EqualEqual),
        84: Token(Int, "3"),
        85: Token(RightParen),
        86: Token(Semicolon),
        87: Token(Ident, "assert"),
        88: Token(LeftParen),
        89: Token(Ident, "result2"),
        90: Token(EqualEqual),
        91: Token(Int, "11"),
        92: Token(RightParen),
        93: Token(Semicolon),
        94: Token(Ident, "assert"),
        95: Token(LeftParen),
        96: Token(Ident, "j"),
        97: Token(EqualEqual),
        98: Token(Int, "6"),
        99: Token(RightParen),
        100: Token(Semicolon),
        101: Token(Ident, "assert"),
        102: Token(LeftParen),
        103: Token(Ident, "count"),
        104: Token(EqualEqual),
        105: Token(Int, "5"),
        106: Token(RightParen),
        107: Token(Semicolon),
        108: Token(Eof)
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
                    "i",
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
                    "result",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "i",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "i",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Lt(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Continue,
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                                Break(
                                    Some(
                                        Mul(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    10,
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "j",
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
                    "count",
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
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "j",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "j",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Le(
                                            Identifier(
                                                "j",
                                            ),
                                            Literal(
                                                Int(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Assignment {
                                                    target: Identifier(
                                                        "count",
                                                    ),
                                                    op: Assign,
                                                    value: Add(
                                                        Identifier(
                                                            "count",
                                                        ),
                                                        Literal(
                                                            Int(
                                                                1,
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Continue,
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                                Break(
                                    Some(
                                        Add(
                                            Identifier(
                                                "j",
                                            ),
                                            Identifier(
                                                "count",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
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
                                Int(
                                    30,
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
                                "i",
                            ),
                            Literal(
                                Int(
                                    3,
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
                                    11,
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
                                "j",
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
                            Identifier(
                                "count",
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
        ],
    ),
)
```

# Eval
```rust
Ok(
    RustValue(<unknown>),
)
```