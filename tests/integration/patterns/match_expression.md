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
None

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