# Program
Status: 🟢
Assertions: 2

```rustleaf
class File() {
    var opened;
    var closed;

    fn op_open() {
        self.opened += 1;
    }

    fn op_close() {
        self.closed += 1;
    }

    fn read() {}
}

var f = File();
f.opened = 0;
f.closed = 0;
with f {
    f.read();
}
assert(f.opened == 1, "opened should be 1");
assert(f.closed == 1, "closed should be 1");
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
        0: Token(Class),
        1: Token(Ident, "File"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Var),
        6: Token(Ident, "opened"),
        7: Token(Semicolon),
        8: Token(Var),
        9: Token(Ident, "closed"),
        10: Token(Semicolon),
        11: Token(Fn),
        12: Token(Ident, "op_open"),
        13: Token(LeftParen),
        14: Token(RightParen),
        15: Token(LeftBrace),
        16: Token(Ident, "self"),
        17: Token(Dot),
        18: Token(Ident, "opened"),
        19: Token(PlusEqual),
        20: Token(Int, "1"),
        21: Token(Semicolon),
        22: Token(RightBrace),
        23: Token(Fn),
        24: Token(Ident, "op_close"),
        25: Token(LeftParen),
        26: Token(RightParen),
        27: Token(LeftBrace),
        28: Token(Ident, "self"),
        29: Token(Dot),
        30: Token(Ident, "closed"),
        31: Token(PlusEqual),
        32: Token(Int, "1"),
        33: Token(Semicolon),
        34: Token(RightBrace),
        35: Token(Fn),
        36: Token(Ident, "read"),
        37: Token(LeftParen),
        38: Token(RightParen),
        39: Token(LeftBrace),
        40: Token(RightBrace),
        41: Token(RightBrace),
        42: Token(Var),
        43: Token(Ident, "f"),
        44: Token(Equal),
        45: Token(Ident, "File"),
        46: Token(LeftParen),
        47: Token(RightParen),
        48: Token(Semicolon),
        49: Token(Ident, "f"),
        50: Token(Dot),
        51: Token(Ident, "opened"),
        52: Token(Equal),
        53: Token(Int, "0"),
        54: Token(Semicolon),
        55: Token(Ident, "f"),
        56: Token(Dot),
        57: Token(Ident, "closed"),
        58: Token(Equal),
        59: Token(Int, "0"),
        60: Token(Semicolon),
        61: Token(With),
        62: Token(Ident, "f"),
        63: Token(LeftBrace),
        64: Token(Ident, "f"),
        65: Token(Dot),
        66: Token(Ident, "read"),
        67: Token(LeftParen),
        68: Token(RightParen),
        69: Token(Semicolon),
        70: Token(RightBrace),
        71: Token(Ident, "assert"),
        72: Token(LeftParen),
        73: Token(Ident, "f"),
        74: Token(Dot),
        75: Token(Ident, "opened"),
        76: Token(EqualEqual),
        77: Token(Int, "1"),
        78: Token(Comma),
        79: Token(String, "opened should be 1"),
        80: Token(RightParen),
        81: Token(Semicolon),
        82: Token(Ident, "assert"),
        83: Token(LeftParen),
        84: Token(Ident, "f"),
        85: Token(Dot),
        86: Token(Ident, "closed"),
        87: Token(EqualEqual),
        88: Token(Int, "1"),
        89: Token(Comma),
        90: Token(String, "closed should be 1"),
        91: Token(RightParen),
        92: Token(Semicolon),
        93: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            ClassDecl {
                name: "File",
                members: [
                    ClassMember {
                        name: "opened",
                        kind: Field(
                            None,
                        ),
                    },
                    ClassMember {
                        name: "closed",
                        kind: Field(
                            None,
                        ),
                    },
                    ClassMember {
                        name: "op_open",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [
                                    Assignment {
                                        target: GetAttr(
                                            Identifier(
                                                "self",
                                            ),
                                            "opened",
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
                        },
                    },
                    ClassMember {
                        name: "op_close",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [
                                    Assignment {
                                        target: GetAttr(
                                            Identifier(
                                                "self",
                                            ),
                                            "closed",
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
                        },
                    },
                    ClassMember {
                        name: "read",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [],
                                final_expr: None,
                            },
                        },
                    },
                ],
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "f",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "File",
                        ),
                        [],
                    ),
                ),
            },
            Assignment {
                target: GetAttr(
                    Identifier(
                        "f",
                    ),
                    "opened",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        0,
                    ),
                ),
            },
            Assignment {
                target: GetAttr(
                    Identifier(
                        "f",
                    ),
                    "closed",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        0,
                    ),
                ),
            },
            Expression(
                With {
                    resources: [
                        WithResource {
                            name: "",
                            value: Identifier(
                                "f",
                            ),
                        },
                    ],
                    body: Block {
                        statements: [
                            Expression(
                                MethodCall(
                                    Identifier(
                                        "f",
                                    ),
                                    "read",
                                    [],
                                ),
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
                            GetAttr(
                                Identifier(
                                    "f",
                                ),
                                "opened",
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "opened should be 1",
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
                                    "f",
                                ),
                                "closed",
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "closed should be 1",
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