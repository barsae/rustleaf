# Program
Status: 🟢
Assertions: 3

```rustleaf
var x = 1;
var result = match x {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result == "one");

var y = 42;
var result2 = match y {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result2 == "other");

var z = 0;
var result3 = match z {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result3 == "zero");
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
parse_expression: starting at position 8 (Match)
consume_token: position 8 consumed Match
parse_primary: success - parsing match expression
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed LeftBrace
consume_token: position 11 consumed Int
consume_token: position 11 consumed Int
consume_token: position 12 consumed Colon
consume_token: position 13 consumed LeftBrace
parse_statement: starting at position 14 (String(zero))
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (String(zero))
consume_token: position 14 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 15
parse_expression: starting at position 14 (String(zero))
consume_token: position 14 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed RightBrace
consume_token: position 16 consumed Int
consume_token: position 16 consumed Int
consume_token: position 17 consumed Colon
consume_token: position 18 consumed LeftBrace
parse_statement: starting at position 19 (String(one))
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (String(one))
consume_token: position 19 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 20
parse_expression: starting at position 19 (String(one))
consume_token: position 19 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightBrace
consume_token: position 21 consumed Ident
consume_token: position 22 consumed Colon
consume_token: position 23 consumed LeftBrace
parse_statement: starting at position 24 (String(other))
parse_statement: falling back to expression statement
parse_expression: starting at position 24 (String(other))
consume_token: position 24 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 25
parse_expression: starting at position 24 (String(other))
consume_token: position 24 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed RightBrace
consume_token: position 26 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 28 (Ident(assert))
parse_statement: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Ident(result))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 31 consumed EqualEqual
consume_token: position 32 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed Semicolon
parse_program: parsing statement at position 35 (Var)
parse_statement: starting at position 35 (Var)
consume_token: position 35 consumed Var
consume_token: position 36 consumed Ident
consume_token: position 37 consumed Equal
parse_expression: starting at position 38 (Int(42))
consume_token: position 38 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 40 (Var)
parse_statement: starting at position 40 (Var)
consume_token: position 40 consumed Var
consume_token: position 41 consumed Ident
consume_token: position 42 consumed Equal
parse_expression: starting at position 43 (Match)
consume_token: position 43 consumed Match
parse_primary: success - parsing match expression
parse_expression: starting at position 44 (Ident(y))
consume_token: position 44 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed LeftBrace
consume_token: position 46 consumed Int
consume_token: position 46 consumed Int
consume_token: position 47 consumed Colon
consume_token: position 48 consumed LeftBrace
parse_statement: starting at position 49 (String(zero))
parse_statement: falling back to expression statement
parse_expression: starting at position 49 (String(zero))
consume_token: position 49 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 50
parse_expression: starting at position 49 (String(zero))
consume_token: position 49 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 50 consumed RightBrace
consume_token: position 51 consumed Int
consume_token: position 51 consumed Int
consume_token: position 52 consumed Colon
consume_token: position 53 consumed LeftBrace
parse_statement: starting at position 54 (String(one))
parse_statement: falling back to expression statement
parse_expression: starting at position 54 (String(one))
consume_token: position 54 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 55
parse_expression: starting at position 54 (String(one))
consume_token: position 54 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed RightBrace
consume_token: position 56 consumed Ident
consume_token: position 57 consumed Colon
consume_token: position 58 consumed LeftBrace
parse_statement: starting at position 59 (String(other))
parse_statement: falling back to expression statement
parse_expression: starting at position 59 (String(other))
consume_token: position 59 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 60
parse_expression: starting at position 59 (String(other))
consume_token: position 59 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 60 consumed RightBrace
consume_token: position 61 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 63 (Ident(assert))
parse_statement: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 64 consumed LeftParen
parse_expression: starting at position 65 (Ident(result2))
consume_token: position 65 consumed Ident
parse_primary: success - parsed identifier (result2)
consume_token: position 66 consumed EqualEqual
consume_token: position 67 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 68 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Semicolon
parse_program: parsing statement at position 70 (Var)
parse_statement: starting at position 70 (Var)
consume_token: position 70 consumed Var
consume_token: position 71 consumed Ident
consume_token: position 72 consumed Equal
parse_expression: starting at position 73 (Int(0))
consume_token: position 73 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 74 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 75 (Var)
parse_statement: starting at position 75 (Var)
consume_token: position 75 consumed Var
consume_token: position 76 consumed Ident
consume_token: position 77 consumed Equal
parse_expression: starting at position 78 (Match)
consume_token: position 78 consumed Match
parse_primary: success - parsing match expression
parse_expression: starting at position 79 (Ident(z))
consume_token: position 79 consumed Ident
parse_primary: success - parsed identifier (z)
parse_expression: success - parsed precedence expression
consume_token: position 80 consumed LeftBrace
consume_token: position 81 consumed Int
consume_token: position 81 consumed Int
consume_token: position 82 consumed Colon
consume_token: position 83 consumed LeftBrace
parse_statement: starting at position 84 (String(zero))
parse_statement: falling back to expression statement
parse_expression: starting at position 84 (String(zero))
consume_token: position 84 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 85
parse_expression: starting at position 84 (String(zero))
consume_token: position 84 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 85 consumed RightBrace
consume_token: position 86 consumed Int
consume_token: position 86 consumed Int
consume_token: position 87 consumed Colon
consume_token: position 88 consumed LeftBrace
parse_statement: starting at position 89 (String(one))
parse_statement: falling back to expression statement
parse_expression: starting at position 89 (String(one))
consume_token: position 89 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 90
parse_expression: starting at position 89 (String(one))
consume_token: position 89 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 90 consumed RightBrace
consume_token: position 91 consumed Ident
consume_token: position 92 consumed Colon
consume_token: position 93 consumed LeftBrace
parse_statement: starting at position 94 (String(other))
parse_statement: falling back to expression statement
parse_expression: starting at position 94 (String(other))
consume_token: position 94 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 95
parse_expression: starting at position 94 (String(other))
consume_token: position 94 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 95 consumed RightBrace
consume_token: position 96 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 97 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 98 (Ident(assert))
parse_statement: starting at position 98 (Ident(assert))
consume_token: position 98 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 98 (Ident(assert))
consume_token: position 98 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 99 consumed LeftParen
parse_expression: starting at position 100 (Ident(result3))
consume_token: position 100 consumed Ident
parse_primary: success - parsed identifier (result3)
consume_token: position 101 consumed EqualEqual
consume_token: position 102 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 103 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 104 consumed Semicolon
parse_program: parsed 9 statements
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
        8: Token(Match),
        9: Token(Ident, "x"),
        10: Token(LeftBrace),
        11: Token(Int, "0"),
        12: Token(Colon),
        13: Token(LeftBrace),
        14: Token(String, "zero"),
        15: Token(RightBrace),
        16: Token(Int, "1"),
        17: Token(Colon),
        18: Token(LeftBrace),
        19: Token(String, "one"),
        20: Token(RightBrace),
        21: Token(Ident, "_"),
        22: Token(Colon),
        23: Token(LeftBrace),
        24: Token(String, "other"),
        25: Token(RightBrace),
        26: Token(RightBrace),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "result"),
        31: Token(EqualEqual),
        32: Token(String, "one"),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Var),
        36: Token(Ident, "y"),
        37: Token(Equal),
        38: Token(Int, "42"),
        39: Token(Semicolon),
        40: Token(Var),
        41: Token(Ident, "result2"),
        42: Token(Equal),
        43: Token(Match),
        44: Token(Ident, "y"),
        45: Token(LeftBrace),
        46: Token(Int, "0"),
        47: Token(Colon),
        48: Token(LeftBrace),
        49: Token(String, "zero"),
        50: Token(RightBrace),
        51: Token(Int, "1"),
        52: Token(Colon),
        53: Token(LeftBrace),
        54: Token(String, "one"),
        55: Token(RightBrace),
        56: Token(Ident, "_"),
        57: Token(Colon),
        58: Token(LeftBrace),
        59: Token(String, "other"),
        60: Token(RightBrace),
        61: Token(RightBrace),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Ident, "result2"),
        66: Token(EqualEqual),
        67: Token(String, "other"),
        68: Token(RightParen),
        69: Token(Semicolon),
        70: Token(Var),
        71: Token(Ident, "z"),
        72: Token(Equal),
        73: Token(Int, "0"),
        74: Token(Semicolon),
        75: Token(Var),
        76: Token(Ident, "result3"),
        77: Token(Equal),
        78: Token(Match),
        79: Token(Ident, "z"),
        80: Token(LeftBrace),
        81: Token(Int, "0"),
        82: Token(Colon),
        83: Token(LeftBrace),
        84: Token(String, "zero"),
        85: Token(RightBrace),
        86: Token(Int, "1"),
        87: Token(Colon),
        88: Token(LeftBrace),
        89: Token(String, "one"),
        90: Token(RightBrace),
        91: Token(Ident, "_"),
        92: Token(Colon),
        93: Token(LeftBrace),
        94: Token(String, "other"),
        95: Token(RightBrace),
        96: Token(RightBrace),
        97: Token(Semicolon),
        98: Token(Ident, "assert"),
        99: Token(LeftParen),
        100: Token(Ident, "result3"),
        101: Token(EqualEqual),
        102: Token(String, "zero"),
        103: Token(RightParen),
        104: Token(Semicolon),
        105: Token(Eof)
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
                    Match {
                        expr: Identifier(
                            "x",
                        ),
                        cases: [
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "zero",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "one",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Wildcard,
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "other",
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
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
                                    "one",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "y",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    Match {
                        expr: Identifier(
                            "y",
                        ),
                        cases: [
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "zero",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "one",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Wildcard,
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "other",
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
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
                                "result2",
                            ),
                            Literal(
                                String(
                                    "other",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "z",
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
                    "result3",
                ),
                value: Some(
                    Match {
                        expr: Identifier(
                            "z",
                        ),
                        cases: [
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "zero",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "one",
                                            ),
                                        ),
                                    ),
                                },
                            },
                            MatchCase {
                                pattern: Wildcard,
                                guard: None,
                                body: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "other",
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
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
                                "result3",
                            ),
                            Literal(
                                String(
                                    "zero",
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