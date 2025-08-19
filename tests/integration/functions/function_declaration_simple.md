# Program
Status: 🟢
Assertions: 4

```rustleaf
fn add(x, y) { x + y }
fn multiply(a, b) { a * b }

var sum = add(5, 3);
var product = multiply(4, 6);

assert(sum == 8);
assert(product == 24);
assert(add(10, -2) == 8);
assert(multiply(0, 100) == 0);
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
        1: Token(Ident, "add"),
        2: Token(LeftParen),
        3: Token(Ident, "x"),
        4: Token(Comma),
        5: Token(Ident, "y"),
        6: Token(RightParen),
        7: Token(LeftBrace),
        8: Token(Ident, "x"),
        9: Token(Plus),
        10: Token(Ident, "y"),
        11: Token(RightBrace),
        12: Token(Fn),
        13: Token(Ident, "multiply"),
        14: Token(LeftParen),
        15: Token(Ident, "a"),
        16: Token(Comma),
        17: Token(Ident, "b"),
        18: Token(RightParen),
        19: Token(LeftBrace),
        20: Token(Ident, "a"),
        21: Token(Star),
        22: Token(Ident, "b"),
        23: Token(RightBrace),
        24: Token(Var),
        25: Token(Ident, "sum"),
        26: Token(Equal),
        27: Token(Ident, "add"),
        28: Token(LeftParen),
        29: Token(Int, "5"),
        30: Token(Comma),
        31: Token(Int, "3"),
        32: Token(RightParen),
        33: Token(Semicolon),
        34: Token(Var),
        35: Token(Ident, "product"),
        36: Token(Equal),
        37: Token(Ident, "multiply"),
        38: Token(LeftParen),
        39: Token(Int, "4"),
        40: Token(Comma),
        41: Token(Int, "6"),
        42: Token(RightParen),
        43: Token(Semicolon),
        44: Token(Ident, "assert"),
        45: Token(LeftParen),
        46: Token(Ident, "sum"),
        47: Token(EqualEqual),
        48: Token(Int, "8"),
        49: Token(RightParen),
        50: Token(Semicolon),
        51: Token(Ident, "assert"),
        52: Token(LeftParen),
        53: Token(Ident, "product"),
        54: Token(EqualEqual),
        55: Token(Int, "24"),
        56: Token(RightParen),
        57: Token(Semicolon),
        58: Token(Ident, "assert"),
        59: Token(LeftParen),
        60: Token(Ident, "add"),
        61: Token(LeftParen),
        62: Token(Int, "10"),
        63: Token(Comma),
        64: Token(Minus),
        65: Token(Int, "2"),
        66: Token(RightParen),
        67: Token(EqualEqual),
        68: Token(Int, "8"),
        69: Token(RightParen),
        70: Token(Semicolon),
        71: Token(Ident, "assert"),
        72: Token(LeftParen),
        73: Token(Ident, "multiply"),
        74: Token(LeftParen),
        75: Token(Int, "0"),
        76: Token(Comma),
        77: Token(Int, "100"),
        78: Token(RightParen),
        79: Token(EqualEqual),
        80: Token(Int, "0"),
        81: Token(RightParen),
        82: Token(Semicolon),
        83: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
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
                        default: None,
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
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "b",
                        default: None,
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
                    "sum",
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
                                    3,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "product",
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
                                    6,
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
                                "sum",
                            ),
                            Literal(
                                Int(
                                    8,
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
                                "product",
                            ),
                            Literal(
                                Int(
                                    24,
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
                            FunctionCall(
                                Identifier(
                                    "add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    Neg(
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    8,
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
                            FunctionCall(
                                Identifier(
                                    "multiply",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            100,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    0,
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