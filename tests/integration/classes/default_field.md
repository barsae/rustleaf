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
        Token(Class),
        Token(Ident, "Greeter"),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "Eric"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(StringPart, "Hello, "),
        Token(InterpolationStart),
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(RightBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "greeter"),
        Token(Equal),
        Token(Ident, "Greeter"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "msg"),
        Token(Equal),
        Token(Ident, "greeter"),
        Token(Dot),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "msg"),
        Token(EqualEqual),
        Token(String, "Hello, Eric"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
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
    Program(
        [
            ClassDecl(
                ClassDeclData {
                    name: "Greeter",
                    field_names: [
                        "name",
                    ],
                    field_defaults: [
                        Some(
                            Literal(
                                String(
                                    "Eric",
                                ),
                            ),
                        ),
                    ],
                    methods: [
                        ClassMethod {
                            name: "greet",
                            params: [
                                "self",
                            ],
                            body: Block(
                                [],
                                Some(
                                    Call(
                                        GetAttr(
                                            Literal(
                                                String(
                                                    "Hello, ",
                                                ),
                                            ),
                                            "op_add",
                                        ),
                                        [
                                            Call(
                                                Variable(
                                                    "str",
                                                ),
                                                [
                                                    GetAttr(
                                                        Variable(
                                                            "self",
                                                        ),
                                                        "name",
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                            is_static: false,
                        },
                    ],
                },
            ),
            Declare(
                "greeter",
                Some(
                    Call(
                        Variable(
                            "Greeter",
                        ),
                        [],
                    ),
                ),
            ),
            Declare(
                "msg",
                Some(
                    Call(
                        GetAttr(
                            Variable(
                                "greeter",
                            ),
                            "greet",
                        ),
                        [],
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "msg",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "Hello, Eric",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ),
)
```