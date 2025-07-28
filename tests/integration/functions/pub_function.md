# Program
Status: 🟢
Assertions: 2

```rustleaf
pub fn greet(name) {
    return "Hello, " + name;
}

var result = greet("World");
assert(result == "Hello, World");
assert(greet("Alice") == "Hello, Alice");
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
        Token(Pub),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Return),
        Token(String, "Hello, "),
        Token(Plus),
        Token(Ident, "name"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(String, "World"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "Hello, World"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(String, "Alice"),
        Token(RightParen),
        Token(EqualEqual),
        Token(String, "Hello, Alice"),
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
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        Return(
                            Some(
                                Add(
                                    Literal(
                                        String(
                                            "Hello, ",
                                        ),
                                    ),
                                    Identifier(
                                        "name",
                                    ),
                                ),
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: true,
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [
                            Literal(
                                String(
                                    "World",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "Hello, World",
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
                            Literal(
                                String(
                                    "Hello, Alice",
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
            Function(
                FunctionData {
                    name: "greet",
                    params: [
                        (
                            "name",
                            None,
                            Regular,
                        ),
                    ],
                    body: Block(
                        [
                            Return(
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
                                            Variable(
                                                "name",
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        None,
                    ),
                },
            ),
            Declare(
                "result",
                Some(
                    Call(
                        Variable(
                            "greet",
                        ),
                        [
                            Literal(
                                String(
                                    "World",
                                ),
                            ),
                        ],
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
                                "result",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "Hello, World",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                Variable(
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
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "Hello, Alice",
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