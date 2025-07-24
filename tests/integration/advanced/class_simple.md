# Program
Status: 🟢
Assertions: 0

```rustleaf
class Person {
    var name;

    fn greet() {
        print("Hello");
    }
}
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
        Token(Ident, "Person"),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "name"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(String, "Hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
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
                name: "Person",
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
                                statements: [
                                    Expression(
                                        FunctionCall(
                                            Identifier(
                                                "print",
                                            ),
                                            [
                                                Literal(
                                                    String(
                                                        "Hello",
                                                    ),
                                                ),
                                            ],
                                        ),
                                    ),
                                ],
                                final_expr: None,
                            },
                        },
                    },
                ],
                is_pub: false,
            },
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [
            ClassDecl {
                name: "Person",
                field_names: [
                    "name",
                ],
                field_defaults: [
                    None,
                ],
                methods: [
                    ClassMethod {
                        name: "greet",
                        params: [],
                        body: Block(
                            [
                                Call(
                                    Variable(
                                        "print",
                                    ),
                                    [
                                        Literal(
                                            String(
                                                "Hello",
                                            ),
                                        ),
                                    ],
                                ),
                            ],
                            None,
                        ),
                        is_static: false,
                    },
                ],
            },
        ],
        None,
    ),
)
```