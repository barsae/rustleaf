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
    RustValue(<unknown>),
)
```