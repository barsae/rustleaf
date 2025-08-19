# Program
Status: 🟢
Assertions: 1

```rustleaf
class Greeter {
    var name;

    fn greet() {
        "Hello, ${self.name}"
    }
}

var greeter = Greeter();
greeter.name = "Eric";
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
        5: Token(Semicolon),
        6: Token(Fn),
        7: Token(Ident, "greet"),
        8: Token(LeftParen),
        9: Token(RightParen),
        10: Token(LeftBrace),
        11: Token(StringPart, "Hello, "),
        12: Token(InterpolationStart),
        13: Token(Ident, "self"),
        14: Token(Dot),
        15: Token(Ident, "name"),
        16: Token(InterpolationEnd),
        17: Token(RightBrace),
        18: Token(RightBrace),
        19: Token(Var),
        20: Token(Ident, "greeter"),
        21: Token(Equal),
        22: Token(Ident, "Greeter"),
        23: Token(LeftParen),
        24: Token(RightParen),
        25: Token(Semicolon),
        26: Token(Ident, "greeter"),
        27: Token(Dot),
        28: Token(Ident, "name"),
        29: Token(Equal),
        30: Token(String, "Eric"),
        31: Token(Semicolon),
        32: Token(Var),
        33: Token(Ident, "msg"),
        34: Token(Equal),
        35: Token(Ident, "greeter"),
        36: Token(Dot),
        37: Token(Ident, "greet"),
        38: Token(LeftParen),
        39: Token(RightParen),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Ident, "msg"),
        44: Token(EqualEqual),
        45: Token(String, "Hello, Eric"),
        46: Token(RightParen),
        47: Token(Semicolon),
        48: Token(Eof)
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
                            None,
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
            Assignment {
                target: GetAttr(
                    Identifier(
                        "greeter",
                    ),
                    "name",
                ),
                op: Assign,
                value: Literal(
                    String(
                        "Eric",
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