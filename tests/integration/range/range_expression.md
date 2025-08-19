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
    RustValue(<unknown>),
)
```