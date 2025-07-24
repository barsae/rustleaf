# Program
Status: 🟢
Assertions: 2

```rustleaf
var user = {"name": "Alice", "age": 30};
var {name, age: user_age} = user;
assert(name == "Alice");
assert(user_age == 30);
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
        Token(Var),
        Token(Ident, "user"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "name"),
        Token(Colon),
        Token(String, "Alice"),
        Token(Comma),
        Token(String, "age"),
        Token(Colon),
        Token(Int, "30"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(LeftBrace),
        Token(Ident, "name"),
        Token(Comma),
        Token(Ident, "age"),
        Token(Colon),
        Token(Ident, "user_age"),
        Token(RightBrace),
        Token(Equal),
        Token(Ident, "user"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "name"),
        Token(EqualEqual),
        Token(String, "Alice"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "user_age"),
        Token(EqualEqual),
        Token(Int, "30"),
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
            VarDecl {
                pattern: Variable(
                    "user",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "name",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "Alice",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "age",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        30,
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Dict(
                    [
                        DictPattern {
                            key: "name",
                            alias: None,
                        },
                        DictPattern {
                            key: "age",
                            alias: Some(
                                "user_age",
                            ),
                        },
                    ],
                ),
                value: Some(
                    Identifier(
                        "user",
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
                                "name",
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
                                "user_age",
                            ),
                            Literal(
                                Int(
                                    30,
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
    Block(
        [
            Declare(
                "user",
                Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "name",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "Alice",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "age",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        30,
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            DeclarePattern(
                Dict(
                    [
                        EvalDictPattern {
                            key: "name",
                            alias: None,
                        },
                        EvalDictPattern {
                            key: "age",
                            alias: Some(
                                "user_age",
                            ),
                        },
                    ],
                ),
                Variable(
                    "user",
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
                                "name",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "Alice",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "user_age",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    30,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```