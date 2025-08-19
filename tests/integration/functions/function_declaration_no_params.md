# Program
Status: 🟢
Assertions: 4

```rustleaf
fn hello() { 42 }
fn get_pi() { 3.14 }
fn get_greeting() { "Hello, World!" }

var result1 = hello();
var result2 = get_pi();
var result3 = get_greeting();

assert(result1 == 42);
assert(result2 == 3.14);
assert(result3 == "Hello, World!");
assert(hello() == 42);
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
        1: Token(Ident, "hello"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Int, "42"),
        6: Token(RightBrace),
        7: Token(Fn),
        8: Token(Ident, "get_pi"),
        9: Token(LeftParen),
        10: Token(RightParen),
        11: Token(LeftBrace),
        12: Token(Float, "3.14"),
        13: Token(RightBrace),
        14: Token(Fn),
        15: Token(Ident, "get_greeting"),
        16: Token(LeftParen),
        17: Token(RightParen),
        18: Token(LeftBrace),
        19: Token(String, "Hello, World!"),
        20: Token(RightBrace),
        21: Token(Var),
        22: Token(Ident, "result1"),
        23: Token(Equal),
        24: Token(Ident, "hello"),
        25: Token(LeftParen),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Var),
        29: Token(Ident, "result2"),
        30: Token(Equal),
        31: Token(Ident, "get_pi"),
        32: Token(LeftParen),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Var),
        36: Token(Ident, "result3"),
        37: Token(Equal),
        38: Token(Ident, "get_greeting"),
        39: Token(LeftParen),
        40: Token(RightParen),
        41: Token(Semicolon),
        42: Token(Ident, "assert"),
        43: Token(LeftParen),
        44: Token(Ident, "result1"),
        45: Token(EqualEqual),
        46: Token(Int, "42"),
        47: Token(RightParen),
        48: Token(Semicolon),
        49: Token(Ident, "assert"),
        50: Token(LeftParen),
        51: Token(Ident, "result2"),
        52: Token(EqualEqual),
        53: Token(Float, "3.14"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Ident, "assert"),
        57: Token(LeftParen),
        58: Token(Ident, "result3"),
        59: Token(EqualEqual),
        60: Token(String, "Hello, World!"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Ident, "hello"),
        66: Token(LeftParen),
        67: Token(RightParen),
        68: Token(EqualEqual),
        69: Token(Int, "42"),
        70: Token(RightParen),
        71: Token(Semicolon),
        72: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "hello",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            Int(
                                42,
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "get_pi",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            Float(
                                3.14,
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "get_greeting",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            String(
                                "Hello, World!",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "result1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "hello",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "get_pi",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "get_greeting",
                        ),
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
                            Identifier(
                                "result1",
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
                            Identifier(
                                "result2",
                            ),
                            Literal(
                                Float(
                                    3.14,
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
                                "result3",
                            ),
                            Literal(
                                String(
                                    "Hello, World!",
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
                                    "hello",
                                ),
                                [],
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