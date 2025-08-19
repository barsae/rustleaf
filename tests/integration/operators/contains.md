# Program
Status: 🟢
Assertions: 9

```rustleaf
// Test 'in' operator (op_contains)

// String contains
assert("hello" in "hello world");
assert(not ("xyz" in "hello world"));

// List contains
var my_list = [1, 2, 3, "hello"];
assert(2 in my_list);
assert("hello" in my_list);
assert(not (99 in my_list));

// Dict contains (check keys)
var my_dict = {"a": 1, "b": 2, 3: "three"};
assert("a" in my_dict);
assert("b" in my_dict);
assert(3 in my_dict);
assert(not ("z" in my_dict));
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(String, "hello"),
        3: Token(In),
        4: Token(String, "hello world"),
        5: Token(RightParen),
        6: Token(Semicolon),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Not),
        10: Token(LeftParen),
        11: Token(String, "xyz"),
        12: Token(In),
        13: Token(String, "hello world"),
        14: Token(RightParen),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Var),
        18: Token(Ident, "my_list"),
        19: Token(Equal),
        20: Token(LeftBracket),
        21: Token(Int, "1"),
        22: Token(Comma),
        23: Token(Int, "2"),
        24: Token(Comma),
        25: Token(Int, "3"),
        26: Token(Comma),
        27: Token(String, "hello"),
        28: Token(RightBracket),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Int, "2"),
        33: Token(In),
        34: Token(Ident, "my_list"),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Ident, "assert"),
        38: Token(LeftParen),
        39: Token(String, "hello"),
        40: Token(In),
        41: Token(Ident, "my_list"),
        42: Token(RightParen),
        43: Token(Semicolon),
        44: Token(Ident, "assert"),
        45: Token(LeftParen),
        46: Token(Not),
        47: Token(LeftParen),
        48: Token(Int, "99"),
        49: Token(In),
        50: Token(Ident, "my_list"),
        51: Token(RightParen),
        52: Token(RightParen),
        53: Token(Semicolon),
        54: Token(Var),
        55: Token(Ident, "my_dict"),
        56: Token(Equal),
        57: Token(LeftBrace),
        58: Token(String, "a"),
        59: Token(Colon),
        60: Token(Int, "1"),
        61: Token(Comma),
        62: Token(String, "b"),
        63: Token(Colon),
        64: Token(Int, "2"),
        65: Token(Comma),
        66: Token(Int, "3"),
        67: Token(Colon),
        68: Token(String, "three"),
        69: Token(RightBrace),
        70: Token(Semicolon),
        71: Token(Ident, "assert"),
        72: Token(LeftParen),
        73: Token(String, "a"),
        74: Token(In),
        75: Token(Ident, "my_dict"),
        76: Token(RightParen),
        77: Token(Semicolon),
        78: Token(Ident, "assert"),
        79: Token(LeftParen),
        80: Token(String, "b"),
        81: Token(In),
        82: Token(Ident, "my_dict"),
        83: Token(RightParen),
        84: Token(Semicolon),
        85: Token(Ident, "assert"),
        86: Token(LeftParen),
        87: Token(Int, "3"),
        88: Token(In),
        89: Token(Ident, "my_dict"),
        90: Token(RightParen),
        91: Token(Semicolon),
        92: Token(Ident, "assert"),
        93: Token(LeftParen),
        94: Token(Not),
        95: Token(LeftParen),
        96: Token(String, "z"),
        97: Token(In),
        98: Token(Ident, "my_dict"),
        99: Token(RightParen),
        100: Token(RightParen),
        101: Token(Semicolon),
        102: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        In(
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello world",
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
                                    String(
                                        "xyz",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "hello world",
                                    ),
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "my_list",
                ),
                value: Some(
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
                                String(
                                    "hello",
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
                        In(
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                                String(
                                    "hello",
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                                        99,
                                    ),
                                ),
                                Identifier(
                                    "my_list",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "my_dict",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "b",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "three",
                                    ),
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
                        In(
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                                String(
                                    "b",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                                    3,
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                                    String(
                                        "z",
                                    ),
                                ),
                                Identifier(
                                    "my_dict",
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