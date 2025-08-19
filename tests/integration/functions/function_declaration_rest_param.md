# Program
Status: 🟢
Assertions: 10

```rustleaf
fn sum(*args) { args }
fn first_and_rest(first, *rest) { [first, rest] }

var empty = sum();
var single = sum(1);
var multiple = sum(1, 2, 3, 4, 5);

var mixed1 = first_and_rest(42);
var mixed2 = first_and_rest(10, 20, 30);

// Test that rest parameters collect into lists
assert(empty == []);
assert(single == [1]);
assert(multiple == [1, 2, 3, 4, 5]);

// Test mixed regular and rest parameters
assert(mixed1 == [42, []]);
assert(mixed2 == [10, [20, 30]]);

// Test individual access
assert(single[0] == 1);
assert(multiple[0] == 1);
assert(multiple[4] == 5);
assert(mixed1[0] == 42);
assert(mixed2[0] == 10);
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
        0: Token(Fn),
        1: Token(Ident, "sum"),
        2: Token(LeftParen),
        3: Token(Star),
        4: Token(Ident, "args"),
        5: Token(RightParen),
        6: Token(LeftBrace),
        7: Token(Ident, "args"),
        8: Token(RightBrace),
        9: Token(Fn),
        10: Token(Ident, "first_and_rest"),
        11: Token(LeftParen),
        12: Token(Ident, "first"),
        13: Token(Comma),
        14: Token(Star),
        15: Token(Ident, "rest"),
        16: Token(RightParen),
        17: Token(LeftBrace),
        18: Token(LeftBracket),
        19: Token(Ident, "first"),
        20: Token(Comma),
        21: Token(Ident, "rest"),
        22: Token(RightBracket),
        23: Token(RightBrace),
        24: Token(Var),
        25: Token(Ident, "empty"),
        26: Token(Equal),
        27: Token(Ident, "sum"),
        28: Token(LeftParen),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Var),
        32: Token(Ident, "single"),
        33: Token(Equal),
        34: Token(Ident, "sum"),
        35: Token(LeftParen),
        36: Token(Int, "1"),
        37: Token(RightParen),
        38: Token(Semicolon),
        39: Token(Var),
        40: Token(Ident, "multiple"),
        41: Token(Equal),
        42: Token(Ident, "sum"),
        43: Token(LeftParen),
        44: Token(Int, "1"),
        45: Token(Comma),
        46: Token(Int, "2"),
        47: Token(Comma),
        48: Token(Int, "3"),
        49: Token(Comma),
        50: Token(Int, "4"),
        51: Token(Comma),
        52: Token(Int, "5"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Var),
        56: Token(Ident, "mixed1"),
        57: Token(Equal),
        58: Token(Ident, "first_and_rest"),
        59: Token(LeftParen),
        60: Token(Int, "42"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Var),
        64: Token(Ident, "mixed2"),
        65: Token(Equal),
        66: Token(Ident, "first_and_rest"),
        67: Token(LeftParen),
        68: Token(Int, "10"),
        69: Token(Comma),
        70: Token(Int, "20"),
        71: Token(Comma),
        72: Token(Int, "30"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Ident, "assert"),
        76: Token(LeftParen),
        77: Token(Ident, "empty"),
        78: Token(EqualEqual),
        79: Token(LeftBracket),
        80: Token(RightBracket),
        81: Token(RightParen),
        82: Token(Semicolon),
        83: Token(Ident, "assert"),
        84: Token(LeftParen),
        85: Token(Ident, "single"),
        86: Token(EqualEqual),
        87: Token(LeftBracket),
        88: Token(Int, "1"),
        89: Token(RightBracket),
        90: Token(RightParen),
        91: Token(Semicolon),
        92: Token(Ident, "assert"),
        93: Token(LeftParen),
        94: Token(Ident, "multiple"),
        95: Token(EqualEqual),
        96: Token(LeftBracket),
        97: Token(Int, "1"),
        98: Token(Comma),
        99: Token(Int, "2"),
        100: Token(Comma),
        101: Token(Int, "3"),
        102: Token(Comma),
        103: Token(Int, "4"),
        104: Token(Comma),
        105: Token(Int, "5"),
        106: Token(RightBracket),
        107: Token(RightParen),
        108: Token(Semicolon),
        109: Token(Ident, "assert"),
        110: Token(LeftParen),
        111: Token(Ident, "mixed1"),
        112: Token(EqualEqual),
        113: Token(LeftBracket),
        114: Token(Int, "42"),
        115: Token(Comma),
        116: Token(LeftBracket),
        117: Token(RightBracket),
        118: Token(RightBracket),
        119: Token(RightParen),
        120: Token(Semicolon),
        121: Token(Ident, "assert"),
        122: Token(LeftParen),
        123: Token(Ident, "mixed2"),
        124: Token(EqualEqual),
        125: Token(LeftBracket),
        126: Token(Int, "10"),
        127: Token(Comma),
        128: Token(LeftBracket),
        129: Token(Int, "20"),
        130: Token(Comma),
        131: Token(Int, "30"),
        132: Token(RightBracket),
        133: Token(RightBracket),
        134: Token(RightParen),
        135: Token(Semicolon),
        136: Token(Ident, "assert"),
        137: Token(LeftParen),
        138: Token(Ident, "single"),
        139: Token(LeftBracket),
        140: Token(Int, "0"),
        141: Token(RightBracket),
        142: Token(EqualEqual),
        143: Token(Int, "1"),
        144: Token(RightParen),
        145: Token(Semicolon),
        146: Token(Ident, "assert"),
        147: Token(LeftParen),
        148: Token(Ident, "multiple"),
        149: Token(LeftBracket),
        150: Token(Int, "0"),
        151: Token(RightBracket),
        152: Token(EqualEqual),
        153: Token(Int, "1"),
        154: Token(RightParen),
        155: Token(Semicolon),
        156: Token(Ident, "assert"),
        157: Token(LeftParen),
        158: Token(Ident, "multiple"),
        159: Token(LeftBracket),
        160: Token(Int, "4"),
        161: Token(RightBracket),
        162: Token(EqualEqual),
        163: Token(Int, "5"),
        164: Token(RightParen),
        165: Token(Semicolon),
        166: Token(Ident, "assert"),
        167: Token(LeftParen),
        168: Token(Ident, "mixed1"),
        169: Token(LeftBracket),
        170: Token(Int, "0"),
        171: Token(RightBracket),
        172: Token(EqualEqual),
        173: Token(Int, "42"),
        174: Token(RightParen),
        175: Token(Semicolon),
        176: Token(Ident, "assert"),
        177: Token(LeftParen),
        178: Token(Ident, "mixed2"),
        179: Token(LeftBracket),
        180: Token(Int, "0"),
        181: Token(RightBracket),
        182: Token(EqualEqual),
        183: Token(Int, "10"),
        184: Token(RightParen),
        185: Token(Semicolon),
        186: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "sum",
                params: [
                    Parameter {
                        name: "args",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "args",
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "first_and_rest",
                params: [
                    Parameter {
                        name: "first",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "rest",
                        default: None,
                        kind: Rest,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        List(
                            [
                                Identifier(
                                    "first",
                                ),
                                Identifier(
                                    "rest",
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "empty",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "single",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "multiple",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "sum",
                        ),
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "mixed1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "mixed2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "first_and_rest",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                            Literal(
                                Int(
                                    20,
                                ),
                            ),
                            Literal(
                                Int(
                                    30,
                                ),
                            ),
                        ],
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
                            Identifier(
                                "empty",
                            ),
                            List(
                                [],
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
                                "single",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ],
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
                                "multiple",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
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
                                "mixed1",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            42,
                                        ),
                                    ),
                                    List(
                                        [],
                                    ),
                                ],
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
                                "mixed2",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    List(
                                        [
                                            Literal(
                                                Int(
                                                    20,
                                                ),
                                            ),
                                            Literal(
                                                Int(
                                                    30,
                                                ),
                                            ),
                                        ],
                                    ),
                                ],
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
                                    "single",
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
                                    "multiple",
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
                                    "multiple",
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "mixed1",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "mixed2",
                                ),
                                Literal(
                                    Int(
                                        0,
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