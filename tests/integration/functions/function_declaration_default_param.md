# Program
Status: 🟢
Assertions: 7

```rustleaf
fn greet(name = "world") { name }
fn add(x, y = 10) { x + y }
fn multiply(a = 2, b = 3) { a * b }

var greeting1 = greet();
var greeting2 = greet("Alice");
var sum1 = add(5);
var sum2 = add(5, 15);
var product1 = multiply();
var product2 = multiply(4);
var product3 = multiply(4, 7);

assert(greeting1 == "world");
assert(greeting2 == "Alice");
assert(sum1 == 15);
assert(sum2 == 20);
assert(product1 == 6);
assert(product2 == 12);
assert(product3 == 28);
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
        1: Token(Ident, "greet"),
        2: Token(LeftParen),
        3: Token(Ident, "name"),
        4: Token(Equal),
        5: Token(String, "world"),
        6: Token(RightParen),
        7: Token(LeftBrace),
        8: Token(Ident, "name"),
        9: Token(RightBrace),
        10: Token(Fn),
        11: Token(Ident, "add"),
        12: Token(LeftParen),
        13: Token(Ident, "x"),
        14: Token(Comma),
        15: Token(Ident, "y"),
        16: Token(Equal),
        17: Token(Int, "10"),
        18: Token(RightParen),
        19: Token(LeftBrace),
        20: Token(Ident, "x"),
        21: Token(Plus),
        22: Token(Ident, "y"),
        23: Token(RightBrace),
        24: Token(Fn),
        25: Token(Ident, "multiply"),
        26: Token(LeftParen),
        27: Token(Ident, "a"),
        28: Token(Equal),
        29: Token(Int, "2"),
        30: Token(Comma),
        31: Token(Ident, "b"),
        32: Token(Equal),
        33: Token(Int, "3"),
        34: Token(RightParen),
        35: Token(LeftBrace),
        36: Token(Ident, "a"),
        37: Token(Star),
        38: Token(Ident, "b"),
        39: Token(RightBrace),
        40: Token(Var),
        41: Token(Ident, "greeting1"),
        42: Token(Equal),
        43: Token(Ident, "greet"),
        44: Token(LeftParen),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Var),
        48: Token(Ident, "greeting2"),
        49: Token(Equal),
        50: Token(Ident, "greet"),
        51: Token(LeftParen),
        52: Token(String, "Alice"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Var),
        56: Token(Ident, "sum1"),
        57: Token(Equal),
        58: Token(Ident, "add"),
        59: Token(LeftParen),
        60: Token(Int, "5"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Var),
        64: Token(Ident, "sum2"),
        65: Token(Equal),
        66: Token(Ident, "add"),
        67: Token(LeftParen),
        68: Token(Int, "5"),
        69: Token(Comma),
        70: Token(Int, "15"),
        71: Token(RightParen),
        72: Token(Semicolon),
        73: Token(Var),
        74: Token(Ident, "product1"),
        75: Token(Equal),
        76: Token(Ident, "multiply"),
        77: Token(LeftParen),
        78: Token(RightParen),
        79: Token(Semicolon),
        80: Token(Var),
        81: Token(Ident, "product2"),
        82: Token(Equal),
        83: Token(Ident, "multiply"),
        84: Token(LeftParen),
        85: Token(Int, "4"),
        86: Token(RightParen),
        87: Token(Semicolon),
        88: Token(Var),
        89: Token(Ident, "product3"),
        90: Token(Equal),
        91: Token(Ident, "multiply"),
        92: Token(LeftParen),
        93: Token(Int, "4"),
        94: Token(Comma),
        95: Token(Int, "7"),
        96: Token(RightParen),
        97: Token(Semicolon),
        98: Token(Ident, "assert"),
        99: Token(LeftParen),
        100: Token(Ident, "greeting1"),
        101: Token(EqualEqual),
        102: Token(String, "world"),
        103: Token(RightParen),
        104: Token(Semicolon),
        105: Token(Ident, "assert"),
        106: Token(LeftParen),
        107: Token(Ident, "greeting2"),
        108: Token(EqualEqual),
        109: Token(String, "Alice"),
        110: Token(RightParen),
        111: Token(Semicolon),
        112: Token(Ident, "assert"),
        113: Token(LeftParen),
        114: Token(Ident, "sum1"),
        115: Token(EqualEqual),
        116: Token(Int, "15"),
        117: Token(RightParen),
        118: Token(Semicolon),
        119: Token(Ident, "assert"),
        120: Token(LeftParen),
        121: Token(Ident, "sum2"),
        122: Token(EqualEqual),
        123: Token(Int, "20"),
        124: Token(RightParen),
        125: Token(Semicolon),
        126: Token(Ident, "assert"),
        127: Token(LeftParen),
        128: Token(Ident, "product1"),
        129: Token(EqualEqual),
        130: Token(Int, "6"),
        131: Token(RightParen),
        132: Token(Semicolon),
        133: Token(Ident, "assert"),
        134: Token(LeftParen),
        135: Token(Ident, "product2"),
        136: Token(EqualEqual),
        137: Token(Int, "12"),
        138: Token(RightParen),
        139: Token(Semicolon),
        140: Token(Ident, "assert"),
        141: Token(LeftParen),
        142: Token(Ident, "product3"),
        143: Token(EqualEqual),
        144: Token(Int, "28"),
        145: Token(RightParen),
        146: Token(Semicolon),
        147: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: Some(
                            String(
                                "world",
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Identifier(
                            "name",
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: Some(
                            Int(
                                10,
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "multiply",
                params: [
                    Parameter {
                        name: "a",
                        default: Some(
                            Int(
                                2,
                            ),
                        ),
                        kind: Regular,
                    },
                    Parameter {
                        name: "b",
                        default: Some(
                            Int(
                                3,
                            ),
                        ),
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Mul(
                            Identifier(
                                "a",
                            ),
                            Identifier(
                                "b",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "greeting1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "greeting2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [
                            Literal(
                                String(
                                    "Alice",
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "sum1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
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
                    "sum2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "add",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product3",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "multiply",
                        ),
                        [
                            Literal(
                                Int(
                                    4,
                                ),
                            ),
                            Literal(
                                Int(
                                    7,
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
                                "greeting1",
                            ),
                            Literal(
                                String(
                                    "world",
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
                                "greeting2",
                            ),
                            Literal(
                                String(
                                    "Alice",
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
                                "sum1",
                            ),
                            Literal(
                                Int(
                                    15,
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
                                "sum2",
                            ),
                            Literal(
                                Int(
                                    20,
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
                                "product1",
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
                                "product2",
                            ),
                            Literal(
                                Int(
                                    12,
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
                                "product3",
                            ),
                            Literal(
                                Int(
                                    28,
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