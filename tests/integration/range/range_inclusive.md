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
    RustValue(<unknown>),
)
```