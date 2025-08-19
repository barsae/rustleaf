# Program
Status: 🟢
Assertions: 3

```rustleaf
// Dict access with different key types
var my_dict = {"a": 1, "b": 2};
assert(my_dict["a"] == 1);

// Mixed key types
var mixed = {1: "one", "two": 2, true: "yes"};
assert(mixed[1] == "one");
assert(mixed["two"] == 2);
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
        1: Token(Ident, "my_dict"),
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(String, "a"),
        5: Token(Colon),
        6: Token(Int, "1"),
        7: Token(Comma),
        8: Token(String, "b"),
        9: Token(Colon),
        10: Token(Int, "2"),
        11: Token(RightBrace),
        12: Token(Semicolon),
        13: Token(Ident, "assert"),
        14: Token(LeftParen),
        15: Token(Ident, "my_dict"),
        16: Token(LeftBracket),
        17: Token(String, "a"),
        18: Token(RightBracket),
        19: Token(EqualEqual),
        20: Token(Int, "1"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Var),
        24: Token(Ident, "mixed"),
        25: Token(Equal),
        26: Token(LeftBrace),
        27: Token(Int, "1"),
        28: Token(Colon),
        29: Token(String, "one"),
        30: Token(Comma),
        31: Token(String, "two"),
        32: Token(Colon),
        33: Token(Int, "2"),
        34: Token(Comma),
        35: Token(True),
        36: Token(Colon),
        37: Token(String, "yes"),
        38: Token(RightBrace),
        39: Token(Semicolon),
        40: Token(Ident, "assert"),
        41: Token(LeftParen),
        42: Token(Ident, "mixed"),
        43: Token(LeftBracket),
        44: Token(Int, "1"),
        45: Token(RightBracket),
        46: Token(EqualEqual),
        47: Token(String, "one"),
        48: Token(RightParen),
        49: Token(Semicolon),
        50: Token(Ident, "assert"),
        51: Token(LeftParen),
        52: Token(Ident, "mixed"),
        53: Token(LeftBracket),
        54: Token(String, "two"),
        55: Token(RightBracket),
        56: Token(EqualEqual),
        57: Token(Int, "2"),
        58: Token(RightParen),
        59: Token(Semicolon),
        60: Token(Eof)
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
                            GetItem(
                                Identifier(
                                    "my_dict",
                                ),
                                Literal(
                                    String(
                                        "a",
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
            VarDecl {
                pattern: Variable(
                    "mixed",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "one",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "two",
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
                                    Bool(
                                        true,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "yes",
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
                        Eq(
                            GetItem(
                                Identifier(
                                    "mixed",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "mixed",
                                ),
                                Literal(
                                    String(
                                        "two",
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
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