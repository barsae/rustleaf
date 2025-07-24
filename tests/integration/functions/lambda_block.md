# Program
Status: 🟢
Assertions: 0

```rustleaf
// #[fail_quietly]
|x| {
    print(x);
    x + 1
};
```

# Output
None

# Result
```rust
Ok(
    RustValue(
        RustValueRef(
            RefCell {
                value: RustLeafFunction {
                    params: [
                        "x",
                    ],
                    body: Block(
                        [
                            Call(
                                Variable(
                                    "print",
                                ),
                                [
                                    Variable(
                                        "x",
                                    ),
                                ],
                            ),
                        ],
                        Some(
                            Call(
                                GetAttr(
                                    Variable(
                                        "x",
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ],
                            ),
                        ),
                    ),
                    closure_env: ScopeRef(
                        RefCell {
                            value: Scope {
                                vars: {},
                                parent: Some(
                                    ScopeRef(
                                        RefCell {
                                            value: Scope {
                                                vars: {
                                                    "Int": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Int",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "str": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "str",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Bool": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Bool",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "assert": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "assert",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Null": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Null",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "raise": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "raise",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Unit": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Unit",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Float": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Float",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "List": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "List",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "String": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "String",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Dict": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Dict",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Range": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Range",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "print": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "print",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Function": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Function",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "is_unit": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "is_unit",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                                parent: None,
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                },
            },
        ),
    ),
)
```

# Lex
```rust
Ok(
    [
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBrace),
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
            Expression(
                Lambda {
                    params: [
                        "x",
                    ],
                    body: Block(
                        Block {
                            statements: [
                                Expression(
                                    FunctionCall(
                                        Identifier(
                                            "print",
                                        ),
                                        [
                                            Identifier(
                                                "x",
                                            ),
                                        ],
                                    ),
                                ),
                            ],
                            final_expr: Some(
                                Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [],
        Some(
            Lambda(
                [
                    "x",
                ],
                Block(
                    [
                        Call(
                            Variable(
                                "print",
                            ),
                            [
                                Variable(
                                    "x",
                                ),
                            ],
                        ),
                    ],
                    Some(
                        Call(
                            GetAttr(
                                Variable(
                                    "x",
                                ),
                                "op_add",
                            ),
                            [
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ],
                        ),
                    ),
                ),
            ),
        ),
    ),
)
```