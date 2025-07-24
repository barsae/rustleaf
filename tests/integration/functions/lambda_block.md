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
                                                    "Bool": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Bool",
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
                                                    "Function": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Function",
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
                                                    "is_unit": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "is_unit",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Int": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Int",
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
                                                    "Float": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Float",
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
                                                    "raise": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "raise",
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
                                                    "str": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "str",
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
                                                    "Unit": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Unit",
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