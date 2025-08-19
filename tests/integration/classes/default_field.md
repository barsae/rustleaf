# Program
Status: 🟢
Assertions: 1

```rustleaf
class Greeter {
    var name = "Eric";

    fn greet() {
        "Hello, ${self.name}"
    }
}

var greeter = Greeter();
var msg = greeter.greet();
assert(msg == "Hello, Eric");
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
        1: Token(Ident, "Greeter"),
        2: Token(LeftBrace),
        3: Token(Var),
        4: Token(Ident, "name"),
        5: Token(Equal),
        6: Token(String, "Eric"),
        7: Token(Semicolon),
        8: Token(Fn),
        9: Token(Ident, "greet"),
        10: Token(LeftParen),
        11: Token(RightParen),
        12: Token(LeftBrace),
        13: Token(StringPart, "Hello, "),
        14: Token(InterpolationStart),
        15: Token(Ident, "self"),
        16: Token(Dot),
        17: Token(Ident, "name"),
        18: Token(InterpolationEnd),
        19: Token(RightBrace),
        20: Token(RightBrace),
        21: Token(Var),
        22: Token(Ident, "greeter"),
        23: Token(Equal),
        24: Token(Ident, "Greeter"),
        25: Token(LeftParen),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Var),
        29: Token(Ident, "msg"),
        30: Token(Equal),
        31: Token(Ident, "greeter"),
        32: Token(Dot),
        33: Token(Ident, "greet"),
        34: Token(LeftParen),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Ident, "assert"),
        38: Token(LeftParen),
        39: Token(Ident, "msg"),
        40: Token(EqualEqual),
        41: Token(String, "Hello, Eric"),
        42: Token(RightParen),
        43: Token(Semicolon),
        44: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            ClassDecl {
                name: "Greeter",
                members: [
                    ClassMember {
                        name: "name",
                        kind: Field(
                            Some(
                                Literal(
                                    String(
                                        "Eric",
                                    ),
                                ),
                            ),
                        ),
                    },
                    ClassMember {
                        name: "greet",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [],
                                final_expr: Some(
                                    InterpolatedString(
                                        [
                                            Text(
                                                "Hello, ",
                                            ),
                                            Expression(
                                                GetAttr(
                                                    Identifier(
                                                        "self",
                                                    ),
                                                    "name",
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            },
                        },
                    },
                ],
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "greeter",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "Greeter",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "msg",
                ),
                value: Some(
                    MethodCall(
                        Identifier(
                            "greeter",
                        ),
                        "greet",
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
                                "msg",
                            ),
                            Literal(
                                String(
                                    "Hello, Eric",
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